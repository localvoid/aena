use aena::{AC, CC, Comment, DisplayAttribute, Element, SafeStr, VoidElement, XmlElement};
use bytes::BytesMut;

fn render(f: impl FnOnce(&mut CC) -> std::fmt::Result) -> String {
    let mut out = BytesMut::new();
    f(&mut CC(&mut out)).unwrap();
    String::from_utf8(out.freeze().into()).unwrap()
}

#[test]
fn test_option_some() {
    let out = render(|c: &mut CC| {
        let v: Option<&str> = Some("text");
        c.add(v)
    });
    assert_eq!(out, "text");
}

#[test]
fn test_option_none() {
    let out = render(|c: &mut CC| {
        let v: Option<&str> = None;
        c.add(v)
    });
    assert_eq!(out, "");
}

#[test]
fn test_unit_renders_nothing() {
    let out = render(|c: &mut CC| c.add(()));
    assert_eq!(out, "");
}

#[test]
fn test_element_with_empty_children() {
    let out = render(|c: &mut CC| c.add(Element::new("div", (), ())));
    assert_eq!(out, "<div></div>");
}

#[test]
fn test_void_element() {
    let out = render(|c: &mut CC| c.add(VoidElement::new("br", ())));
    assert_eq!(out, "<br>");
}

#[test]
fn test_xml_element_self_closing() {
    let out = render(|c: &mut CC| c.add(XmlElement::new("circle", (), ())));
    assert_eq!(out, "<circle/>");
}

#[test]
fn test_xml_element_with_children() {
    let out = render(|c: &mut CC| c.add(XmlElement::new("g", (), "text")));
    assert_eq!(out, "<g>text</g>");
}

#[test]
fn test_xml_element_with_class_self_closing() {
    let out = render(|c: &mut CC| c.add(XmlElement::new("circle", ("big",), ())));
    assert_eq!(out, "<circle class=\"big\"/>");
}

#[test]
fn test_comment_double_dash_escaping() {
    let out = render(|c: &mut CC| c.add(Comment("foo -- bar")));
    assert_eq!(out, "<!-- foo -&#45; bar -->");
}

#[test]
fn test_comment_simple() {
    let out = render(|c: &mut CC| c.add(Comment("plain text")));
    assert_eq!(out, "<!-- plain text -->");
}

#[test]
fn test_safe_str_as_attribute() {
    let out = render(|c: &mut CC| {
        c.add(Element::new(
            "a",
            |a: &mut AC| a.set("href", SafeStr("https://example.com/?a=1&b=2")),
            "link",
        ))
    });
    assert_eq!(out, "<a href=\"https://example.com/?a=1&b=2\">link</a>");
}

#[test]
fn test_safe_str_class_shorthand() {
    let out = render(|c: &mut CC| c.add(Element::new("div", SafeStr("a&b"), "x")));
    assert_eq!(out, "<div class=\"a&b\">x</div>");
}

#[test]
fn test_display_attribute() {
    let out = render(|c: &mut CC| {
        c.add(Element::new(
            "div",
            |a: &mut AC| a.set("data-value", DisplayAttribute(42u32)),
            (),
        ))
    });
    assert_eq!(out, "<div data-value=\"42\"></div>");
}

#[test]
fn test_display_attribute_escapes() {
    let out = render(|c: &mut CC| {
        c.add(Element::new(
            "div",
            |a: &mut AC| a.set("data-value", DisplayAttribute(r#"a"b&c"#)),
            (),
        ))
    });
    assert_eq!(out, "<div data-value=\"a&quot;b&amp;c\"></div>");
}

#[test]
fn test_numeric_attribute_u32() {
    let out = render(|c: &mut CC| {
        c.add(Element::new(
            "div",
            |a: &mut AC| a.set("tabindex", 5u32),
            (),
        ))
    });
    assert_eq!(out, "<div tabindex=\"5\"></div>");
}

#[test]
fn test_numeric_attribute_i32() {
    let out = render(|c: &mut CC| {
        c.add(Element::new(
            "div",
            |a: &mut AC| a.set("data-x", -7i32),
            (),
        ))
    });
    assert_eq!(out, "<div data-x=\"-7\"></div>");
}

#[test]
fn test_option_attribute_some() {
    let out = render(|c: &mut CC| {
        c.add(Element::new(
            "div",
            |a: &mut AC| a.set("id", Some("main")),
            (),
        ))
    });
    assert_eq!(out, "<div id=\"main\"></div>");
}

#[test]
fn test_option_attribute_none() {
    let out = render(|c: &mut CC| {
        let none_attr: Option<&str> = None;
        c.add(Element::new(
            "div",
            |a: &mut AC| {
                a.set("id", Some("main"))?;
                a.set("data-x", none_attr)
            },
            (),
        ))
    });
    assert_eq!(out, "<div id=\"main\"></div>");
}

#[test]
fn test_bool_attribute_true() {
    let out = render(|c: &mut CC| {
        c.add(VoidElement::new("input", |a: &mut AC| a.set("disabled", true)))
    });
    assert_eq!(out, "<input disabled>");
}

#[test]
fn test_bool_attribute_false() {
    let out = render(|c: &mut CC| {
        c.add(VoidElement::new("input", |a: &mut AC| a.set("disabled", false)))
    });
    assert_eq!(out, "<input>");
}

#[test]
fn test_nested_elements() {
    let out = render(|c: &mut CC| {
        c.add(Element::new("div", (), |c: &mut CC| {
            c.add(Element::new("span", (), "hello"))?;
            c.add(Element::new("span", (), "world"))
        }))
    });
    assert_eq!(out, "<div><span>hello</span><span>world</span></div>");
}

#[test]
fn test_format_args_render() {
    let out = render(|c: &mut CC| {
        c.add(Element::new("div", (), |c: &mut CC| c.add(format_args!("n = {}", 42))))
    });
    assert_eq!(out, "<div>n = 42</div>");
}

#[test]
fn test_format_args_escapes() {
    let out = render(|c: &mut CC| {
        c.add(Element::new("div", (), |c: &mut CC| c.add(format_args!("<{}>", "x"))))
    });
    assert_eq!(out, "<div>&lt;x&gt;</div>");
}

#[test]
fn test_text_newlines_preserved() {
    let out = render(|c: &mut CC| c.add(Element::new("pre", (), "a\nb")));
    assert_eq!(out, "<pre>a\nb</pre>");
}
