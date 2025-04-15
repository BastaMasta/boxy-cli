# boxy-cli


[![Static Badge](https://img.shields.io/badge/GitHub-BastaMasta%2Fboxy--cli-blue?style=flat-square&logo=github)](https://github.com/BastaMasta/boxy-cli)
[![Crates.io](https://img.shields.io/crates/v/boxy-cli?style=flat-square&logo=rust)](https://crates.io/crates/boxy-cli)
[![Docs.rs](https://img.shields.io/badge/docs.rs-boxy--cli-66c2a5?style=flat-square&logo=docs.rs)](https://docs.rs/boxy-cli/0.1.0/)
[![Crates.io](https://img.shields.io/crates/d/boxy-cli?style=flat-square)](https://crates.io/crates/boxy-cli)
[![Static Badge](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-APACHE)
[![Static Badge](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/BastaMasta/boxy-cli/rust.yml?branch=main&style=flat-square)](https://github.com/BastaMasta/boxy-cli/actions/workflows/rust.yml?query=branch%3Amain)

**A Crate to create boxes in command-line interfaces with Rust**

Dual-licensed under [Apache 2.0](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-APACHE) or [MIT](https://github.com/BastaMasta/boxy-cli/blob/main/LICENSE-MIT).

## About:
**boxy-cli** is a crate to create simple textboxes in command-line interfaces, with a simple and easy-to-use design.

### How to use:

First import the crate into the current scope, using:

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
box1.add_text_sgmt("Lorem ipsum dolor sit amet", "#fffff");
```
Add some more text to the same segment (or the latest segment):
```rust
box1.add_text_line("consectetur adipiscing elit");
```
or to a segment with a particular index:
```rust
box1.add_text_line_indx(" consectetur adipiscing elit", 0);
```
Once you are done, just display the TextBox:
```rust
box1.display();
```

*the text colour is a required argument, and will be implemented into a usable feature in the very near future. But for now, it does not work*

## Example:

### Textbox 1

```rust
use boxy_cli::prelude::*;

fn main() {
    let mut box1 = Boxy::new(BoxType::Double,"#bfff00");
    box1.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur", "#ffff");
    box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff");
    box1.add_text_sgmt("Hello Theree", "#ffff");
    box1.display();
}
```
### Output:
![First textbox, lime green, double borders](readme-assets/textbox1.jpg)

### Textbox 2:

```rust
use boxy_cli::prelude::*;

fn main() {
    let mut box1 = Boxy::new(BoxType::Bold,"#00ffff");
    box1.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur", "#ffff");
    box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff");
    box1.add_text_sgmt("Hello Theree", "#ffff");
    box1.display();
}
```

### Output (Wider terminal):
![Second textbox, Cyan Blue, Bold borders](readme-assets/textbox2.jpg)



