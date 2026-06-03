use aena::{AC, CC, Comment, html as h, render_html, style};
use bytes::BytesMut;

fn render(f: impl FnOnce(&mut CC) -> std::fmt::Result) -> String {
    let mut out = BytesMut::new();
    f(&mut CC(&mut out)).unwrap();
    String::from_utf8(out.freeze().into()).unwrap()
}

#[test]
fn test_entry() {
    let mut out = BytesMut::new();
    render_html(&mut out, |c| c.add(h::div((), ()))).unwrap();
    assert_eq!(out, "<!doctype html><div></div>");
}

#[test]
fn test_div_empty() {
    let out = render(|c| c.add(h::div((), ())));
    assert_eq!(out, "<div></div>");
}

#[test]
fn test_div_with_text() {
    let out = render(|c: &mut CC| c.add(h::div((), "Hello")));
    assert_eq!(out, "<div>Hello</div>");
}

#[test]
fn test_div_with_class() {
    let out = render(|c: &mut CC| c.add(h::div("container", "content")));
    assert_eq!(out, "<div class=\"container\">content</div>");
}

#[test]
fn test_multiple_elements() {
    let out = render(|c: &mut CC| {
        c.add(h::h1((), "H1"))?;
        c.add(h::h2((), "H2"))?;
        c.add(h::h3((), "H3"))?;
        Ok(())
    });
    assert_eq!(out, "<h1>H1</h1><h2>H2</h2><h3>H3</h3>");
}

#[test]
fn test_attributes_closure() {
    let out = render(|c: &mut CC| {
        c.add(h::div(
            |a: &mut AC| {
                a.set("id", "main")?;
                a.set("data-value", "42")
            },
            "content",
        ))
    });
    assert_eq!(out, "<div id=\"main\" data-value=\"42\">content</div>");
}

#[test]
fn test_void_element() {
    let out = render(|c: &mut CC| c.add(h::img(|a: &mut AC| a.set("src", "/img.png"))));
    assert_eq!(out, "<img src=\"/img.png\">");
}

#[test]
fn test_void_element_attributes() {
    let out = render(|c: &mut CC| {
        c.add(h::input(|a: &mut AC| {
            a.set("type", "text")?;
            a.set("name", "username")?;
            Ok(())
        }))
    });
    assert_eq!(out, "<input type=\"text\" name=\"username\">");
}

#[test]
fn test_attributes_bool_true() {
    let out = render(|c: &mut CC| c.add(h::input(|a: &mut AC| a.set("disabled", true))));
    assert_eq!(out, "<input disabled>");
}

#[test]
fn test_attributes_bool_false() {
    let out = render(|c: &mut CC| c.add(h::input(|a: &mut AC| a.set("disabled", false))));
    assert_eq!(out, "<input>");
}

#[test]
fn test_text_escaping() {
    let out = render(|c: &mut CC| c.add(h::div((), "<script>alert('xss')</script>")));
    assert_eq!(out, "<div>&lt;script&gt;alert('xss')&lt;/script&gt;</div>");
}

#[test]
fn test_text_escaping_ampersand() {
    let out = render(|c: &mut CC| c.add(h::div((), "A & B")));
    assert_eq!(out, "<div>A &amp; B</div>");
}

#[test]
fn test_text_escaping_quotes() {
    let out = render(|c: &mut CC| c.add(h::div((), "He said \"hello\"")));
    assert_eq!(out, "<div>He said \"hello\"</div>");
}

#[test]
fn test_comment() {
    let out = render(|c: &mut CC| c.add(Comment("This is a comment")));
    assert_eq!(out, "<!-- This is a comment -->");
}

#[test]
fn test_classes_single() {
    let out = render(|c: &mut CC| c.add(h::div("container", "content")));
    assert_eq!(out, "<div class=\"container\">content</div>");
}

#[test]
fn test_classes_multiple() {
    let out = render(|c: &mut CC| c.add(h::div(("btn", "btn-primary"), "click")));
    assert_eq!(out, "<div class=\"btn btn-primary\">click</div>");
}

#[test]
fn test_classes_with_option_some() {
    let out = render(|c: &mut CC| {
        let active = Some("active");
        c.add(h::div(("btn", active), "click"))
    });
    assert_eq!(out, "<div class=\"btn active\">click</div>");
}

#[test]
fn test_classes_with_option_none() {
    let out = render(|c: &mut CC| {
        let active: Option<&str> = None;
        c.add(h::div(("btn", active, "primary"), "click"))
    });
    assert_eq!(out, "<div class=\"btn primary\">click</div>");
}

#[test]
fn test_classes_as_into_attrs() {
    let out = render(|c| {
        c.add(h::div(("outer", "wrapper"), |c: &mut CC| c.add(h::span("inner", "text"))))
    });
    assert_eq!(out, "<div class=\"outer wrapper\"><span class=\"inner\">text</span></div>");
}

#[test]
fn test_classes_in_attr_closure() {
    let out = render(|c: &mut CC| {
        c.add(h::div(
            |a: &mut AC| {
                a.set("id", "main")?;
                a.set("class", ("container", "flex"))
            },
            "content",
        ))
    });
    assert_eq!(out, "<div id=\"main\" class=\"container flex\">content</div>");
}

#[test]
fn test_classes_conditional() {
    let out = render(|c: &mut CC| {
        let is_active = true;
        c.add(h::div(("btn", is_active.then_some("active")), "click"))
    });
    assert_eq!(out, "<div class=\"btn active\">click</div>");
}

#[test]
fn test_classes_conditional_false() {
    let out = render(|c: &mut CC| {
        let is_active = false;
        c.add(h::div(("btn", is_active.then_some("active")), "click"))
    });
    assert_eq!(out, "<div class=\"btn\">click</div>");
}

#[test]
fn test_classes_with_owned_string() {
    let out = render(|c: &mut CC| c.add(h::div(("btn", String::from("dynamic")), "click")));
    assert_eq!(out, "<div class=\"btn dynamic\">click</div>");
}

#[test]
fn test_style_in_attr_closure() {
    let out = render(|c: &mut CC| {
        c.add(h::div(
            |a: &mut AC| {
                a.set("id", "main")?;
                a.set(
                    "style",
                    style(|s| {
                        s.set("color", "red")?;
                        s.set("font-size", "1rem")
                    }),
                )?;
                Ok(())
            },
            "content",
        ))
    });
    assert_eq!(out, "<div id=\"main\" style=\"color:red;font-size:1rem\">content</div>");
}
