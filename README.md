# aena

Lightweight HTML/SVG/MathML renderer.

- **Dependencies**: `std`, `bytes`
- **Composition**: components
- **Escaping**: context-dependent escaping rules
- **Macro-Free**: pure functions

```rust
use bytes::BytesMut;
use aena::{render_html, html as h, CC};

let mut out = BytesMut::new();
render_html(&mut out, |c| {
    c.add(h::html((), |c: &mut CC| {
        c.add(h::head((), |c: &mut CC| {
            c.add(h::title((), "Hello"))
        }))?;
        c.add(h::body((), |c: &mut CC| {
            c.add(h::div("container", |c: &mut CC| {
                c.add(h::h1((), "Welcome"))?;
                c.add(h::p((), "text"))
            }))
        }))
    }))?;
    Ok(())
})
```

## API

### Element Factories

`html`, `svg`, `mathml` modules provide element factories:

```rust
html::div((), "content")
html::img(|a: &mut AC| a.set("href", "logo.png"))
svg::circle((), ())
```

- **HTML elements**: `html::div`, `html::span`, …
- **HTML void elements**: `html::img`, `html::br`, …
- **XML elements**: `svg::circle`, `mathml::mrow`, …

### Attributes

```rust
// None
html::div((), ())

// Class shortcut (first arg)
html::div("container", "text")
html::div(("btn", "primary"), "text")

// Conditional classes
html::div(("btn", is_active.then_some("active")), ())

// Inline styles
html::div(|a: &mut AC| {
    a.set("style", style(|s: &mut SC| {
        s.set("color", "red")?;
        s.set("font-size", "1rem")
    }))
}, ())

// Closure
html::div(|a: &mut AC| {
    a.set("id", "main")?;
    a.set("data-val", "42")?;
    a.set("disabled", true)
}, ())
```

### Text

```rust
html::div((), "escaped <text>")       // &lt;text&gt;
html::div((), SafeStr("<raw>"))       // <raw> (unescaped)
html::div((), format_args!("{}", n))  // formatted, escaped

// Also works with children context `CC`
html::div((), |c: &mut CC| {
    c.add("text")
})
```

### Components

Implement `Render` on your types:

```rust
impl Render for Button<'_> {
    fn render(self, c: &mut CC) -> std::fmt::Result {
        c.add(html::button(
            |a: &mut AC| {
                a.set("class", self.variant)?;
                a.set("disabled", self.disabled)
            },
            self.label,
        ))
    }
}

c.add(Button { label: "Click", disabled: false, variant: "primary" });
```

### Custom Elements

```rust
Element::new("my-tag", "class-name", "content")
VoidElement::new("my-void", ())
XmlElement::new("circle", ("big",), ())
```

### Misc

```rust
html::Doctype // <!doctype html>
Comment("note") // <!-- note -->
```

## Tradeoffs

Everything is coupled to `BytesMut` rather than abstracting over `std::fmt::Write` to keep user code less verbose.

## Features

All enabled by default:

- `html` — HTML element factories
- `svg` — SVG element factories
- `mathml` — MathML element factories

## License

MIT or Apache-2.0
