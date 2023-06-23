fn reverse_helper<'a>(s: &'a str, acc: Vec<&'a str>) -> Vec<&'a str> {
    if s.is_empty() {
        return acc
    } else {
        let mut chars = s.chars();
        let c = chars
            .next()
            .expect("slice s isn't empty")
            .to_string()
            .as_str();
        let rest = chars.as_str();
        let new_acc = [&vec!(c)[..], &acc].concat();
        reverse_helper(rest, new_acc)
    }
}

pub fn reverse(input: &str) -> String {
    reverse_helper(input, vec![])
        .join("")
}
