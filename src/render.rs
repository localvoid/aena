//! Core rendering traits, types, and implementations.
//!
//! This module defines the trait hierarchy and element types that form the foundation
//! of `aena`'s rendering system.

use std::borrow::Cow;
use std::fmt::Write;

use bytes::BytesMut;

use crate::escape::{
    EscapeArgWriter, EscapeTextWriter, write_escape_arg, write_escape_comment, write_escape_text,
};

/// Content rendering context.
///
/// Wraps a [`BytesMut`] buffer and provides [`add`](CC::add) as the entry point
/// for rendering anything that implements [`Render`].
pub struct CC<'a>(pub &'a mut BytesMut);

impl CC<'_> {
    /// Render a value into the content buffer.
    ///
    /// Delegates to [`Render::render`].
    ///
    /// # Errors
    ///
    /// Returns [`std::fmt::Error`] if writing to the underlying buffer fails.
    #[inline]
    pub fn add<R: Render>(&mut self, r: R) -> std::fmt::Result {
        r.render(self)
    }
}

/// A value that can appear in an attribute-value position.
///
/// The [`ignore`](RenderAttributeValue::ignore) method allows conditional omission,
/// used by [`Option`] to skip `None` values.
///
/// # Implementors
///
/// - `&str`, `String`, `Cow<'_, str>` — string (escaped)
/// - [`SafeStr`] — raw string (no escaping)
/// - `Option<T>` — delegates to `T`; `ignore()` returns `true` when `None`
/// - Integer types
pub trait RenderAttributeValue {
    /// Write the attribute value into `w`.
    ///
    /// # Errors
    ///
    /// Returns [`std::fmt::Error`] if writing to `w` fails.
    fn render(self, w: &mut BytesMut) -> std::fmt::Result;

    /// Whether this value should be omitted from output.
    ///
    /// Returns `false` by default.
    #[inline]
    fn ignore(&self) -> bool {
        false
    }
}

impl RenderAttributeValue for SafeStr<'_> {
    #[inline]
    fn render(self, w: &mut BytesMut) -> std::fmt::Result {
        w.write_str(self.0)
    }
}

impl<T: RenderAttributeValue> RenderAttributeValue for Option<T> {
    #[inline]
    fn render(self, w: &mut BytesMut) -> std::fmt::Result {
        if let Some(v) = self {
            v.render(w)?;
        }
        Ok(())
    }

    #[inline]
    fn ignore(&self) -> bool {
        self.is_none()
    }
}

macro_rules! impl_render_value_for_str {
    ($($ty:ty),+) => {
        $(
            impl RenderAttributeValue for $ty {
                #[inline]
                fn render(self, w: &mut BytesMut) -> std::fmt::Result {
                    write_escape_arg(&self, w)
                }
            }
        )+
    };
}

impl_render_value_for_str!(&str, String, Cow<'_, str>);

macro_rules! impl_render_value_for_num {
    ($($ty:ty),+) => {
        $(
            impl RenderAttributeValue for $ty {
                #[inline]
                fn render(self, w: &mut BytesMut) -> std::fmt::Result {
                    write!(w, "{}", self)
                }
            }
        )+
    };
}

impl_render_value_for_num!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// Wrapper that renders a tuple of [`RenderAttributeValue`]s as a space-separated list.
pub(crate) struct SpaceSeparated<T>(pub(crate) T);

macro_rules! impl_space_separated {
    ($($var:ident: $ty:ident),+) => {
        impl<$($ty: RenderAttributeValue),+> SpaceSeparated<($($ty,)+)> {
            #[inline]
            fn write_into(
                self,
                w: &mut BytesMut,
            ) -> std::fmt::Result {
                let SpaceSeparated(($($var,)+)) = self;
                let mut _space_prefix = false;
                $(
                    if !$var.ignore() {
                        if _space_prefix {
                            w.write_char(' ')?;
                        } else {
                            _space_prefix = true;
                        }
                        $var.render(w)?;
                    }
                )+
                Ok(())
            }
        }

        impl<$($ty: RenderAttributeValue),+> RenderAttribute for ($($ty,)+) {
            #[inline]
            fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
                write!(w, " {key}=\"")?;
                SpaceSeparated(self).write_into(w)?;
                w.write_char('"')?;
                Ok(())
            }
        }

        impl<$($ty: RenderAttributeValue),+> RenderAttributes for ($($ty,)+)
        {
            #[inline]
            fn render(self, cx: &mut AC) -> std::fmt::Result {
                cx.writer.write_str(" class=\"")?;
                SpaceSeparated(self).write_into(cx.writer)?;
                cx.writer.write_char('"')?;
                Ok(())
            }
        }
    };
}

impl_space_separated!(a: A);
impl_space_separated!(a: A, b: B);
impl_space_separated!(a: A, b: B, c: C);
impl_space_separated!(a: A, b: B, c: C, d: D);
impl_space_separated!(a: A, b: B, c: C, d: D, e: E);
impl_space_separated!(a: A, b: B, c: C, d: D, e: E, f: F);
impl_space_separated!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
impl_space_separated!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);

/// A context for building inline CSS declarations.
///
/// Passed to the closure argument of [`style()`].
///
/// # Example
///
/// ```rust,ignore
/// a.set("style", style(|s: &mut SC| {
///     s.set("background", "#eee")?;
///     s.set("color", "#333")
/// }))
/// ```
pub struct SC<'a> {
    writer: &'a mut BytesMut,
    needs_semicolon: bool,
}

impl SC<'_> {
    /// Sets a CSS property `key` to `value`.
    ///
    /// Respects the [`ignore`](RenderAttributeValue::ignore) method — if the value
    /// is `None` (or otherwise marked ignored), the declaration is skipped.
    ///
    /// # Errors
    ///
    /// Returns [`std::fmt::Error`] if writing to the underlying buffer fails.
    #[inline]
    pub fn set(&mut self, key: &str, value: impl RenderAttributeValue) -> std::fmt::Result {
        if !value.ignore() {
            if self.needs_semicolon {
                self.writer.write_char(';')?;
            }
            write!(self.writer, "{key}:")?;
            value.render(self.writer)?;
            self.needs_semicolon = true;
        }
        Ok(())
    }
}

/// A style attribute value constructed via the [`style()`] function.
///
/// Implements [`RenderAttributeValue`] so the rendered style can be embedded
/// as a value, and [`RenderAttribute`] for direct use with [`AC::set`].
pub struct StyleClosure<F>(F)
where
    F: FnOnce(&mut SC) -> std::fmt::Result;

impl<F> RenderAttribute for StyleClosure<F>
where
    F: FnOnce(&mut SC) -> std::fmt::Result,
{
    #[inline]
    fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
        write!(w, " {key}=\"")?;
        self.0(&mut SC { writer: w, needs_semicolon: false })?;
        w.write_char('"')
    }
}

/// Builds an inline style attribute value using a builder closure.
///
/// The closure receives a [`SC`] context and should call [`SC::set`]
/// for each CSS property.
#[inline]
pub fn style<F>(f: F) -> StyleClosure<F>
where
    F: FnOnce(&mut SC) -> std::fmt::Result,
{
    StyleClosure(f)
}

/// A single HTML/SVG/XML attribute with a key and value.
///
/// Implementors render the full ` key="value"` (or ` key` for boolean attributes)
/// into the output buffer.
///
/// # Implementors
///
/// - `&str`, `String`, `Cow<'_, str>` — `key="escaped_value"`
/// - [`SafeStr`] — `key="raw_value"` (no escaping)
/// - `bool` — `key` when `true`, empty when `false`
/// - All numeric types — `key="value"`
/// - [`DisplayAttribute<T>`] — adapts any [`Display`](std::fmt::Display) type
/// - [`StyleClosure`] — inline style attribute via [`style()`](crate::style)
/// - Tuples of [`RenderAttributeValue`] — space-separated `class="..."` (via [`SpaceSeparated`])
/// - `Option<T>` — delegates to `T`; omitted when `None`
/// - [`std::fmt::Arguments`] — rendered with attribute-value escaping
pub trait RenderAttribute {
    /// Render the attribute (including the leading space and key) into `w`.
    ///
    /// # Errors
    ///
    /// Returns [`std::fmt::Error`] if writing to `w` fails.
    fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result;
}

impl RenderAttribute for SafeStr<'_> {
    fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
        write!(w, " {key}=\"{}\"", &self.0)
    }
}

impl RenderAttribute for bool {
    fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
        if self { write!(w, " {key}") } else { Ok(()) }
    }
}

impl RenderAttribute for std::fmt::Arguments<'_> {
    fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
        write!(w, " {key}=\"")?;
        EscapeArgWriter::new(w).write_fmt(self)?;
        w.write_char('"')
    }
}

impl<T: RenderAttribute> RenderAttribute for Option<T> {
    fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
        if let Some(value) = self { value.render(w, key) } else { Ok(()) }
    }
}

macro_rules! impl_render_attr_for_str {
    ($($ty:ty),+) => {
        $(
            impl RenderAttribute for $ty {
                #[inline]
                fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
                    write!(w, " {key}=\"")?;
                    write_escape_arg(&self, w)?;
                    w.write_char('"')
                }
            }
        )+
    };
}

impl_render_attr_for_str!(&str, String, Cow<'_, str>);

macro_rules! impl_render_attr_for_num {
    ($($ty:ty),+) => {
        $(
            impl RenderAttribute for $ty {
                fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
                    write!(w, " {key}=\"{}\"", self)
                }
            }
        )+
    };
}

impl_render_attr_for_num!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// Adapts any [`Display`](std::fmt::Display) type as a [`RenderAttribute`].
///
/// Useful for custom types that implement `Display` but are not directly supported
/// as attributes.
pub struct DisplayAttribute<T: std::fmt::Display>(pub T);
impl<T: std::fmt::Display> RenderAttribute for DisplayAttribute<T> {
    fn render(self, w: &mut BytesMut, key: &str) -> std::fmt::Result {
        write!(w, " {key}=\"")?;
        write!(&mut EscapeArgWriter::new(w), "{}", self.0)?;
        w.write_char('"')
    }
}

/// Attribute rendering context.
pub struct AC<'a> {
    /// The underlying output buffer.
    pub writer: &'a mut BytesMut,
}

impl<'a> AC<'a> {
    /// Creates a new attribute context wrapping `writer`.
    pub fn new(writer: &'a mut BytesMut) -> Self {
        Self { writer }
    }

    /// Sets a single attribute `key` to `value`.
    ///
    /// Delegates to [`RenderAttribute::render`].
    ///
    /// # Errors
    ///
    /// Returns [`std::fmt::Error`] if writing to the underlying buffer fails.
    #[inline]
    pub fn set(&mut self, key: &str, value: impl RenderAttribute) -> std::fmt::Result {
        value.render(self.writer, key)
    }
}

/// A collection of attributes consumed during element rendering.
///
/// The element rendering code ([`Element`], [`VoidElement`], [`XmlElement`]) calls
/// `render` to emit all attributes between the tag name and the closing `>`.
///
/// # Implementors
///
/// - Closures `FnOnce(&mut AC) -> fmt::Result` — full control over attribute output
/// - `()` — no attributes
/// - `&str`, `String`, `Cow<'_, str>` — shorthand for `class="..."` with escaping
/// - Tuples of [`RenderAttributeValue`] — space-separated `class="..."` (via [`SpaceSeparated`])
pub trait RenderAttributes {
    /// Render all attributes into the attribute context `cx`.
    ///
    /// # Errors
    ///
    /// Returns [`std::fmt::Error`] if writing to the underlying buffer fails.
    fn render(self, cx: &mut AC) -> std::fmt::Result;
}

impl<F> RenderAttributes for F
where
    F: FnOnce(&mut AC) -> std::fmt::Result,
{
    #[inline]
    fn render(self, cx: &mut AC) -> std::fmt::Result {
        self(cx)
    }
}

impl RenderAttributes for () {
    #[inline]
    fn render(self, _: &mut AC) -> std::fmt::Result {
        Ok(())
    }
}

impl RenderAttributes for SafeStr<'_> {
    #[inline]
    fn render(self, cx: &mut AC) -> std::fmt::Result {
        write!(cx.writer, " class=\"{}\"", self.0)
    }
}

macro_rules! impl_render_attrs_for_str {
    ($($ty:ty),+) => {
        $(
            impl RenderAttributes for $ty {
                #[inline]
                fn render(self, cx: &mut AC) -> std::fmt::Result {
                    cx.writer.write_str(" class=\"")?;
                    write_escape_arg(&self, cx.writer)?;
                    cx.writer.write_char('"')
                }
            }
        )+
    };
}

impl_render_attrs_for_str!(&str, String, Cow<'_, str>);

/// Top-level rendering trait.
///
/// Anything that implements `Render` can be passed to [`CC::add`] to emit its
/// representation into the output buffer.
///
/// The [`is_empty`](Render::is_empty) method allows container elements to skip
/// rendering children when they have no content.
///
/// # Implementors
///
/// | Type | Behavior |
/// |---|---|
/// | `&str`, `String`, `Cow<'_, str>` | Escaped text |
/// | [`SafeStr`] | Raw text (no escaping) |
/// | integers | Rendered via [`write!`](std::write!) |
/// | [`std::fmt::Arguments`] | Formatted with text escaping |
/// | `Option<T>` | Defers to `T`; `is_empty()` is `true` when `None` |
/// | `()` | Nothing (empty); `is_empty()` is `true` |
/// | closures: `FnOnce(&mut CC) -> fmt::Result` | Full control |
/// | [`Element`], [`VoidElement`], [`XmlElement`] | Markup elements |
/// | [`Comment`] | `<!-- ... -->` |
pub trait Render {
    /// Render this value into the content context.
    ///
    /// # Errors
    ///
    /// Returns [`std::fmt::Error`] if writing to the underlying buffer fails.
    fn render(self, cx: &mut CC) -> std::fmt::Result;

    /// Whether this value has no visible content.
    ///
    /// Used by container elements (e.g. [`Element`]) to decide whether to render
    /// children or use a self-closing form.
    #[inline]
    fn is_empty(&self) -> bool {
        false
    }
}

impl Render for () {
    #[inline]
    fn render(self, _: &mut CC) -> std::fmt::Result {
        Ok(())
    }

    #[inline]
    fn is_empty(&self) -> bool {
        true
    }
}

impl<F> Render for F
where
    F: FnOnce(&mut CC) -> std::fmt::Result,
{
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        self(cx)
    }
}

impl Render for SafeStr<'_> {
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        cx.0.write_str(self.0)
    }
}

impl Render for std::fmt::Arguments<'_> {
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        EscapeTextWriter::new(cx.0).write_fmt(self)
    }
}

impl<T: Render> Render for Option<T> {
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        if let Some(value) = self { value.render(cx) } else { Ok(()) }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}

macro_rules! impl_render_for_str {
    ($($ty:ty),+) => {
        $(
            impl Render for $ty {
                #[inline]
                fn render(self, cx: &mut CC) -> std::fmt::Result {
                    write_escape_text(&self, cx.0)
                }
            }
        )+
    };
}

impl_render_for_str!(&str, String, Cow<'_, str>);

macro_rules! impl_render_for_num {
    ($($ty:ty),+) => {
        $(
            impl Render for $ty {
                #[inline]
                fn render(self, cx: &mut CC) -> std::fmt::Result {
                    write!(cx.0, "{}", self)
                }
            }
        )+
    };
}

impl_render_for_num!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// An HTML comment (`<!-- ... -->`).
pub struct Comment<'a>(pub &'a str);

impl Render for Comment<'_> {
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        cx.0.write_str("<!-- ")?;
        write_escape_comment(self.0, cx.0)?;
        cx.0.write_str(" -->")
    }
}

/// A container element that always renders with both open and close tags.
///
/// This is the element type for standard HTML container elements
/// (e.g. `<div>`, `<span>`, `<p>`).
pub struct Element<'a, A, C> {
    /// The tag name.
    pub tag: &'a str,
    /// The attributes.
    pub attrs: A,
    /// The child content.
    pub children: C,
}

impl<'a, A, C> Element<'a, A, C> {
    /// Creates a new element with the given `tag`, `attrs`, and `children`.
    #[inline]
    pub fn new(tag: &'a str, attrs: A, children: C) -> Self {
        Self { tag, attrs, children }
    }
}

impl<A: RenderAttributes, C: Render> Render for Element<'_, A, C> {
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        write!(cx.0, "<{}", self.tag)?;
        self.attrs.render(&mut AC::new(cx.0))?;
        cx.0.write_char('>')?;
        if !self.children.is_empty() {
            self.children.render(cx)?;
        }
        write!(cx.0, "</{}>", self.tag)
    }
}

/// An HTML void element that renders without a close tag.
///
/// Renders as `<tag attrs>` with no closing tag or children.
///
/// This is the element type for HTML void elements (e.g. `<br>`, `<img>`).
pub struct VoidElement<'a, A> {
    /// The tag name.
    pub tag: &'a str,
    /// The attributes.
    pub attrs: A,
}

impl<'a, A> VoidElement<'a, A> {
    /// Creates a new void element with the given `tag` and `attrs`.
    #[inline]
    pub fn new(tag: &'a str, attrs: A) -> Self {
        Self { tag, attrs }
    }
}

impl<A: RenderAttributes> Render for VoidElement<'_, A> {
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        write!(cx.0, "<{}", self.tag)?;
        self.attrs.render(&mut AC::new(cx.0))?;
        cx.0.write_char('>')
    }
}

/// An XML element that self-closes when empty or renders as a container.
///
/// Renders as `<tag attrs/>` when [`Render::is_empty`] returns `true` for children,
/// otherwise as `<tag attrs>children</tag>`.
///
/// This is the element type for SVG and MathML elements.
pub struct XmlElement<'a, A, C> {
    /// The tag name.
    pub tag: &'a str,
    /// The attributes.
    pub attrs: A,
    /// The child content.
    pub children: C,
}

impl<'a, A, C> XmlElement<'a, A, C> {
    /// Creates a new XML element with the given `tag`, `attrs`, and `children`.
    #[inline]
    pub fn new(tag: &'a str, attrs: A, children: C) -> Self {
        Self { tag, attrs, children }
    }
}

impl<A, C> Render for XmlElement<'_, A, C>
where
    A: RenderAttributes,
    C: Render,
{
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        write!(cx.0, "<{}", self.tag)?;
        self.attrs.render(&mut AC::new(cx.0))?;
        if self.children.is_empty() {
            cx.0.write_str("/>")
        } else {
            cx.0.write_char('>')?;
            self.children.render(cx)?;
            write!(cx.0, "</{}>", self.tag)
        }
    }
}

/// A string wrapper that bypasses HTML escaping.
///
/// When a `SafeStr` is rendered as text content or an attribute value, its contents
/// are written verbatim without any HTML entity escaping.
///
/// **Security note:** Only use `SafeStr` with trusted input. Passing user-controlled
/// data through `SafeStr` can introduce cross-site scripting (XSS) vulnerabilities.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SafeStr<'a>(pub &'a str);

impl AsRef<str> for SafeStr<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl std::ops::Deref for SafeStr<'_> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.0
    }
}
