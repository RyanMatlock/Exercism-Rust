fn reverse_helper<'a>(s: &'a str, acc: Vec<&'a str>) -> Vec<&'a str> {
    if s.is_empty() {
        return acc
    } else {
        // let mut chars = s.chars();
        // let mut buffer = [0; 4];
        // let c = chars.next().expect("slice s isn't empty").encode_utf8(&mut buffer);
        // let rest = chars.collect::<Vec<_>>();
        let mut new_vec = vec!(&s[0..1]);
        new_vec.extend(acc);
        reverse_helper(&s[1..], new_vec)
    }
}

pub fn reverse(input: &str) -> String {
    reverse_helper(input, vec![])
        .join("")
}
