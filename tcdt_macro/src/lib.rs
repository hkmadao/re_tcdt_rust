extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{self, parse_macro_input, DeriveInput};

mod macro_impl {
    pub mod tcdt_api_scan;
    pub mod tcdt_po_cud;
    pub mod tcdt_po_save;
    pub mod tcdt_vo_convert;
}

use macro_impl::{tcdt_api_scan, tcdt_po_cud, tcdt_po_save, tcdt_vo_convert};

#[proc_macro_derive(ViewObectConvert, attributes(tcdt_vo))]
pub fn view_object_convert_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = parse_macro_input!(input as DeriveInput);
    tcdt_vo_convert::tcdt_vo_convert(ast).unwrap()
}

#[proc_macro_derive(ParamOjectSave, attributes(tcdt_po))]
pub fn param_object_save_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = parse_macro_input!(input as DeriveInput);
    tcdt_po_save::tcdt_po_save(ast).unwrap()
}

#[proc_macro_derive(ParamObjectCud, attributes(tcdt_po))]
pub fn param_object_cud_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = parse_macro_input!(input as DeriveInput);
    tcdt_po_cud::tcdt_po_cud(ast).unwrap()
}

/// annotation, attribute must be method name
#[proc_macro_attribute]
pub fn tcdt_route(attr: TokenStream, item: TokenStream) -> TokenStream {
    tcdt_api_scan::tcdt_route_scan(attr, item).unwrap()
}
