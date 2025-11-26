const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];
//  [0, 1,    2, 3,  4, 5,      6, 7,    8, 9,  10, 11, 12, 13,  14, 15,    16, 17,   18, 19,
//   20, 21,    22, 23 ]

fn name_to_coordinates(student: &str) -> (usize, usize) {
    let student_index = STUDENTS.iter().position(|&s| s == student).unwrap(); // We can safely unwrap here because all the students are in the list
    (student_index * 2, student_index * 2 + 1)
}

#[test]
fn name_to_coordinates_test() {
    assert_eq!(name_to_coordinates("Alice"), (0, 1));
    assert_eq!(name_to_coordinates("Bob"), (2, 3));
    assert_eq!(name_to_coordinates("Charlie"), (4, 5));
    assert_eq!(name_to_coordinates("David"), (6, 7));
    assert_eq!(name_to_coordinates("Eve"), (8, 9));
    assert_eq!(name_to_coordinates("Fred"), (10, 11));
    assert_eq!(name_to_coordinates("Ginny"), (12, 13));
    assert_eq!(name_to_coordinates("Harriet"), (14, 15));
    assert_eq!(name_to_coordinates("Ileana"), (16, 17));
    assert_eq!(name_to_coordinates("Joseph"), (18, 19));
    assert_eq!(name_to_coordinates("Kincaid"), (20, 21));
    assert_eq!(name_to_coordinates("Larry"), (22, 23));
}

pub fn plants<'a>(diagram: &'a str, student: &str) -> Vec<&'a str> {
    diagram
        .lines()
        .flat_map(|line| {
            let coordinates = name_to_coordinates(student);
            line[coordinates.0..=coordinates.1] // safe here because all inputs are ASCII so 1 character is 1 byte
                .chars()
                .map(|c| match c {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => "I don't know what this is",
                })
        })
        .collect()
}

#[test]
fn unicode_scalar_value() {
    let word = "l'étéêëè";
    assert_eq!(&word[..1], "l");
    let mut chars = word.chars();
    assert_eq!(chars.next(), Some('l'));
    assert_eq!(chars.next(), Some('\''));
    assert_eq!(chars.next(), Some('é'));
    assert_eq!(chars.next(), Some('t'));
    assert_eq!(chars.next(), Some('é'));
    assert_eq!(chars.next(), Some('ê'));
    assert_eq!(chars.next(), Some('ë'));
    assert_eq!(chars.next(), Some('è'));
    assert_eq!(chars.next(), None);
}