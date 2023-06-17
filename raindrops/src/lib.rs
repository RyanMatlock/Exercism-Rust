pub fn raindrops(n: u32) -> String {
    let sounds: [(u32, &str); 3] = [
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong")
    ];
    let sounds = sounds.iter()
        .filter(|(divisor, _)| n % divisor == 0)
        .map(|(_, sound)| sound.to_string())
        .collect::<Vec<_>>()
        .join("");
    if sounds.is_empty() {
        n.to_string()
    } else {
        sounds
    }
}
