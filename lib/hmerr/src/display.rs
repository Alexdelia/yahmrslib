pub fn padding(n: Option<usize>) -> String {
    if let Some(n) = n {
        " ".repeat(n)
    } else {
        String::new()
    }
}
