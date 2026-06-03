//! HTML element factory functions.
//!
//! This module provides factory functions for all standard HTML elements. Container
//! elements (e.g. `div`, `span`) return [`Element`](crate::Element); void elements
//! (e.g. `br`, `img`) return [`VoidElement`](crate::VoidElement).
//!
//! This module is enabled by the `html` feature (default on).

use std::fmt::Write as _;

use crate::{CC, Render, define_html_elements, define_html_void_elements};

/// `<!doctype html>` declaration.
///
/// Implements [`Render`] and is automatically prepended by
/// [`render_html`](crate::render_html).
///
/// # Examples
///
/// ```rust
/// use aena::{CC, html::Doctype, Render};
/// use bytes::BytesMut;
///
/// let mut buf = BytesMut::new();
/// CC(&mut buf).add(Doctype).unwrap();
/// assert_eq!(&buf[..], b"<!doctype html>");
/// ```
pub struct Doctype;

impl Render for Doctype {
    #[inline]
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        cx.0.write_str("<!doctype html>")
    }
}

define_html_elements! {
    html => "html",
    head => "head",
    body => "body",
    title => "title",
    style => "style",
    script => "script",
    noscript => "noscript",
    div => "div",
    span => "span",
    p => "p",
    pre => "pre",
    a => "a",
    h1 => "h1",
    h2 => "h2",
    h3 => "h3",
    h4 => "h4",
    h5 => "h5",
    h6 => "h6",
    ul => "ul",
    ol => "ol",
    li => "li",
    dl => "dl",
    dt => "dt",
    dd => "dd",
    table => "table",
    thead => "thead",
    tbody => "tbody",
    tfoot => "tfoot",
    tr => "tr",
    th => "th",
    td => "td",
    caption => "caption",
    colgroup => "colgroup",
    form => "form",
    button => "button",
    label => "label",
    select => "select",
    option => "option",
    optgroup => "optgroup",
    textarea => "textarea",
    fieldset => "fieldset",
    legend => "legend",
    datalist => "datalist",
    output => "output",
    meter => "meter",
    progress => "progress",
    nav => "nav",
    main => "main",
    header => "header",
    footer => "footer",
    article => "article",
    section => "section",
    aside => "aside",
    details => "details",
    summary => "summary",
    dialog => "dialog",
    figure => "figure",
    figcaption => "figcaption",
    blockquote => "blockquote",
    menu => "menu",
    strong => "strong",
    em => "em",
    code => "code",
    small => "small",
    b => "b",
    i => "i",
    u => "u",
    s => "s",
    sub => "sub",
    sup => "sup",
    mark => "mark",
    abbr => "abbr",
    time => "time",
    cite => "cite",
    q => "q",
    kbd => "kbd",
    samp => "samp",
    var => "var",
    ruby => "ruby",
    rt => "rt",
    rp => "rp",
    audio => "audio",
    video => "video",
    canvas => "canvas",
    iframe => "iframe",
    picture => "picture",
    map => "map",
    slot => "slot",
    template => "template",
    address => "address",
    bdi => "bdi",
    bdo => "bdo",
    data => "data",
    del => "del",
    dfn => "dfn",
    hgroup => "hgroup",
    ins => "ins",
    object => "object",
    search => "search",
}

define_html_void_elements! {
    meta => "meta",
    base => "base",
    link => "link",
    input => "input",
    col => "col",
    img => "img",
    br => "br",
    hr => "hr",
    area => "area",
    wbr => "wbr",
    source => "source",
    track => "track",
    embed => "embed",
    param => "param",
    selectedcontent => "selectedcontent",
}
