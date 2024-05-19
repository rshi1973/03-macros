use darling::{ast::Data, FromDeriveInput, FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[allow(dead_code)]
#[derive(FromDeriveInput, Debug)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EnumVariants, ()>,
}

#[allow(dead_code)]
#[derive(FromVariant, Debug)]
struct EnumVariants {
    ident: syn::Ident,
    fields: darling::ast::Fields<EnumVariantFields>,
}

#[allow(dead_code)]
#[derive(FromField, Debug)]
struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).unwrap()
    else {
        panic!("Only enum is supported");
    };

    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            darling::ast::Style::Tuple => {
                let field = variant.fields.iter().next().expect("should have 1 field");
                let ty = &field.ty;
                quote! {
                    impl  #generics From<#ty> for #ident #generics {
                        fn from(val : #ty) -> Self {
                            #ident::#var(val)
                        }
                    }
                }
            }
            darling::ast::Style::Unit => quote! {},
            darling::ast::Style::Struct => quote! {},
        }
    });

    println!("ident: {:#?}, generics: {:#?}", ident, generics);
    println!("data: {:#?}", data);

    quote! {
        #(#from_impls)*
    }
}
