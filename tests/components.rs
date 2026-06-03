use aena::{AC, CC, Render, html};
use bytes::BytesMut;

fn render(f: impl FnOnce(&mut CC) -> std::fmt::Result) -> String {
    let mut out = BytesMut::new();
    f(&mut CC(&mut out)).unwrap();
    String::from_utf8(out.freeze().into()).unwrap()
}

pub struct Button<'a> {
    pub label: &'a str,
    pub disabled: bool,
    pub variant: &'a str,
}

impl Render for Button<'_> {
    fn render(self, cx: &mut CC) -> std::fmt::Result {
        cx.add(html::button(
            |a: &mut AC| {
                a.set("class", self.variant)?;
                a.set("disabled", self.disabled)
            },
            self.label,
        ))
    }
}

#[test]
fn test_component() {
    let out =
        render(|c: &mut CC| c.add(Button { label: "test", disabled: true, variant: "primary" }));
    assert_eq!(out, "<button class=\"primary\" disabled>test</button>");
}

pub struct Card<'a, C: Render> {
    pub title: &'a str,
    pub children: C,
}

impl<C: Render> Render for Card<'_, C> {
    fn render(self, c: &mut CC) -> std::fmt::Result {
        c.add(html::div("card", |c: &mut CC| {
            c.add(html::h2("card-title", self.title))?;
            c.add(html::div("card-body", self.children))?;
            c.add(Button { label: "Action", disabled: false, variant: "btn-secondary" })?;
            Ok(())
        }))
    }
}

#[test]
fn test_nested_component() {
    let out = render(|c: &mut CC| c.add(Card { title: "test", children: "children" }));
    assert_eq!(
        out,
        "<div class=\"card\"><h2 class=\"card-title\">test</h2><div class=\"card-body\">children</div><button class=\"btn-secondary\">Action</button></div>"
    );
}
