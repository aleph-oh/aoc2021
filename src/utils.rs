use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;

pub(crate) fn input() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buf)
        .expect("Failed to read to buffer");
    buf
}

pub(crate) fn parse_input<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    s.lines().flat_map(|sub| sub.parse()).collect()
}
