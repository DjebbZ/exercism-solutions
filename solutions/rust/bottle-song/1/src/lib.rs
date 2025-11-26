use std::collections::HashMap;
use std::sync::LazyLock;

static NUMBERS: LazyLock<HashMap<u32, &str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(10u32, "Ten");
    m.insert(9u32, "Nine");
    m.insert(8u32, "Eight");
    m.insert(7u32, "Seven");
    m.insert(6u32, "Six");
    m.insert(5u32, "Five");
    m.insert(4u32, "Four");
    m.insert(3u32, "Three");
    m.insert(2u32, "Two");
    m.insert(1u32, "One");
    m.insert(0u32, "No");
    m
});

fn to_word(n: u32) -> &'static str {
    (*NUMBERS).get(&n).expect("Number should be in map")
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = Vec::new();

    for b in ((start_bottles - take_down + 1)..=start_bottles).rev() {
        if b < 1 {
            break;
        }
        let (bottle_before, bottle_after) = match b {
            1 => ("bottle", "bottles"),
            2 => ("bottles", "bottle"),
            _ => ("bottles", "bottles"),
        };

        song.push(format!(
            "{} green {bottle_before} hanging on the wall,\n",
            to_word(b)
        ));
        song.push(format!(
            "{} green {bottle_before} hanging on the wall,\n",
            to_word(b)
        ));
        song.push("And if one green bottle should accidentally fall,\n".to_string());
        song.push(format!(
            "There'll be {} green {bottle_after} hanging on the wall.\n\n",
            to_word(b - 1).to_ascii_lowercase()
        ));
    }

    song.into_iter().collect()
}
