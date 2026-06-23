#![warn(missing_docs)]

//! # boxy-cli
//!
//! A Rust library for creating beautifully styled, multi-segment text boxes in terminal
//! applications — with full Unicode border support, true-color text, word-wrapping, columnar
//! layouts, and automatic terminal-width awareness.
//!
//! ## Features
//!
//! - **9 border styles** — classic ASCII, single, double, bold, rounded, and more
//! - **True-color support** — hex color codes for both borders and per-line text
//! - **Multi-segment boxes** — stack multiple sections separated by smart dividers
//! - **Columnar layouts** — side-by-side columns inside a single box, with per-segment
//!   ratio control and correct `┼`/`┬`/`┴` junction characters where column boundaries meet
//! - **Word wrapping** — automatic wrapping to terminal width with internal padding awareness
//! - **Text alignment** — left, center, or right per segment
//! - **Terminal-aware sizing** — auto-sizes to terminal width, or use a fixed width
//! - **Two APIs** — imperative ([`Boxy`]) and fluent builder
//!   ([`BoxyBuilder`])
//! - **Macro support** — [`boxy!`] for quick one-liner (Work in Progress)
//!
//! ## Known Limitations
//!
//! **Unicode wide characters** — characters that occupy two terminal columns (CJK
//! glyphs, most emoji) are measured as one column internally. Text containing these
//! characters will appear narrower than expected and centering/alignment will be off.
//! Full wide-character support is planned for a future release.
//!
//! ## Quick Start
//!
//! ```rust
//! use boxy_cli::prelude::*;
//!
//! // Fluent builder API
//! Boxy::builder()
//!     .box_type(BoxType::Double)
//!     .color("#00ffff")
//!     .add_segment("Hello, boxy-cli!", "#ffffff", BoxAlign::Center)
//!     .add_segment("A terminal box library for Rust", "#32CD32", BoxAlign::Center)
//!     .padding(BoxPad::uniform(1), BoxPad::vh(1, 2))
//!     .build()
//!     .display();
//! ```
//!
//! ## Columnar Layout
//!
//! Columnar segments let you place content side-by-side inside one box. Column widths are
//! controlled by ratio values — `vec![1, 2, 1]` gives the middle column twice the space.
//!
//! ```rust
//! use boxy_cli::prelude::*;
//!
//! let mut b = Boxy::new(BoxType::Single, "#00ffff");
//! b.add_text_sgmt("Project Status", "#ffffff", BoxAlign::Center);
//! b.add_col_text_sgmt(BoxAlign::Left, 3);
//! b.add_col_text_line("Name",     "#aaaaaa", &0usize);
//! b.add_col_text_line("Status",   "#aaaaaa", &1usize);
//! b.add_col_text_line("Notes",    "#aaaaaa", &2usize);
//! b.add_col_text_line("Lumio V2", "#ffffff", &0usize);
//! b.add_col_text_line("Shipped",  "#32CD32", &1usize);
//! b.add_col_text_line("Internship project", "#ffffff", &2usize);
//! b.set_segment_ratios(1, vec![1, 1, 2]);
//! b.display();
//! ```
//!
//! ## Further Reading
//!
//! - [`Boxy`] — imperative API reference
//! - [`BoxyBuilder`] — builder API reference
//! - [`BoxType`] — all available border styles
//! - [`boxy!`] — macro reference
//! - [GitHub README](https://github.com/BastaMasta/boxy-cli) — more examples and screenshots

#[allow(dead_code)]
pub mod boxer;
mod constructs;
mod macros;
pub mod prelude;
pub(crate) mod templates;

// crate tests
mod tests;

// Re-export prelude at crate root
pub use prelude::*;
