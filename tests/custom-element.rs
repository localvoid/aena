use aena::{CC, Element};
use bytes::BytesMut;

fn render(f: impl FnOnce(&mut CC) -> std::fmt::Result) -> String {
    let mut out = BytesMut::new();
    f(&mut CC(&mut out)).unwrap();
    String::from_utf8(out.freeze().into()).unwrap()
}

#[test]
fn test_custom_element() {
    let out = render(|c| c.add(Element::new("custom-element", (), "content")));
    assert_eq!(out, "<custom-element>content</custom-element>");
}
