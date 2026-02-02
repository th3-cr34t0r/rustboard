use proc_macro::{Ident, TokenStream};
use quote::quote;
// use rustboard_source::Pins;
use syn::{DeriveInput, Expr, parse_macro_input};

#[proc_macro]
pub fn gen_output_pins(input: TokenStream) -> TokenStream {
    // Parse the input tokens
    let pins = parse_macro_input!(input as Ident);

    // let variants = match pins.data {
    //     syn::Data::Enum(ref data_enum) => &data_enum.variants,
    //     _ => panic!("generate_outputs macro can only be used on enums!"),
    // };

    let ident = pins.ident;

    let generated_code =
     // variants.iter().map(|variant| {
     //    let ident = &variant.ident;

        quote! {
            Pins::#ident => Output::new(p.#ident, Level::Low, OutputDrive::Standard),
        };
    // });

    // let expanded = quote! {match pin {
    //     #(#generated_code)*
    //     _ => unreachable!().
    // }};

    // TokenStream::from(expanded)
    TokenStream::from(generated_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = generate_pin_config([Pins::P0_00, Pins::P0_01]);
        let test_against = [
            Output::new(p.P0_00, Level::Low, OutputDrive::Standard),
            Output::new(p.P0_01, Level::Low, OutputDrive::Standard),
        ];
        assert_eq!(result, 4);
    }
}
