# boxy-cli
[![Static Badge](https://img.shields.io/badge/GitHub-BastaMasta%2Fboxy--cli-blue?style=flat-square&logo=github)](https://github.com/BastaMasta/boxy-cli)
[![Crates.io](https://img.shields.io/crates/v/boxy-cli?style=flat-square&logo=rust)](https://crates.io/crates/boxy-cli)
[![Docs.rs](https://img.shields.io/badge/docs.rs-boxy--cli-66c2a5?style=flat-square&logo=docs.rs)](https://docs.rs/boxy-cli/latest/)
[![Crates.io](https://img.shields.io/crates/d/boxy-cli?style=flat-square)](https://crates.io/crates/boxy-cli)
[![Static Badge](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-APACHE)
[![Static Badge](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/BastaMasta/boxy-cli/rust.yml?branch=main&style=flat-square)](https://github.com/BastaMasta/boxy-cli/actions/workflows/rust.yml?query=branch%3Amain)


**A Crate to create boxes in command-line interfaces with Rust**

Dual-licensed under [Apache 2.0](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-APACHE) or [MIT](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-MIT).

## About

**boxy-cli** is a Rust crate that makes it easy to create stylish text boxes in terminal applications. With a simple, intuitive API, you can quickly add visually appealing elements to your CLI applications.

## Features

- **Multiple Border Styles**: Choose from single, double, bold, rounded, and other border styles
- **Color Support**: Customize border and text colors using hex color codes
- **Flexible Layouts**: Create multi-segment boxes with horizontal dividers
- **Text Alignment**: Align text left, center, or right within each segment
- **Custom Padding**: Control spacing both inside and outside the box
- **Terminal-Aware**: Automatically adjusts to terminal width or use fixed dimensions
- **Builder Pattern**: Fluent API for easy box creation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
boxy-cli = "0.1.0"
```

Or use cargo add:
```bash
cargo add boxy-cli
```

### How to use:

### Using the Builder Pattern

The builder pattern provides a fluent, chainable API for creating and configuring text boxes:

```rust
use boxy_cli::prelude::*;
```

Next, you can create the BoxyBuilder struct

```rust
let mut my_box = Boxy::builder()
    .box_type(BoxType::Double)       // Set border style
    .color("#00ffff")               // Set border color
    .padding(
        BoxPad::uniform(1),         // External padding
        BoxPad::from_tldr(2, 2, 1, 1) // Internal padding
    )
    .align(BoxAlign::Center)         // Center the box in the terminal
    .add_segment("Hello, Boxy!", "#ffffff", BoxAlign::Center)
    .add_line("This is a new line.", "#32CD32")
    .add_segment("Another section", "#663399", BoxAlign::Left)
    .width(50)                      // Set fixed width
    .build();
```

and now, display it:

```rust
my_box.display();
```

You can also build and display in one go:

```rust
Boxy::builder()
    .box_type(BoxType::Double)
    .color("#aaffff")
    .padding(BoxPad::uniform(1), BoxPad::from_tldr(2, 2, 1, 1))
    .align(BoxAlign::Center)
    .add_segment("Hello, Boxy!", "#ffffff", BoxAlign::Center)
    .add_line("This is a new line.", "#32CD32")
    .add_segment("Another section", "#f19356", BoxAlign::Right)
    .width(50)
    .build()
    .display();
```

further, you can use the same methods as displayed above to modify the textbox before building.

But you can also modify the textbox after building it (before displaying) using the methods shown in the following section.

#### Using the Struct and methods.

First, import the crate into the current scope, using:

```rust
use boxy_cli::prelude::*;
```

Next you create a new boxy struct with either the ```new``` method:

```rust
let mut box1 = Boxy::new(BoxType::Double,"#00ffff");
```
or the macro:

```rust
let mut box2 = boxy!(type: BoxType::Double, color:"#00ffff");
```
*for more info on the macro, view the [macro documentation](https://docs.rs/boxy-cli/0.1.0/boxy_cli/macro.boxy.html)*

Next, we just add in text sections:
```rust
box1.add_text_sgmt("Lorem ipsum dolor sit amet", "#fffff", BoxAlign::Center);
```
Add some more text to the same segment (or the latest segment):
```rust
box1.add_text_line("consectetur adipiscing elit", "#32CD32");
```
or to a segment with a particular index:
```rust
box1.add_text_line_indx(" consectetur adipiscing elit", "#32CD32", 0);
```
Once you are done, display the TextBox:
```rust
box1.display();
```

### Using the Macro

You can also use the `boxy!` macro for quick box creation:


## Examples:

### Textbox 1

```rust
use boxy_cli::prelude::*;

fn main() {
    let mut box1 = Boxy::new(BoxType::Double, "#bfff00");

    // Add multiple text segments
    box1.add_text_sgmt(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.", 
        "#ffffff", 
        BoxAlign::Left
    );

    box1.add_text_sgmt(
        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", 
        "#ffffff", 
        BoxAlign::Left
    );

    box1.add_text_sgmt("Hello There", "#ffffff", BoxAlign::Left);

    // Display the box
    box1.display();
}
```
### Output:
![First textbox, lime green, double borders](readme-assets/textbox1.jpg)

### Textbox 2:

```rust
use boxy_cli::prelude::*;

fn main() {
    let mut box1 = Boxy::new(BoxType::Bold, "#00ffff");

    // Add multiple text segments with auto text wrapping
    box1.add_text_sgmt(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.", 
        "#ffffff", 
        BoxAlign::Left
    );

    box1.add_text_sgmt(
        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", 
        "#ffffff", 
        BoxAlign::Left
    );

    box1.add_text_sgmt("Hello There", "#ffffff", BoxAlign::Left);

    // Display the box
    box1.display();
}
```

### Output (Wider terminal):
![Second textbox, Cyan Blue, Bold borders](readme-assets/textbox2.jpg)



