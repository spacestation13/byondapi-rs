use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{spanned::Spanned, Lit};

fn extract_args(a: &syn::FnArg) -> &syn::PatType {
    match a {
        syn::FnArg::Typed(p) => p,
        _ => panic!("Not supported on types with `self`!"),
    }
}
//this is an example, mr clippy
#[allow(clippy::test_attr_in_doctest)]
/// Macro for generating byond binds
/// Usage:
/// ```
/// use byondapi::prelude::*;
/// #[byondapi::bind]
/// fn example() {Ok(ByondValue::null())}
///
/// #[byondapi::bind("/datum/example/proc/other_example")]
/// fn example_other(_: ByondValue, _: ByondValue) {Ok(ByondValue::null())}
///
/// ```
/// Then generate the bindings.dm file with
/// ```
/// #[test]
/// fn generate_binds() {
///     byondapi::byondapi_macros::generate_bindings(env!("CARGO_CRATE_NAME"));
/// }
/// ```
/// and run cargo test to actually create the file
///
#[proc_macro_attribute]
pub fn bind(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let proc = syn::parse_macro_input!(attr as Option<syn::Lit>);

    let func_name = &input.sig.ident;
    let func_name_disp = quote!(#func_name).to_string();

    let func_name_ffi = format!("{func_name_disp}_ffi");
    let func_name_ffi = Ident::new(&func_name_ffi, func_name.span());
    let func_name_ffi_disp = quote!(#func_name_ffi).to_string();

    let args = &input.sig.inputs;

    //Check for returns
    let func_return = match &input.sig.output {
        syn::ReturnType::Default => {
            return syn::Error::new(
                input.span(),
                "Empty returns are not allowed, please return a Result",
            )
            .to_compile_error()
            .into()
        }

        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            &syn::Type::Path(_) => &input.sig.output,
            _ => {
                return syn::Error::new(input.span(), "Invalid return type, please return a Result")
                    .to_compile_error()
                    .into()
            }
        },
    };

    let signature = quote! {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn #func_name_ffi (
            __argc: ::byondapi::sys::u4c,
            __argv: *mut ::byondapi::value::ByondValue
        ) -> ::byondapi::value::ByondValue
    };

    let body = &input.block;
    let mut arg_names: syn::punctuated::Punctuated<syn::Ident, syn::Token![,]> =
        syn::punctuated::Punctuated::new();
    let mut proc_arg_unpacker: syn::punctuated::Punctuated<
        proc_macro2::TokenStream,
        syn::Token![,],
    > = syn::punctuated::Punctuated::new();

    for arg in args.iter().map(extract_args) {
        if let syn::Pat::Ident(p) = &*arg.pat {
            arg_names.push(p.ident.clone());
            let index = arg_names.len() - 1;
            proc_arg_unpacker.push(quote! {
                args.get(#index).map(::byondapi::value::ByondValue::clone).unwrap_or_default()
            });
        }
    }

    let arg_names_disp = quote!(#arg_names).to_string();

    //Submit to inventory
    let cthook_prelude = match &proc {
        Some(Lit::Str(p)) => {
            quote! {
                ::byondapi::inventory::submit!({
                    ::byondapi::binds::Bind {
                        proc_path: #p,
                        func_name: #func_name_ffi_disp,
                        func_arguments: #arg_names_disp,
                        is_variadic: false,
                    }
                });
            }
        }
        Some(other_literal) => {
            return syn::Error::new(
                other_literal.span(),
                "Bind attributes must be a string literal or empty",
            )
            .to_compile_error()
            .into()
        }
        None => {
            let mut func_name_disp = func_name_disp.clone();
            func_name_disp.insert_str(0, "/proc/");
            quote! {
                ::byondapi::inventory::submit!({
                    ::byondapi::binds::Bind{
                        proc_path: #func_name_disp,
                        func_name: #func_name_ffi_disp,
                        func_arguments: #arg_names_disp,
                        is_variadic: false,
                    }
                });
            }
        }
    };

    let result = quote! {
        #cthook_prelude
        #signature {
            let args = unsafe { ::byondapi::parse_args(__argc, __argv) };
            match #func_name(#proc_arg_unpacker) {
                Ok(val) => val,
                Err(e) => {
                    let error_string = ::byondapi::value::ByondValue::try_from(::std::format!("{e:?}")).unwrap();
                    ::byondapi::global_call::call_global_id({
                            static STACK_TRACE: ::std::sync::OnceLock<u32> = ::std::sync::OnceLock::new();
                            *STACK_TRACE.get_or_init(|| ::byondapi::byond_string::str_id_of("byondapi_stack_trace")
                                .expect("byondapi-rs implicitly expects byondapi_stack_trace to exist as a proc for error reporting purposes, this proc doesn't exist!")
                            )
                        }
                        ,&[error_string]).unwrap();
                    ::byondapi::value::ByondValue::null()
                }
            }

        }
        fn #func_name(#args) #func_return
        #body
    };
    result.into()
}

/// Same as [`bind`] but accepts variable amount of args, with src in the beginning if there's a src
/// The args are just a variable named `args` in the macro'd function
#[proc_macro_attribute]
pub fn bind_raw_args(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let proc = syn::parse_macro_input!(attr as Option<syn::Lit>);

    let func_name = &input.sig.ident;
    let func_name_disp = quote!(#func_name).to_string();

    let func_name_ffi = format!("{func_name_disp}_ffi");
    let func_name_ffi = Ident::new(&func_name_ffi, func_name.span());
    let func_name_ffi_disp = quote!(#func_name_ffi).to_string();

    //Check for returns
    let func_return = match &input.sig.output {
        syn::ReturnType::Default => {
            return syn::Error::new(
                input.span(),
                "Empty returns are not allowed, please return a Result",
            )
            .to_compile_error()
            .into()
        }

        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            &syn::Type::Path(_) => &input.sig.output,
            _ => {
                return syn::Error::new(input.span(), "Invalid return type, please return a Result")
                    .to_compile_error()
                    .into()
            }
        },
    };

    if !input.sig.inputs.is_empty() {
        return syn::Error::new(
            input.sig.inputs.span(),
            "Do not specify arguments for raw arg binds",
        )
        .to_compile_error()
        .into();
    }

    let signature = quote! {
        #[no_mangle]
        pub unsafe extern "C-unwind" fn #func_name_ffi (
            __argc: ::byondapi::sys::u4c,
            __argv: *mut ::byondapi::value::ByondValue
        ) -> ::byondapi::value::ByondValue
    };

    let body = &input.block;

    //Submit to inventory
    let cthook_prelude = match proc {
        Some(Lit::Str(p)) => {
            quote! {
                ::byondapi::inventory::submit!({
                    ::byondapi::binds::Bind {
                        proc_path: #p,
                        func_name: #func_name_ffi_disp,
                        func_arguments: "",
                        is_variadic: true,
                    }
                });
            }
        }
        Some(other_literal) => {
            return syn::Error::new(
                other_literal.span(),
                "Bind attributes must be a string literal or empty",
            )
            .to_compile_error()
            .into()
        }
        None => {
            let mut func_name_disp = func_name_disp.clone();
            func_name_disp.insert_str(0, "/proc/");
            quote! {
                    ::byondapi::inventory::submit!({
                        ::byondapi::binds::Bind{
                            proc_path: #func_name_disp,
                            func_name: #func_name_ffi_disp,
                            func_arguments: "",
                            is_variadic: true,
                        }
                    });
            }
        }
    };

    let result = quote! {
        #cthook_prelude
        #signature {
            let mut args = unsafe { ::byondapi::parse_args(__argc, __argv) };
            match #func_name(args) {
                Ok(val) => val,
                Err(e) => {
                    let error_string = ::byondapi::value::ByondValue::try_from(::std::format!("{e:?}")).unwrap();
                    ::byondapi::global_call::call_global_id({
                            static STACK_TRACE: ::std::sync::OnceLock<u32> = ::std::sync::OnceLock::new();
                            *STACK_TRACE.get_or_init(|| ::byondapi::byond_string::str_id_of("byondapi_stack_trace")
                                .expect("byondapi-rs implicitly expects byondapi_stack_trace to exist as a proc for error reporting purposes, this proc doesn't exist!")
                            )
                        }
                        ,&[error_string]).unwrap();
                    ::byondapi::value::ByondValue::null()
                }
            }
        }
        fn #func_name(args: &mut [::byondapi::value::ByondValue]) #func_return
        #body
    };
    result.into()
}

#[proc_macro_attribute]
pub fn init(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let func_name = &input.sig.ident;
    quote! {
        #input
        ::byondapi::inventory::submit!({::byondapi::InitFunc(#func_name)});
    }
    .into()
}
