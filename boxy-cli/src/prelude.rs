//! Convenient imports for working with Boxy CLI.
//!
//! This module re-exports all the essential types and functions that are commonly used when
//! working with the `boxy-cli` library. Import this module to get everything you need at once.
//!
//! # Example
//!
//! ```
//! use boxy_cli::prelude::*;
//!
//! let mut my_box = Boxy::new(BoxType::Double, "#00ffff");
//! my_box.add_text_sgmt("Hello, World!", "#ffffff", BoxAlign::Center);
//! ```

pub use crate::boxer::*;
pub use crate::boxy;
pub use crate::constructs::{BoxAlign, BoxPad, BoxType};
