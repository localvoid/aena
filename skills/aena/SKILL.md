---
name: aena
description: Use when working with Rust library `aena` to write HTML,SVG,MathML templates.
---

# aena

`aena` is a template engine for generating HTML,SVG,MathML in Rust language.

## Core Concepts

### Entry Point

`render_html` adds HTML doctype and invokes a closure.

```rust
use bytes::BytesMut;
use aena::{render_html, html, CC};

let mut out = BytesMut::new();
render_html(&mut out, |c| {
    c.add(Doctype)?;
    c.add(html::html((), |c: &mut CC| {
        c.add(html::head((), |c: &mut CC| {
            c.add(html::title((), "Page Title"))
        }))?;
        c.add(html::body((), |c: &mut CC| {
            c.add("Body Text")
        }))
    }))?;
    Ok(())
})?;
```

### Element Method Signature

Every element follows the same pattern:

```rust
html::element_name(attrs, children);
html::void_element_name(attrs);
```

- **attrs**: `()` for none, `"class"` shortcut, or `|a: &mut AC| { ... }` closure (`RenderAttributes` trait)
- **children**: `()` for none, `"text"` for escaped text, or `|c: &mut CC| { ... }` closure for nested elements or component (`Render` trait)

### Attributes

```rust
use aena::{html, AC};

// No attributes
c.add(html::div((), ()));
c.add(html::img(()));

// Class shortcut (first arg = class)
c.add(html::div("container", "content"));
c.add(html::div(("first", "second"), "content"));

// Multiple attributes via closure
c.add(html::div(|a: &mut AC| {
    a.set("id", "main")?;
    a.set("data-value", "42")?;
    a.set("disabled", true)
}, ()));
// Output: <div id="main" data-value="42" disabled></div>
```

### Empty

```rust
c.add(html::div((), ()));
c.add(html::br(())); // void element
```

### Add Text

```rust
// Escaped (default)
c.add("Hello <world>");  // => Hello &lt;world&gt;

// Raw/unescaped
c.add(SafeStr("<br>"));  // => <br>

// Formatted (escaped)
c.add(format_args!("Count: {}", 42));
```

### Nested Elements

```rust
c.add(html::div((), |c: &mut CC| {
    c.add(html::span((), "Hello "))?;
    c.add(html::strong((), "world"))
}))
```

### Comments

```rust
use aena::Comment;

c.add(Comment("TODO: fix this"));  // => <!-- TODO: fix this -->
```

### Custom Elements

```rust
use aena::{Element, VoidElement, XmlElement};

// Normal HTML element
c.add(Element::new("custom", (), "content"));  // => <custom>content</custom>

// Void element (no children param, no closing tag)
c.add(VoidElement::new("custom-void", ()));    // => <custom-void>

// XML element (self-closes when empty)
c.add(XmlElement::new("circle", (), ()));      // => <circle/>
c.add(XmlElement::new("g", (), "text"));       // => <g>text</g>
```

## Component-Based Design

### Define Components

Components are types implementing `Render`:

```rust
use aena::{CC, AC, Render};

pub struct Button<'a> {
    pub label: &'a str,
    pub disabled: bool,
    pub variant: &'a str,
}

impl Render for Button<'_> {
    fn render(self, c: &mut CC) -> std::fmt::Result {
        c.add(html::button(|a: &mut AC| {
            a.set("class", self.variant)?;
            a.set("disabled", self.disabled)
        }, self.label))
    }
}
```

### Render Components

Use `c.add(component)` to render components:

```rust
c.add(Button {
    label: "Click me",
    disabled: false,
    variant: "btn-primary",
})
```

### Nested Components

Components can render other components:

```rust
pub struct Card<'a, C: Render> {
    pub title: &'a str,
    pub children: C,
}

impl<C: Render> Render for Card<'a, C> {
    fn render(self, c: &mut CC) -> std::fmt::Result {
        c.add(html::div("card", |c: &mut CC| {
            c.add(html::h2("card-title", self.title))?;
            c.add(html::div("card-body", self.children))?;
            c.add(Button {
                label: "Action",
                disabled: false,
                variant: "btn-secondary",
            })
        }))
    }
}
```

### Component with children (Slot Pattern)

```rust
pub struct Modal<C: Render> {
    pub title: &'static str,
    pub children: C,
}

impl<C: Render> Render for Modal<C> {
    fn render(self, c: &mut CC) -> std::fmt::Result {
        c.add(html::div("modal-overlay", |c: &mut CC| {
            c.add(html::div("modal", |c: &mut CC| {
                c.add(html::div("modal-header", |c: &mut CC| {
                    c.add(html::h3((), self.title))
                }))?;
                c.add(html::div("modal-body", |c: &mut CC| {
                    c.add(self.children)
                }))
            }))
        }))
    }
}

// Usage:
c.add(Modal {
    title: "Confirm",
    children: |c: &mut CC| {
        c.add(html::p((), "Are you sure?"))?;
        c.add(Button {
            label: "Yes",
            disabled: false,
            variant: "btn-primary",
        })
    },
})
```

## Helpers

### Class Names

```rust
// As first arg:
c.add(html::div(("btn", "btn-primary"), "Click me"))
// => <div class="btn btn-primary">Click me</div>

// With conditional classes:
c.add(html::div(("btn", is_active.then_some("active")), "Click me"))

// Inside attr closure:
c.add(html::div(|a: &mut AC| {
    a.set("id", "main")?;
    a.set("class", ("container", "flex"))
}, "content"))
```

Supports `&str`, `String`, `SafeStr`, and `Option<T>` values. Empty strings and `None` are
silently skipped. Max 8 items.

### `style()`

```rust
c.add(html::div(|a: &mut AC| {
    a.set("style", style(|s: &mut Style| {
        s.set("color", "red")?;
        s.set("font-size", "1rem")
    }))
}, "content"))
```

Supports `&str`, `String`, `Option<T>`, and any [`RenderAttributeValue`]. No limit on property count.

## SVG and MathML

### SVG

```rust
use aena::svg;

c.add(svg::svg(|a: &mut AC| {
    a.set("width", "100")?;
    a.set("height", "100")
}, |c: &mut CC| {
    c.add(svg::circle(|a: &mut AC| {
        a.set("cx", "50")?;
        a.set("cy", "50")?;
        a.set("r", "40")
    }, ()))
}));
```

### MathML

```rust
use aena::{Element, mathml as m};

c.add(Element::new("math", (), |c: &mut CC| {
    c.add(m::mrow((), |c: &mut CC| {
        c.add(m::mi((), "x"))?;
        c.add(m::mo((), "="))?;
        c.add(m::mfrac((), |c: &mut CC| {
            c.add(m::mi((), "a"))?;
            c.add(m::mi((), "b"))
        }))
    }))
}));
```

## Key Types

- `CC` - Children context, used for rendering children content
- `AC` - Attribute context, passed to attribute closures
- `Render` - Trait for renderable types
- `RenderAttribute` - Trait for attribute values passed to `a.set(key, value)`
- `RenderAttributes` - Trait for attribute sets (first arg to element factories)
- `Element` - Normal HTML element with closing tag
- `VoidElement` - HTML void element (no closing tag)
- `XmlElement` - XML element (self-closes when empty)
- `Doctype` - `<!doctype html>` declaration
- `Comment` - HTML comment `<!-- ... -->`
- `SafeStr` - Unescaped text content
- `DisplayAttribute<T>` - Wrapper to render `Display` types as attribute values
- `SpaceSeparated<T>` - Space-separated list (used by class tuples)
- `Style` - Builder context for CSS declarations (passed to `style()` closures)
- `StyleClosure` - Style attribute value (created by `style()`, wraps a closure)

## Best Practices

- Use `"class-name"` as first arg for simple class attributes
- Use tuples `("a", "b")` for multiple classes
- Use closures `|a: &mut AC| { a.set(...) }` for complex attributes
- Create components with `Component { prop: "value" }` instead of `Component::new("value")`
- Prefer reusable and composable component-based design
