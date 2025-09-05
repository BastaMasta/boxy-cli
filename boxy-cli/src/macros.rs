//! The boxy! macro

/// Macro for creating a new Boxy struct
///
/// Currently, it has the following accepting fields:
///
///  - **type** - takes a [BoxType](crate::constructs::BoxType) enum
///
///  - **color** - takes a hex code for a color
///
///  - **external_pad** and **internal-pad** - take any integer or float value
///
///  - **alignment** - sets the alignment for the text inside the box. Takes a [BoxAlign](crate::constructs::BoxAlign) enum
///
///  - **segcount** - sets the number of segments in the textbox (not necessary to use)
///
/// # Example
/// ```
/// # use boxy_cli::prelude::*;
/// # fn main() {
/// // use the boxy macro
/// let mut boxy = boxy!(type: BoxType::Double, color:"#00ffff", external_pad: 2, internal_pad: 1, alignment: BoxAlign::Left, segcount: 3);
///
/// // Adding text segments
/// boxy.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit.", "#ffff", BoxAlign::Center);
/// boxy.add_text_sgmt("et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff", BoxAlign::Left);
/// boxy.add_text_sgmt("Hello Theree", "#ffff", BoxAlign::Right);
/// boxy.display();
/// # }
/// ```
/// ! The segcount sets the number of segments in the box. If text for only two segments is provided, the third segment will be displayed empty.
/// 
/// ! The padding values here are taken to be for uniform padding on all sides.
#[macro_export]
macro_rules! boxy {
    ($($key:ident: $value:expr),* $(,)?) => {{
        let mut __builder = Boxy::builder();
        let mut __segcount_opt: Option<usize> = None;
        $(
            match stringify!($key) {
                "type" => { __builder = __builder.box_type(resolve_type($value.to_string())); },
                "color" => { let c = resolve_col($value.to_string()); __builder = __builder.color(&c); },
                "internal_pad" => { __builder = __builder.internal_padding(resolve_pad($value.to_string())); },
                "external_pad" => { __builder = __builder.external_padding(resolve_pad($value.to_string())); },
                "alignment" => { __builder = __builder.align(resolve_align($value.to_string())); },
                "segcount" => { __segcount_opt = Some(resolve_segments($value.to_string())); },
                _ => panic!("Unknown field: {}", stringify!($key)),
            }
        )*
        let mut boxy = __builder.build();
        if let Some(n) = __segcount_opt { boxy.__macro_set_segcount(n); }
        boxy
    }};
}