use std::collections::HashMap;

pub trait ToPyHint {
    fn convert() -> String;
}

impl ToPyHint for usize {
    fn convert() -> String {
        String::from("int")
    }
}

impl ToPyHint for f64 {
    fn convert() -> String {
        String::from("float")
    }
}

impl ToPyHint for String {
    fn convert() -> String {
        String::from("str")
    }
}

impl<'a> ToPyHint for &'a str {
    fn convert() -> String {
        String::from("str")
    }
}

impl<A> ToPyHint for Option<A>
where
    A: ToPyHint,
{
    fn convert() -> String {
        format!("Optional[{elem}]", elem = <A as ToPyHint>::convert())
    }
}

impl<A, B> ToPyHint for (A, B)
where
    A: ToPyHint,
    B: ToPyHint,
{
    fn convert() -> String {
        format!(
            "Tuple[{first}, {second}]",
            first = <A as ToPyHint>::convert(),
            second = <B as ToPyHint>::convert()
        )
    }
}

impl<A> ToPyHint for Vec<A>
where
    A: ToPyHint,
{
    fn convert() -> String {
        format!("List[{elem}]", elem = <A as ToPyHint>::convert())
    }
}

impl<K, V> ToPyHint for HashMap<K, V>
where
    K: ToPyHint,
    V: ToPyHint,
{
    fn convert() -> String {
        format!(
            "Dict[{key}, {val}]",
            key = <K as ToPyHint>::convert(),
            val = <V as ToPyHint>::convert()
        )
    }
}
