//! Deprecated: import from the crate root instead, e.g. `use boxy_cli::{Boxy, BoxType};`.

#![deprecated(since = "2.3.0", note = "import from the crate root: `use boxy_cli::{Boxy, BoxType, ...}`")]

pub use crate::boxy;
pub use crate::{BoxAlign, BoxPad, BoxType, Boxy, BoxyBuilder};
