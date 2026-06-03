//! Lightweight HTML/SVG/MathML renderer.
//!
//! `aena` provides a composable, type-safe API for generating markup.
//!
//! # Types
//!
//! ## Content
//!
//! | Trait / Type | Role |
//! |---|---|
//! | [`CC`] | Content context |
//! | [`Render`] | Renderable content |
//! | [`Element`] | `<tag attrs>children</tag>` |
//! | [`VoidElement`] | `<tag attrs>` (no close tag) |
//! | [`XmlElement`] | `<tag/>` or `<tag>…</tag>` |
//!
//! ## Attributes
//!
//! | Trait / Type | Role |
//! |---|---|
//! | [`AC`] | Attributes context |
//! | [`SC`] | Styles context |
//! | [`RenderAttributes`] | Collection of renderable attributes |
//! | [`RenderAttribute`] | Renderable `key="value"` attribute pair |
//! | [`RenderAttributeValue`] | Renderable attribute value |
//!
//! ## Attribute shorthands
//!
//! A string type as first argument renders as `class="…"`.
//!
//! Tuples of [`RenderAttributeValue`] render as space-separated values.
//!
//! ```rust,ignore
//! html::div(("main", "page"), "content")
//! // or
//! html::div(|a: &mut AC| { a.set("class", ("main", "page")); }, "content")
//! ```
//!
//! For full control over attributes, use a closure:
//!
//! ```rust,ignore
//! html::div(|a: &mut AC| { a.set("id", "main"); }, "content")
//! ```
//!
//! Boolean attributes are handled via `bool`:
//!
//! ```rust,ignore
//! html::input(|a: &mut AC| { a.set("disabled", true); }) // <input disabled>
//! ```
//!
//! ## Inline styles
//!
//! The [`style()`] function builds inline style attributes via a builder closure:
//!
//! ```rust,ignore
//! html::div(|a: &mut AC| a.set("style", style(|s| s.set("color", "red"))), "text")
//! // <div style="color:red">text</div>
//! ```
//!
//! ## Escaping
//!
//! | Context | Escaped characters |
//! |---|---|
//! | Text content | `&`, `<`, `>` |
//! | Attribute values | `&`, `"` |
//! | Script / style | `&`, `<`, `>`, `/` |
//!
//! Wrap a string in [`SafeStr`] to emit raw unescaped content.
//! **Warning:** [`SafeStr`] bypasses all escaping — only use with trusted input.
//!
//! # Feature flags
//!
//! - `html` — HTML element factories (enabled by default)
//! - `svg` — SVG element factories (enabled by default)
//! - `mathml` — `MathML` element factories (enabled by default)

pub mod escape;
#[cfg(feature = "html")]
pub mod html;
mod macros;
#[cfg(feature = "mathml")]
pub mod mathml;
mod render;
#[cfg(feature = "svg")]
pub mod svg;

pub use render::*;

/// Creates a HTML document by prepending `<!doctype html>` to rendered content.
///
/// The closure receives a [`CC`] context and can add any [`Render`] values.
///
/// Only available with the `html` feature enabled.
///
/// # Errors
///
/// Returns [`std::fmt::Error`] if rendering fails.
///
/// # Examples
///
/// ```rust
/// use bytes::BytesMut;
/// use aena::render_html;
///
/// let mut out = BytesMut::new();
/// render_html(&mut out, |cx| {
///     cx.add("hello")
/// }).unwrap();
/// assert_eq!(out, "<!doctype html>hello");
/// ```
#[cfg(feature = "html")]
pub fn render_html<F>(bytes: &mut bytes::BytesMut, f: F) -> std::fmt::Result
where
    F: FnOnce(&mut render::CC) -> std::fmt::Result,
{
    let mut cx = render::CC(bytes);
    cx.add(html::Doctype)?;
    f(&mut cx)
}
