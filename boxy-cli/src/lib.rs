#![allow(dead_code)]
//! # Boxy CLI
//!
//! A Rust library for creating beautifully styled text boxes in command-line interfaces.
//!
//! `boxy-cli` provides an easy way to create and display text boxes with various [border styles](./constructs/enum.BoxType.html),
//! [colors](https://docs.rs/colored/latest/colored/), and [alignment](./constructs/enum.BoxAlign.html) options. It's perfect for creating eye-catching CLI applications,
//! status displays, or simply organizing text output in a visually appealing way.
//!
//! ## Features
//!
//! - Multiple [border styles](./constructs/enum.BoxType.html) (single, double, bold, rounded, etc.)
//! - Colored borders and text using [hex color codes](https://docs.rs/colored/latest/colored/)
//! - Customizable [internal and external padding](./constructs/struct.BoxPad.html)
//! - Text [alignment](./constructs/enum.BoxAlign.html) options (left, center, right)
//! - Support for [multiple text segments](./boxer/struct.Boxy.html#method.add_text_sgmt) with dividers
//! - Builder pattern for fluent API usage
//! - Dynamic sizing based on terminal dimensions
//!
//! ## Quick Start
//!
//! ```rust
//! use boxy_cli::prelude::*;
//!
//! fn main() {
//!     // Create a text box with fluent builder API
//!     Boxy::builder()
//!         .box_type(BoxType::Double)
//!         .color("#00ffff")
//!         .add_segment("Hello, Boxy!", "#ffffff", BoxAlign::Center)
//!         .add_segment("A beautiful CLI box library", "#32CD32", BoxAlign::Center)
//!         .padding(BoxPad::uniform(1), BoxPad::vh(1, 2))
//!         .build()
//!         .display();
//! }
//! ```
//!
//! # Detailed docs
//! See the [Boxy](./boxer/struct.Boxy.html) struct or the [BoxyBuilder](./boxer/struct.BoxyBuilder.html) struct for more information.
//!
//! ## Usage Examples
//!
//! See the [README](https://github.com/BastaMasta/boxy-cli) for more examples and usage information.

pub mod boxer;
pub mod templates;
pub mod macros;
pub mod constructs;
pub mod prelude;

// Re-export prelude at crate root
pub use prelude::*;