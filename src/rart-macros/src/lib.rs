mod entry_macro;

use crate::entry_macro::{entry_parse, entry_analyze, entry_codegen};

use proc_macro::TokenStream;
use proc_macro::TokenTree;
use quote::quote;

#[proc_macro_attribute]
pub fn entry(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = entry_parse(args.into(), item.into());
    let model = entry_analyze(ast);
    let rust = entry_codegen(model);
    rust.into()
}
