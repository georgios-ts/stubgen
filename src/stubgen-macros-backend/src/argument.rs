use std::convert::TryFrom;

use syn::spanned::Spanned;
use syn::{FnArg, Ident, Type};

/// Taken from
/// [pyo3](https://github.com/PyO3/pyo3/blob/release-0.14/pyo3-macros-backend/src/utils.rs#L15)
macro_rules! bail_spanned {
    ($span:expr => $msg:expr) => {
        return Err(syn::Error::new($span, $msg))
    };
}

pub struct PyArg<'a> {
    pub name: &'a Ident,
    pub ty: &'a Type,
}

impl<'a> TryFrom<&'a FnArg> for PyArg<'a> {
    type Error = syn::Error;

    fn try_from(arg: &'a FnArg) -> syn::Result<PyArg<'a>> {
        match arg {
            FnArg::Receiver(recv) => {
                bail_spanned!(recv.span() => "unexpected receiver")
            }
            FnArg::Typed(cap) => {
                let ident = match *cap.pat {
                    syn::Pat::Ident(syn::PatIdent { ref ident, .. }) => ident,
                    _ => bail_spanned!(cap.pat.span() => "unsupported argument"),
                };

                let arg = PyArg {
                    name: ident,
                    ty: &*cap.ty,
                };

                Ok(arg)
            }
        }
    }
}
