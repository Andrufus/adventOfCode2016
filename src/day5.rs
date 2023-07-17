pub mod day5 {
    use std::time::Instant;

    pub fn do_it() {
        decrypt(false);
        decrypt(true);
    }

    fn decrypt(with_position: bool) {
        println!("Decrypting...");
        let now = Instant::now();

        let mut password = String::from("########");
        let mut index = 0;
        let mut position = 0;
        loop {
            let hash = format!("{:x}", md5::compute(format!("abbhdwsy{index}")));
            if hash.starts_with("00000") {
                if with_position {
                    let position = hash[5..6].parse::<usize>().unwrap_or(9);
                    if position <= 7 && &password[position..position + 1] == "#" {
                        password.replace_range(position..position + 1, &hash[6..7]);
                    }
                } else {
                    password.replace_range(position..position + 1, &hash[5..6]);
                    position += 1;
                }

                if !password.contains("#") {
                    break;
                }
            }
            index += 1;
        }
        println!("{password}");
        println!("Elapsed: {:.2?}", now.elapsed());
    }
}
