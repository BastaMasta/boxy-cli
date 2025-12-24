use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(BoxyConstruct)]
pub fn boxy_construct(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    box_build(&input)
}

fn box_build(input: &DeriveInput) -> TokenStream {
     // Extract the struct name
    let name = &input.ident;

    // Count the number of fields if the input is a struct
    let count = if let syn::Data::Struct(data) = &input.data {
        data.fields.iter().count()
    } else {
        0 // Only structs are supported in this example
    };

    // Generate the method to count fields
    let expanded = quote! {
        impl #name {
            pub fn field_count() -> usize {
                #count
            }
        }
    };

    // Convert the generated code into a TokenStream
    TokenStream::from(expanded)
}