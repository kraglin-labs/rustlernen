
#[macro_export]
macro_rules! scan {
    ($($val:expr),*) => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.split_whitespace();
        $(
            if let Some(token) = tokens.next() {
                *$val = token.parse().expect("Failed to parse token");
            }
        )*
    }};
}
