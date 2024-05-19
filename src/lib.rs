// proc mac crate

use proc_macro::TokenStream;
use quote::quote;

// for enum, we would like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("input: {:#?}", input);
    // get the ident
    let ident = input.ident;
    // get the generics
    let generics = input.generics;
    // get enum variant
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("Only enum is supported"),
    };
    // for each variant in the variants, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl  #generics From<#ty> for #ident #generics {
                            fn from(val : #ty) -> Self {
                                #ident::#var(val)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        }
    });

    (quote! {
        #(#from_impls)*
    })
    .into()
}
