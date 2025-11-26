const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }
    let height = minefield.len();
    (0..height).map(|y| {
        let width = minefield[y].len();
        (0..width).map(|x| {
            if minefield[y].as_bytes()[x] == b'*' {
                "*".to_string()
            } else {
                let count = NEIGHBOURS.iter().filter(|(dx, dy)| {
                    let x = x as i32 + dx;
                    let y = y as i32 + dy;
                    x >= 0 && y >= 0 && x < width as i32 && y < height as i32 && minefield[y as usize].as_bytes()[x as usize] == b'*'
                }).count();
                match count {
                    0 => " ".to_string(),
                    _ => count.to_string(),
                }
            }
        }).collect()
    }).collect()
}
