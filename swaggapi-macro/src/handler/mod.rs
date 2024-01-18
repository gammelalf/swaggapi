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
        #[allow(non_camel_case_types)]
        #vis struct #func_ident;
        mod #module_ident {
            use super::*;
            #tokens
        }
        impl ::swaggapi::handler::Handler for #func_ident {
            fn method(&self) -> ::swaggapi::Method {
                ::swaggapi::Method::#method
            }
            fn path(&self) -> &'static str {
                #path
            }
            fn ctx_path(&self) -> &'static str {
                #ctx_path
            }
            fn description(&self, gen: &mut ::swaggapi::re_exports::schemars::gen::SchemaGenerator) -> ::swaggapi::OperationDescription {
                let mut parameters = Vec::new();
                let mut request_body = Vec::new();

                #(
                    parameters.extend(
                        <#argument_type as ::swaggapi::handler_argument::HandlerArgument>::parameters(gen)
                    );
                    request_body.extend(
                        <#argument_type as ::swaggapi::handler_argument::HandlerArgument>::request_body(gen)
                    );
                )*

                ::swaggapi::OperationDescription {
                    deprecated: #deprecated,
                    doc: &[#(
                        #doc,
                    )*],
                    ident: #ident,
                    responses: <#return_type as ::swaggapi::as_responses::AsResponses>::responses(gen),
                    request_body,
                    parameters,
                }
            }
            ::swaggapi::impl_Foo_actix!(
                #module_ident::#func_ident: fn(#(#argument_type),*) -> #return_type
            );
            ::swaggapi::impl_Foo_axum!(
                #module_ident::#func_ident: fn(#(#argument_type),*) -> #return_type
            );
        }
    }
}
