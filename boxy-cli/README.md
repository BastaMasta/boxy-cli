# boxy-cli
[![Github](https://img.shields.io/badge/GitHub-BastaMasta%2Fboxy--cli-blue?style=flat-square&logo=github)](https://github.com/BastaMasta/boxy-cli)
[![Crates.io](https://img.shields.io/crates/v/boxy-cli?style=flat-square&logo=rust&color=orange)](https://crates.io/crates/boxy-cli)
[![Docs.rs](https://img.shields.io/badge/docs.rs-boxy--cli-66c2a5?style=flat-square&logo=docs.rs)](https://docs.rs/boxy-cli/latest/)
[![Crates.io](https://img.shields.io/crates/d/boxy-cli?style=flat-square)](https://crates.io/crates/boxy-cli)
[![Apache License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-APACHE)
[![MIT License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/BastaMasta/boxy-cli/rust.yml?branch=main&style=flat-square)](https://github.com/BastaMasta/boxy-cli/actions/workflows/rust.yml?query=branch%3Amain)

**A Rust crate for creating styled, multi-segment text boxes in the terminal.**

Dual-licensed under [Apache 2.0](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-APACHE) or [MIT](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-MIT).

---

## Features

- **9 border styles** — classic ASCII, single, double, bold, rounded, bold-corners, and more
- **True-color support** — hex color codes (`#rrggbb`) for borders and per-line text
- **Multi-segment boxes** — stack sections separated by horizontal dividers
- **Columnar layouts** — side-by-side columns inside a single box, with configurable width ratios and correct junction characters (`┼` / `┬` / `┴`) where column boundaries meet across adjacent segments
- **Automatic word wrapping** — wraps to terminal width, respecting internal padding
- **Text alignment** — left, center, or right per segment
- **Terminal-aware sizing** — auto-sizes to terminal width, or set a fixed width
- **Two APIs** — imperative `Boxy` struct and fluent `BoxyBuilder`
- **Macro support** — `boxy!` for quick one-liners

---

## Installation

```toml
[dependencies]
boxy-cli = "2.1.0"
```

Or:

```bash
cargo add boxy-cli
```

---

## Quick Start

### Builder API

```rust
use boxy_cli::prelude::*;

Boxy::builder()
    .box_type(BoxType::Double)
    .color("#00ffff")
    .add_segment("Hello, boxy-cli!", "#ffffff", BoxAlign::Center)
    .add_segment("A terminal box library for Rust.", "#32CD32", BoxAlign::Left)
    .add_line("Second line in the same segment.", "#aaaaaa")
    .padding(BoxPad::uniform(1), BoxPad::vh(1, 2))
    .build()
    .display();
```

### Imperative API

```rust
use boxy_cli::prelude::*;

let mut b = Boxy::new(BoxType::Bold, "#00ffff");
b.add_text_sgmt("Hello, boxy-cli!", "#ffffff", BoxAlign::Center);
b.add_text_sgmt("A terminal box library for Rust.", "#32CD32", BoxAlign::Left);
b.add_text_line("Second line in the same segment.", "#aaaaaa");
b.display();
```

### Macro

```rust
use boxy_cli::prelude::*;

let mut b = boxy!(type: BoxType::Double, color: "#00ffff", internal_pad: 1, alignment: BoxAlign::Left);
b.add_text_sgmt("Hello from the macro!", "#ffffff", BoxAlign::Center);
b.display();
```

---

## Columnar Layouts

Columnar segments let you place content side-by-side inside one box. Column widths are
controlled by ratio values — `vec![1, 2, 1]` gives the middle column twice the space of the
others.

Where column boundaries from adjacent segments coincide, `boxy-cli` renders the correct
junction character automatically:

```
┌─────────────────────────────────────────────────┐
│                  Project Status                 │
├─────────────────┼─────────────────┴─────────────┤   <- ┼ where both have a boundary
│ Lumio V2        │ Shipped         Internship     │      ┴ where only the top segment does
└─────────────────────────────────────────────────┘
```

```rust
use boxy_cli::prelude::*;

let mut b = Boxy::new(BoxType::Single, "#00ffff");

// Plain text header segment
b.add_text_sgmt("Project Status", "#ffffff", BoxAlign::Center);

// 3-column segment with equal widths
b.add_col_text_sgmt(BoxAlign::Left, 3);
b.add_col_text_line("Name",     "#aaaaaa", &0usize);
b.add_col_text_line("Status",   "#aaaaaa", &1usize);
b.add_col_text_line("Notes",    "#aaaaaa", &2usize);
b.add_col_text_line("Lumio V2", "#ffffff", &0usize);
b.add_col_text_line("Shipped",  "#32CD32", &1usize);
b.add_col_text_line("Internship project", "#ffffff", &2usize);

// Optional: customize column width ratios (must match column count)
b.set_segment_ratios(1, vec![1, 1, 2]);

b.display();
```

Or using the builder:

```rust
use boxy_cli::prelude::*;

Boxy::builder()
    .add_segment("Project Status", "#ffffff", BoxAlign::Center)
    .add_col_segment(BoxAlign::Left, 3)
    .add_col_line("Name",     "#aaaaaa", 0)
    .add_col_line("Status",   "#aaaaaa", 1)
    .add_col_line("Notes",    "#aaaaaa", 2)
    .add_col_line("Lumio V2", "#ffffff", 0)
    .add_col_line("Shipped",  "#32CD32", 1)
    .add_col_line("Internship project", "#ffffff", 2)
    .segment_ratios(1, vec![1, 1, 2])
    .build()
    .display();
```

---

## Border Styles

| `BoxType` variant     | Appearance |
|-----------------------|------------|
| `Single`              | `┌─┐` / `│` / `└─┘` |
| `Double`              | `╔═╗` / `║` / `╚═╝` |
| `Bold`                | `┏━┓` / `┃` / `┗━┛` |
| `Rounded`             | `╭─╮` / `│` / `╰─╯` |
| `DoubleHorizontal`    | `╒═╕` / `│` / `╘═╛` |
| `DoubleVertical`      | `╓─╖` / `║` / `╙─╜` |
| `BoldCorners`         | `┍━┑` / `│` / `┕━┙` |
| `Classic`             | `+-+` / `\|` / `+-+` |
| `Empty`               | invisible borders |

---

## Padding

`BoxPad` controls spacing between the terminal edge and the box (external), and between the
box border and its text (internal).

```rust
use boxy_cli::prelude::*;

// Uniform padding on all sides
let pad = BoxPad::uniform(2);

// Separate vertical and horizontal values
let pad = BoxPad::vh(1, 3); // top/bottom: 1, left/right: 3

// Full control: top, left, down, right
let pad = BoxPad::from_tldr(1, 2, 1, 2);

let mut b = Boxy::new(BoxType::Single, "#00ffff");
b.set_ext_padding(BoxPad::uniform(1));
b.set_int_padding(BoxPad::vh(1, 2));
b.add_text_sgmt("Padded box", "#ffffff", BoxAlign::Center);
b.display();
```

---

## Examples

### Multi-segment box

```rust
use boxy_cli::prelude::*;

fn main() {
    let mut b = Boxy::new(BoxType::Bold, "#00ffff");
    b.add_text_sgmt(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
         incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
         exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
        "#ffffff",
        BoxAlign::Left,
    );
    b.add_text_sgmt(
        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium \
         doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore \
         veritatis et quasi architecto beatae vitae dicta sunt explicabo.",
        "#ffffff",
        BoxAlign::Left,
    );
    b.add_text_sgmt("Hello There", "#ffffff", BoxAlign::Center);
    b.display();
}
```

---

## API Overview

| Method | Description |
|--------|-------------|
| `Boxy::new(type, color)` | Create a new box |
| `Boxy::builder()` | Start a builder chain |
| `add_text_sgmt(text, color, align)` | Add a plain text segment |
| `add_text_line(text, color)` | Add a line to the last segment |
| `add_text_line_indx(text, color, idx)` | Add a line to a specific segment |
| `add_col_text_sgmt(align, count)` | Add a columnar segment |
| `add_col_text_line(text, color, col)` | Add a line to a column in the last segment |
| `add_col_text_line_indx(text, color, seg, col)` | Add a line to a specific column in a specific segment |
| `set_segment_ratios(seg, ratios)` | Set column width ratios for a columnar segment |
| `set_align(align)` | Set box alignment within the terminal |
| `set_int_padding(pad)` | Set internal padding |
| `set_ext_padding(pad)` | Set external padding |
| `set_width(n)` | Fix the box width |
| `set_type(type)` | Change border style |
| `set_color(color)` | Change border color |
| `display()` | Render and print the box |

For the full API reference see [docs.rs/boxy-cli](https://docs.rs/boxy-cli/latest/).
