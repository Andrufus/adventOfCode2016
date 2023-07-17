pub mod day4 {
    use crate::input_to_vec;
    use fancy_regex::Regex;
    use std::collections::HashMap;

    pub fn do_it() {
        let sum: i32 = get_real_rooms().iter().map(|room| room.0).sum();
        println!("{sum}");

        let room = get_real_rooms()
            .into_iter()
            .map(|room| (room.0, caesar_cipher::decrypt(room.1, room.0 % 27)))
            .find(|room| room.1.contains("north"))
            .unwrap();
        println!("{} {}", room.0, room.1);
    }

    fn get_real_rooms() -> Vec<(i32, String)> {
        let reg = Regex::new(r"(?P<name>([a-z]+-)+)(?P<id>\d+)\[(?P<checksum>[a-z]+)]").unwrap();

        let mut rooms: Vec<(i32, String)> = Vec::new();

        for line in input_to_vec(4) {
            let captures = reg.captures(&line).unwrap().unwrap();
            let id = captures["id"].parse::<i32>().unwrap();
            let checksum = captures["checksum"].to_string();

            let mut name = String::new();
            for cap in reg.captures_iter(&line) {
                name += &cap.unwrap()["name"].replace("-", "");
            }

            let mut letters_count = HashMap::new();
            for char in name.chars() {
                *letters_count.entry(char).or_insert(0) += 1;
            }
            let mut letters_count: Vec<_> = letters_count.iter().collect();
            letters_count.sort_unstable_by_key(|item| (-item.1, item.0));

            let mut check = String::new();
            for (letter, _count) in &letters_count[0..5] {
                check.push(**letter);
            }

            if checksum == check {
                rooms.push((id, name));
            }
        }

        rooms
    }
}
