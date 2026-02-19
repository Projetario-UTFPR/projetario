use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, Type, TypePath, TypeReference, parse_macro_input};

/// Injeta uma referência de `PgPool` obtível via parâmetros da função.
/// Requer o uso do atributo `with_setup`. Requer que o teste seja assíncrono.
/// Se utilizado com `rstest`, deve ser posicionado após os atributos do `rstest`
/// e o parâmetro deve utilizar o atributo `#[ignore]` do `rstest`.
///
/// # Exemplo
/// ```rs
/// #[with_setup]
/// #[rstest]
/// #[case(...)]
/// ...
/// #[case(...)]
/// #[awt]
/// #[with_db_conn]
/// #[tokio::test]
/// async fn meu_teste_complexo(
///     #[ignore] db_conn: &PgPool,
/// ) {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn with_db_conn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;
    let generics = &sig.generics;
    let output = &sig.output;

    let inner_name = format_ident!("__{}_inner_with_db_conn", fn_name);

    let inputs = &sig.inputs;

    let mut call_args = Vec::new();
    let mut wrapper_inputs = syn::punctuated::Punctuated::<FnArg, syn::token::Comma>::new();

    for arg in inputs.iter() {
        if let FnArg::Typed(pat_type) = arg {
            let pat = &pat_type.pat;

            let is_pgpool = if let Type::Reference(TypeReference { elem, .. }) = &*pat_type.ty {
                if let Type::Path(TypePath { path, .. }) = &**elem {
                    path.segments
                        .last()
                        .map(|s| s.ident == "PgPool")
                        .unwrap_or(false)
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
                wrapper_inputs.push(arg.clone());
            }
        } else {
            wrapper_inputs.push(arg.clone());
        }
    }

    let expanded = quote! {
        #(#attrs)*
        #vis async fn #fn_name #generics (#wrapper_inputs) #output {
            use futures_util::FutureExt;

            crate::common::setup::setup();
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

        async fn #inner_name #generics (#inputs) #output {
            #block
        }
    };

    expanded.into()
}

/// Executa a função de [`setup`](crate::common::setup::setup), inicializando as configurações.
/// Deve ser posicionado antes de todos os atributos relacionados a testes.
#[proc_macro_attribute]
pub fn with_setup(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let block = &input.block;
    let fn_name = &sig.ident;
    let inputs = &sig.inputs;
    let output = &sig.output;

    let inner_name = format_ident!("__{}_inner_with_setup", fn_name);

    let arg_names = inputs.iter().map(|arg| match arg {
        syn::FnArg::Typed(pat_type) => {
            let pat = &pat_type.pat;
            quote! { #pat }
        }
        syn::FnArg::Receiver(_) => quote! { self },
    });

    let clean_inputs = inputs.iter().map(|arg| {
        if let syn::FnArg::Typed(pt) = arg {
            let mut new_pt = pt.clone();
            new_pt.attrs.clear(); // Remove #[case], #[ignore], etc.
            syn::FnArg::Typed(new_pt)
        } else {
            arg.clone()
        }
    });

    let expanded = quote! {
        #(#attrs)*
        #vis async fn #fn_name(#inputs) #output {
            crate::common::setup::setup();
            #inner_name(#(#arg_names),*).await
        }

        async fn #inner_name(#(#clean_inputs),*) #output {
            #block
        }
    };

    expanded.into()
}
