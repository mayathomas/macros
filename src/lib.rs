// proc macro crate

use proc_macro::TokenStream;
use quote::quote;
// for enum, we'd like to genenrate from impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // get the ident
    let ident = input.ident;
    let generics = input.generics;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics{
                            fn from(v: #ty) -> Self{
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        }
    });
    //quote return proc-macro2 TokenStream, so we need to convert it to TokenStream
    quote! {
        #(#from_impls)*
    }
    .into()
}
