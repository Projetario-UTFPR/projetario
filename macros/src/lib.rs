use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, PatType, Type, TypePath, TypeReference, parse_macro_input};

#[proc_macro_attribute]
pub fn with_db_conn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;

    let inner_name = format_ident!("__{}_inner", fn_name);

    let inputs = &sig.inputs;

    let mut call_args = Vec::new();

    for arg in inputs.iter() {
        if let FnArg::Typed(pat_type) = arg {
            let pat = &pat_type.pat;

            let is_pgpool = if let Type::Reference(TypeReference { elem, .. }) = &*pat_type.ty {
                if let Type::Path(TypePath { path, .. }) = &**elem {
                    path.segments.last().unwrap().ident == "PgPool"
                } else {
                    false
                }
            } else {
                false
            };

            if is_pgpool {
                call_args.push(quote! { &__real_pool });
            } else {
                call_args.push(quote! { #pat });
            }
        }
    }

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            use futures_util::FutureExt;

            let schema = crate::common::utils::esquema_db::obtenha_esquema_unico_do_db().leak();
            let db_guard = crate::common::fixtures::db_guard::DBGuard::novo(schema).await;

            let __real_pool = db_guard.as_ref();

            let __result = std::panic::AssertUnwindSafe(async {
                #inner_name(#(#call_args),*).await
            })
            .catch_unwind()
            .await;

            db_guard.cleanup().await;

            if let Err(panic) = __result {
                std::panic::resume_unwind(panic);
            }
        }

        async fn #inner_name(#inputs) {
            #block
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn _with_db_conn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse a função original
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = input.sig.ident.clone();
    let fn_vis = input.vis.clone();
    let fn_attrs = input.attrs.clone();
    let fn_generics = input.sig.generics.clone();
    let fn_async = input.sig.asyncness;
    let fn_output = input.sig.output.clone();
    let fn_body = input.block;

    // Identificador para a função interna
    let inner_fn_name = format_ident!("__{}_inner", fn_name);

    // Itera nos argumentos e separa:
    // - os que não são &PgPool -> para a função externa
    // - os que são &PgPool -> será injetado
    let mut external_args = Vec::new();
    let mut internal_args = Vec::new();
    for arg in input.sig.inputs.iter() {
        match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                if let Type::Reference(TypeReference { elem, .. }) = &**ty {
                    if let Type::Path(path) = &**elem {
                        if path.path.segments.last().unwrap().ident == "PgPool" {
                            // é &PgPool -> será injetado
                            internal_args.push(arg.clone());
                            continue;
                        }
                    }
                }
                // qualquer outro -> mantemos no wrapper
                external_args.push(arg.clone());
                internal_args.push(arg.clone());
            }
            _ => {
                external_args.push(arg.clone());
                internal_args.push(arg.clone());
            }
        }
    }

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_async fn #fn_name #fn_generics (#(#external_args),*) #fn_output {
            // cria pool dinamicamente
            let schema = crate::common::utils::esquema_db::obtenha_esquema_unico_do_db().leak();
            let db_guard = crate::common::fixtures::db_guard::DBGuard::novo(schema).await;
            let db_pool: &PgPool = db_guard.as_ref();

            #inner_fn_name(#(#internal_args.iter().map(|arg| {
                // substitui o tipo &PgPool pelo pool real
                quote! { db_pool }
            }).collect::<Vec<_>>()),*).await
        }

        async fn #inner_fn_name #fn_generics (#(#internal_args),*) #fn_output #fn_body
    };

    TokenStream::from(expanded)
}
