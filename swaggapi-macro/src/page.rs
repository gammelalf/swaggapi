use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::parse2;
use syn::spanned::Spanned;
use syn::Fields;
use syn::ItemStruct;

pub fn page(input: TokenStream) -> TokenStream {
    let ItemStruct {
        attrs,
        vis: _,
        struct_token: _,
        ident,
        generics: _,
        fields,
        semi_token: _,
    } = match parse2(input) {
        Ok(s) => s,
        Err(err) => return err.into_compile_error(),
    };

    if !matches!(&fields, Fields::Unit) {
        return quote_spanned! {fields.span()=>
            compile_error!("Expected unit struct");
        };
    }

    quote! {
        impl ::swaggapi::internals::AccessSwaggapiPageBuilder for #ident {
            fn builder() -> &'static ::swaggapi::SwaggapiPageBuilder {
                static BUILDER: ::swaggapi::SwaggapiPageBuilder = ::swaggapi::SwaggapiPageBuilder::new();
                &BUILDER
            }
        }
    }
}
