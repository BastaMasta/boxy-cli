// boxy macro
#[macro_export]
macro_rules! boxy {
    ($($key:ident: $value:expr),* $(,)?) => {{
        let mut boxy = Boxy::default();
        $(
            match stringify!($key) {
                "type" => boxy.type_enum = resolve_type($value.to_string()),
                "color" => boxy.box_col = resolve_col($value.to_string()),
                "internal_pad" => boxy.int_padding = resolve_pad($value.to_string()),
                "external_pad" => boxy.ext_padding = resolve_pad($value.to_string()),
                "alignment" => boxy.align = resolve_align($value.to_string()),
                _ => panic!("Unknown field: {}", stringify!($key)),
            }
        )*
        boxy
    }};
}