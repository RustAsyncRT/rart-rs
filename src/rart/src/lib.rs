mod entry_macro;

use crate::entry_macro::{entry_parse, entry_analyze, entry_codegen};

use proc_macro::TokenStream;
use proc_macro::TokenTree;
use quote::{quote};

fn get_lazy_data(args: TokenStream) -> TokenStream {
    let mut values = vec![];
    for arg in args {
        if let TokenTree::Ident(ident) = &arg {
            values.push(ident.to_string());
        }
    }

    if values.len() != 1 {
        panic!("Wrong number of arguments");
    }

    let name = syn::Ident::new(
        &values[0].to_uppercase(),
        proc_macro2::Span::call_site(),
    );

    let rust = quote! {
        #name.data()
    };
    rust.into()
}

#[proc_macro]
pub fn channel_def(args: TokenStream) -> TokenStream {
    let mut values = vec![];
    for arg in args {
        if let TokenTree::Ident(ident) = &arg {
            values.push(ident.to_string());
        } else if let TokenTree::Literal(lit) = &arg {
            values.push(lit.to_string());
        }
    }

    if values.len() != 3 {
        panic!("Wrong number of arguments");
    }

    let name = syn::Ident::new(
        &values[0].to_uppercase(),
        proc_macro2::Span::call_site(),
    );
    let ty = syn::Ident::new(
        &values[1],
        proc_macro2::Span::call_site(),
    );
    let size = syn::LitInt::new(
        &values[2],
        proc_macro2::Span::call_site(),
    );

    let rust = quote! {
        static #name: Lazy<Channel<#ty, #size, __TASK_NUMBER>> = Lazy::new();
    };
    rust.into()
}

#[proc_macro]
pub fn channel_pub_def(args: TokenStream) -> TokenStream {
    let mut values = vec![];
    for arg in args {
        if let TokenTree::Ident(ident) = &arg {
            values.push(ident.to_string());
        } else if let TokenTree::Literal(lit) = &arg {
            values.push(lit.to_string());
        }
    }

    if values.len() != 3 {
        panic!("Wrong number of arguments");
    }

    let name = syn::Ident::new(
        &values[0].to_uppercase(),
        proc_macro2::Span::call_site(),
    );
    let ty = syn::Ident::new(
        &values[1],
        proc_macro2::Span::call_site(),
    );
    let size = syn::LitInt::new(
        &values[2],
        proc_macro2::Span::call_site(),
    );

    let rust = quote! {
        pub static #name: Lazy<Channel<#ty, #size, __TASK_NUMBER>> = Lazy::new();
    };
    rust.into()
}

#[proc_macro]
pub fn channel(args: TokenStream) -> TokenStream {
    get_lazy_data(args)
}

#[proc_macro]
pub fn mutex_def(args: TokenStream) -> TokenStream {
    let mut values = vec![];
    for arg in args {
        if let TokenTree::Ident(ident) = &arg {
            values.push(ident.to_string());
        } else if let TokenTree::Literal(lit) = &arg {
            values.push(lit.to_string());
        }
    }

    if values.len() != 2 {
        panic!("Wrong number of arguments");
    }

    let name = syn::Ident::new(
        &values[0].to_uppercase(),
        proc_macro2::Span::call_site(),
    );
    let ty = syn::Ident::new(
        &values[1],
        proc_macro2::Span::call_site(),
    );

    let rust = quote! {
        static #name: Lazy<Mutex<#ty, __TASK_NUMBER>> = Lazy::new();
    };
    rust.into()
}

#[proc_macro]
pub fn mutex(args: TokenStream) -> TokenStream {
    get_lazy_data(args)
}

#[proc_macro]
pub fn semaphore_def(args: TokenStream) -> TokenStream {
    let mut values = vec![];
    for arg in args {
        if let TokenTree::Ident(ident) = &arg {
            values.push(ident.to_string());
        } else if let TokenTree::Literal(lit) = &arg {
            values.push(lit.to_string());
        }
    }

    if values.len() != 2 {
        panic!("Wrong number of arguments");
    }

    let name = syn::Ident::new(
        &values[0].to_uppercase(),
        proc_macro2::Span::call_site(),
    );
    let number = syn::LitInt::new(
        &values[1],
        proc_macro2::Span::call_site(),
    );

    let rust = quote! {
        static #name: Lazy<Semaphore<#number, __TASK_NUMBER>> = Lazy::new();
    };
    rust.into()
}

#[proc_macro]
pub fn semaphore(args: TokenStream) -> TokenStream {
    get_lazy_data(args)
}

#[proc_macro]
pub fn trigger_def(args: TokenStream) -> TokenStream {
    let mut values = vec![];
    for arg in args {
        if let TokenTree::Ident(ident) = &arg {
            values.push(ident.to_string());
        } else if let TokenTree::Literal(lit) = &arg {
            values.push(lit.to_string());
        }
    }

    if values.len() != 1 {
        panic!("Wrong number of arguments");
    }

    let name = syn::Ident::new(
        &values[0].to_uppercase(),
        proc_macro2::Span::call_site(),
    );

    let rust = quote! {
        static #name: Lazy<Trigger<__TASK_NUMBER>> = Lazy::new();
    };
    rust.into()
}

#[proc_macro]
pub fn trigger(args: TokenStream) -> TokenStream {
    get_lazy_data(args)
}

#[proc_macro_attribute]
pub fn entry(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = entry_parse(args.into(), item.into());
    let model = entry_analyze(ast);
    let rust = entry_codegen(model);
    rust.into()
}
