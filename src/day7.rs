pub mod day7 {
    use crate::input_to_vec;
    use fancy_regex::Regex;

    pub fn do_it() {
        println!("{}", count_ips());
    }

    fn count_ips() -> usize {
        let re = Regex::new(r"\[([a-z]+)]").unwrap();

        let mut ips_count = 0;

        for line in input_to_vec(7) {
            let mut out_of_brackets = String::from(&line);

            let mut texts_in_brackets = re.captures_iter(&line).map(|cap| {
                let text = cap.unwrap().get(1).unwrap().as_str().to_owned();
                out_of_brackets = line.replace(&text, "");
                text
            });

            if !texts_in_brackets.any(|text| is_abba(&text)) && is_abba(&out_of_brackets) {
                ips_count += 1;
            }
        }

        ips_count
    }

    fn is_abba(text: &String) -> bool {
        Regex::new(r"(.)(.)\2\1")
            .unwrap()
            .captures_iter(&text)
            .map(|r| r.unwrap())
            .any(|cap| cap.get(1).unwrap().as_str() != cap.get(2).unwrap().as_str())
    }
}
