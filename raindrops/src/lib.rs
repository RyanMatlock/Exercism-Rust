pub fn raindrops(n: u32) -> String {
    let sounds: [(u32, &str); 3] = [
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong")
    ];
    let sounds = sounds.iter()
        .filter(|(divisor, _)| *n % *divisor == 0)
        .map(|(_, sound)| *sound.to_string());
    // match sounds.join("") {
    //     "" => n.to_string(),
    //     other => sounds.join("").to_string(),
    // }
    match sounds {
        Some(arr) => arr,
        None => [String; 0],
    }
}
