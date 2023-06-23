fn reverse_helper<'a>(s: &'a str, acc: Vec<String>) -> Vec<String> {
    if s.is_empty() {
        return acc
    } else {
        let mut chars = s.chars();
        let c = chars
            .next()
            .expect("string slice s isn't empty");
        let c = format!("{c}");
        let rest = chars
            .as_str()
            .to_string();
        let new_acc = [&vec!(c)[..], &acc[..]].concat();
        reverse_helper(&rest, new_acc)
    }
}

pub fn reverse(input: &str) -> String {
    reverse_helper(input, vec![])
        .join("")
}
