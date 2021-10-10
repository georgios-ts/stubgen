extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

mod argument;
use argument::PyArg;

use std::convert::TryFrom;

#[proc_macro_attribute]
pub fn stubgen(_: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);
    let sig = item_fn.sig;

    let name = sig.ident;
    let stubgen_name = format_ident!("stubgen_{}", name);

    let mut args_with_hints_build = Vec::new();
    for pair in sig.inputs.into_pairs() {
        let (arg, comma) = pair.into_tuple();

        if let Ok(PyArg { name, ty }) = PyArg::try_from(&arg) {
            let delim = comma.map_or("", |_| ", ");
            args_with_hints_build.push(quote!(
                let hint = format!(
                    "{arg}: {ty}{delim}",
                    arg = stringify!(#name),
                    ty = <#ty as ToPyHint>::convert(),
                    delim = #delim
                );
                args_with_hints.push_str(&hint);
            ))
        }
    }

    let out_with_hints = match sig.output {
        syn::ReturnType::Default => quote!("None"),
        syn::ReturnType::Type(_, ty) => quote!(<#ty as ToPyHint>::convert()),
    };

    let out = quote! {
        #[test]
        fn #stubgen_name() {
            let mut args_with_hints = String::new();
            #(#args_with_hints_build)*

            println!(
                "def {name}({args}) -> {output}: ...",
                name = stringify!(#name),
                args = args_with_hints,
                output = #out_with_hints
            );
        }
    };

    TokenStream::from(out)
}
