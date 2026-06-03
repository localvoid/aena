//! HTML-escaping utilities for text content and attribute values.
//!
//! These functions and writer adapters ensure that user-supplied strings are safely
//! embedded in HTML/SVG/XML output. Three escaping contexts are supported:
//!
//! | Context | Escaped characters | Function |
//! |---|---|---|
//! | Text content | `&`, `<`, `>` | [`write_escape_text`] / [`EscapeTextWriter`] |
//! | Attribute values | `&`, `"` | [`write_escape_arg`] / [`EscapeArgWriter`] |
//! | Script / style | `&`, `<`, `>`, `/` | [`write_escape_script`] |

use std::fmt::Write;

/// Write `s` into `out` with HTML text-content escaping.
///
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
///
/// # Errors
///
/// Returns [`std::fmt::Error`] if writing to `out` fails.
///
/// # Examples
///
/// ```rust
/// use aena::escape::write_escape_text;
/// let mut out = String::new();
/// write_escape_text("<script>alert('xss')</script>", &mut out).unwrap();
/// assert_eq!(out, "&lt;script&gt;alert('xss')&lt;/script&gt;");
/// ```
pub fn write_escape_text<W: Write>(s: &str, out: &mut W) -> std::fmt::Result {
    let bytes = s.as_bytes();
    let mut start = 0usize;

    for (i, &b) in bytes.iter().enumerate() {
        let replacement = match b {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            _ => continue,
        };
        if start < i {
            out.write_str(&s[start..i])?;
        }
        out.write_str(replacement)?;
        start = i + 1;
    }
    if start < s.len() {
        out.write_str(&s[start..])?;
    }
    Ok(())
}

/// Write `s` into `out` with HTML attribute-value escaping.
///
/// - `&` → `&amp;`
/// - `"` → `&quot;`
///
/// # Errors
///
/// Returns [`std::fmt::Error`] if writing to `out` fails.
///
/// # Examples
///
/// ```rust
/// use aena::escape::write_escape_arg;
/// let mut out = String::new();
/// write_escape_arg("a\"b&c", &mut out).unwrap();
/// assert_eq!(out, "a&quot;b&amp;c");
/// ```
pub fn write_escape_arg<W: Write>(s: &str, out: &mut W) -> std::fmt::Result {
    let bytes = s.as_bytes();
    let mut start = 0usize;

    for (i, &b) in bytes.iter().enumerate() {
        let replacement = match b {
            b'&' => "&amp;",
            b'"' => "&quot;",
            _ => continue,
        };
        if start < i {
            out.write_str(&s[start..i])?;
        }
        out.write_str(replacement)?;
        start = i + 1;
    }
    if start < s.len() {
        out.write_str(&s[start..])?;
    }
    Ok(())
}

/// Write `s` into `out` with HTML script/style escaping.
///
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `/` → `&#47;`
///
/// Prevents `</script>` or `</style>` breaking out of the element in addition
/// to basic text escaping.
///
/// # Errors
///
/// Returns [`std::fmt::Error`] if writing to `out` fails.
///
/// # Examples
///
/// ```rust
/// use aena::escape::write_escape_script;
/// let mut out = String::new();
/// write_escape_script("</script>", &mut out).unwrap();
/// assert_eq!(out, "&lt;&#47;script&gt;");
/// ```
pub fn write_escape_script<W: Write>(s: &str, out: &mut W) -> std::fmt::Result {
    let bytes = s.as_bytes();
    let mut start = 0usize;

    for (i, &b) in bytes.iter().enumerate() {
        let replacement = match b {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'/' => "&#47;",
            _ => continue,
        };
        if start < i {
            out.write_str(&s[start..i])?;
        }
        out.write_str(replacement)?;
        start = i + 1;
    }
    if start < s.len() {
        out.write_str(&s[start..])?;
    }
    Ok(())
}

/// Write `s` into `out` with comment-content escaping.
///
/// - `--` → `-&#45;` (prevents comment breakout in HTML and invalid `--` in XML)
///
/// # Errors
///
/// Returns [`std::fmt::Error`] if writing to `out` fails.
///
/// # Examples
///
/// ```rust
/// use aena::escape::write_escape_comment;
/// let mut out = String::new();
/// write_escape_comment("foo -- bar", &mut out).unwrap();
/// assert_eq!(out, "foo -&#45; bar");
/// ```
pub fn write_escape_comment<W: Write>(s: &str, out: &mut W) -> std::fmt::Result {
    let bytes = s.as_bytes();
    let mut last = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'-' && i + 1 < bytes.len() && bytes[i + 1] == b'-' {
            out.write_str(&s[last..=i])?;
            out.write_str("&#45;")?;
            last = i + 2;
            i += 2;
        } else {
            i += 1;
        }
    }
    out.write_str(&s[last..])
}

/// A [`Write`] adapter that escapes text content.
///
/// Delegates to [`write_escape_text`] for every `write_str` call.
/// Useful with [`std::fmt::Arguments`] via `write_fmt`.
///
/// # Examples
///
/// ```rust
/// use aena::escape::EscapeTextWriter;
/// use std::fmt::Write;
/// let mut out = String::new();
/// write!(EscapeTextWriter::new(&mut out), "hello {}", "<world>").unwrap();
/// assert_eq!(out, "hello &lt;world&gt;");
/// ```
pub struct EscapeTextWriter<'a, W: Write> {
    inner: &'a mut W,
}

impl<'a, W: Write> EscapeTextWriter<'a, W> {
    /// Creates a new writer adapter that escapes text content.
    #[inline]
    pub fn new(writer: &'a mut W) -> Self {
        Self { inner: writer }
    }
}

impl<W: Write> Write for EscapeTextWriter<'_, W> {
    #[inline]
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write_escape_text(s, self.inner)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        let s = match c {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            _ => {
                return self.inner.write_char(c);
            }
        };
        self.inner.write_str(s)
    }
}

/// A [`Write`] adapter that escapes attribute values.
///
/// Delegates to [`write_escape_arg`] for every `write_str` call.
/// Useful with [`std::fmt::Arguments`] via `write_fmt`.
///
/// # Examples
///
/// ```rust
/// use aena::escape::EscapeArgWriter;
/// use std::fmt::Write;
/// let mut out = String::new();
/// write!(EscapeArgWriter::new(&mut out), "{}", "\"quoted\"").unwrap();
/// assert_eq!(out, "&quot;quoted&quot;");
/// ```
pub struct EscapeArgWriter<'a, W: Write> {
    inner: &'a mut W,
}

impl<'a, W: Write> EscapeArgWriter<'a, W> {
    /// Creates a new writer adapter that escapes attribute values.
    #[inline]
    pub fn new(writer: &'a mut W) -> Self {
        Self { inner: writer }
    }
}

impl<W: Write> Write for EscapeArgWriter<'_, W> {
    #[inline]
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write_escape_arg(s, self.inner)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        let s = match c {
            '&' => "&amp;",
            '"' => "&quot;",
            _ => {
                return self.inner.write_char(c);
            }
        };
        self.inner.write_str(s)
    }
}
