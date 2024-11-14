use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

pub(crate) fn tcdt_route_scan(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let args_str = args.to_string();
    let args_str = args_str.trim();
    let ast = syn::parse::<syn::ItemFn>(input.clone()).unwrap();
    let fn_str = ast.sig.ident.to_string();

    if args_str != fn_str {
        let my_err = syn::Error::new(
            Span::call_site(),
            format!(
                "attribute: '{}' not equal method name: '{}'",
                args_str, fn_str
            ),
        );
        return Err(my_err);
    }

    let gen = quote! {};
    let mut result = gen.to_string();
    result.push_str(&input.to_string());
    Ok(input)
}
