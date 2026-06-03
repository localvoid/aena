/// Generates HTML container-element factory functions.
///
/// Each entry produces a function with signature:
///
/// `pub fn $name<A: RenderAttributes, C: Render>(attrs: A, children: C) -> Element<'static, A, C>`
///
/// The returned [`Element`](crate::Element) always renders with an open and close tag
/// (e.g. `<div>...</div>`).
///
/// # Syntax
///
/// ```ignore
/// define_html_elements! {
///     div => "div",
///     span => "span",
/// }
/// ```
#[macro_export]
macro_rules! define_html_elements {
    { $( $method:ident => $tag:literal ),* $(,)? } => {
        $(
            #[inline]
            pub fn $method<A: $crate::RenderAttributes, C: $crate::Render>(
                attrs: A,
                children: C,
            ) -> $crate::Element<'static, A, C> {
                $crate::Element::new($tag, attrs, children)
            }
        )*
    };
}

/// Generates HTML void-element factory functions.
///
/// Each entry produces a function with signature:
///
/// `pub fn $name<A: RenderAttributes>(attrs: A) -> VoidElement<'static, A>`
///
/// The returned [`VoidElement`](crate::VoidElement) renders without a close tag
/// (e.g. `<br>`, `<img ...>`).
///
/// # Syntax
///
/// ```ignore
/// define_html_void_elements! {
///     br => "br",
///     img => "img",
/// }
/// ```
#[macro_export]
macro_rules! define_html_void_elements {
    { $( $method:ident => $tag:literal ),* $(,)? } => {
        $(
            #[inline]
            pub fn $method<A: $crate::RenderAttributes>(
                attrs: A,
            ) -> $crate::VoidElement<'static, A>  {
                $crate::VoidElement { tag: $tag, attrs }
            }
        )*
    };
}

/// Generates XML element factory functions.
///
/// Each entry produces a function with signature:
///
/// `pub fn $name<A: RenderAttributes, C: Render>(attrs: A, children: C) -> XmlElement<'static, A, C>`
///
/// The returned [`XmlElement`](crate::XmlElement) self-closes when children are empty
/// (e.g. `<path/>`), otherwise renders as a container (e.g. `<g>...</g>`).
///
/// # Syntax
///
/// ```ignore
/// define_xml_elements! {
///     g => "g",
///     path => "path",
/// }
/// ```
#[macro_export]
macro_rules! define_xml_elements {
    { $( $method:ident => $tag:literal ),* $(,)? } => {
        $(
            #[inline]
            pub fn $method<A: $crate::RenderAttributes, C: $crate::Render>(
                attrs: A,
                children: C,
            ) -> $crate::XmlElement<'static, A, C>  {
                $crate::XmlElement { tag: $tag, attrs, children }
            }
        )*
    };
}
