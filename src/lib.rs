#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;

mod conversion;
use conversion::ToPyHint;
use stubgen_macros_backend::stubgen;

#[cfg_attr(feature = "stubgen", stubgen)]
fn inc(x: Option<usize>) -> Option<usize> {
    x.map(|a| a + 1)
}

#[cfg_attr(feature = "stubgen", stubgen)]
fn double(xs: Vec<f64>) -> Vec<f64> {
    xs.into_iter().map(|x| 2.0 * x).collect()
}

#[cfg_attr(feature = "stubgen", stubgen)]
fn concat(x: &str, y: &str) -> String {
    let mut out = String::from(x);
    out.push_str(y);

    out
}

#[cfg_attr(feature = "stubgen", stubgen)]
fn items(data: HashMap<usize, usize>) -> Vec<(usize, usize)> {
    data.into_iter().collect()
}
