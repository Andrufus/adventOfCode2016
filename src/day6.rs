pub mod day6 {
    use crate::input_to_vec;
    use std::collections::HashMap;

    pub fn do_it() {
        let mut letters_count: Vec<HashMap<char, i32>> = Vec::new();
        for _i in 0..8 {
            letters_count.push(HashMap::new());
        }

        for line in input_to_vec(6) {
            let mut column = 0;
            for char in line.chars() {
                *letters_count
                    .get_mut(column)
                    .unwrap()
                    .entry(char)
                    .or_insert(0) += 1;
                column += 1;
            }
        }

        let message: String = letters_count
            .iter()
            .map(|col| col.iter().min_by(|&a, &b| a.1.cmp(b.1)).unwrap().0)
            .collect();

        println!("{message}");
    }
}
