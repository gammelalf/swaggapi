use proc_macro2::{Ident, Literal, TokenStream};
use proc_macro2::{Span, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::{FnArg, ItemFn, Meta, MetaNameValue, ReturnType};

mod parse;

pub fn handler(
    args: TokenStream,
    tokens: TokenStream,
    method: Option<&'static str>,
) -> TokenStream {
    let (
        parse::Args {
            positional,
            mut keyword,
        },
        ItemFn {
            attrs,
            vis,
            sig,
            block: _,
        },
    ) = match parse::parse(args, tokens.clone()) {
        Ok(x) => x,
        Err(err) => {
            return quote! {
                #err
                #tokens
            }
        }
    };

    let mut positional = positional.into_iter();
    let method = method
        .map(|str| TokenTree::Ident(Ident::new(str, Span::call_site())))
        .or_else(|| keyword.remove(&Ident::new("method", Span::call_site())))
        .or_else(|| positional.next())
        .unwrap();
    let ctx_path = keyword
        .remove(&Ident::new("context_path", Span::call_site()))
        .or_else(|| positional.next())
        .unwrap();
    let path = keyword
        .remove(&Ident::new("path", Span::call_site()))
        .or_else(|| positional.next())
        .unwrap();

    let func_ident = &sig.ident;
    let module_ident = format_ident!("__{}_module", sig.ident);
    let argument_type = sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(arg) => Some(&arg.ty),
        })
        .collect::<Vec<_>>();
    let return_type = match sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, return_type) => return_type.into_token_stream(),
    };

    let ident = Literal::string(&sig.ident.to_string());
    let deprecated = attrs.iter().any(|attr| {
        attr.meta
            .path()
            .get_ident()
            .map(|ident| ident == "deprecated")
            .unwrap_or(false)
    });
    let deprecated = if deprecated {
        format_ident!("true")
    } else {
        format_ident!("false")
    };
    let doc = attrs.iter().filter_map(|attr| match &attr.meta {
        Meta::NameValue(MetaNameValue {
            path,
            eq_token: _,
            value,
        }) => {
            if path.get_ident()? != "doc" {
                None
            } else {
                Some(value)
            }
        }
        _ => None,
    });
    quote! {
        mod #module_ident {
            use super::*;
            #tokens
        }
        #[allow(non_upper_case_globals)]
        #vis static #func_ident: ::swaggapi::handler::Handler =  {
            const N: usize =  0 #(+ {let _ = stringify!(#argument_type); 1})*;
            static FNS: [::std::option::Option<::swaggapi::handler_argument::HandlerArgumentFns>; N] = [#(
                ::swaggapi::handler_argument::macro_helper::get_handler_argument_fns(
                    || ::swaggapi::handler_argument::macro_helper::TraitProbe::<#argument_type>::new().get_handler_argument(),
                    || ::swaggapi::handler_argument::macro_helper::TraitProbe::<#argument_type>::new().is_handler_argument(),
                )
            )*];
            const _: () = {#(
                ::swaggapi::handler_argument::macro_helper::check_handler_argument(
                    || ::swaggapi::handler_argument::macro_helper::TraitProbe::<#argument_type>::new().get_handler_argument()
                );
            )*};

            ::swaggapi::handler::Handler {
                method: ::swaggapi::Method::#method,
                path: #path,
                ctx_path: #ctx_path,
                deprecated: #deprecated,
                doc: &[#(
                    #doc,
                )*],
                ident: #ident,
                responses: <#return_type as ::swaggapi::as_responses::AsResponses>::responses,
                handler_arguments: &FNS,
                actix: ::swaggapi::impl_Foo_actix!(
                    #module_ident::#func_ident: fn(#(#argument_type),*) -> #return_type
                ),
                axum: ::swaggapi::impl_Foo_axum!(
                    #module_ident::#func_ident: fn(#(#argument_type),*) -> #return_type
                ),
            }

        };
    }
}
