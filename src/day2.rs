pub mod day2 {
    use crate::input_to_vec;
    use grid::{grid, Grid};

    pub fn do_it() {
        let keypad1: Grid<char> = grid![
            ['1', '2', '3']
            ['4', '5', '6']
            ['7', '8', '9']
        ];
        println!("{}", compute_code(keypad1, (1, 1)));

        let keypad2: Grid<char> = grid![
            [' ', ' ', '1', ' ', ' ']
            [' ', '2', '3', '4', ' ']
            ['5', '6', '7', '8', '9']
            [' ', 'A', 'B', 'C', ' ']
            [' ', ' ', 'D', ' ', ' ']
        ];
        println!("{}", compute_code(keypad2, (0, 2)));
    }

    fn compute_code(keypad: Grid<char>, mut position: (i32, i32)) -> String {
        let mut number = '5';
        let mut code = String::new();

        for line in input_to_vec(2) {
            for letter in line.chars() {
                let previous_position = position;
                position = match letter {
                    'U' => (position.0 - 1, position.1),
                    'D' => (position.0 + 1, position.1),
                    'L' => (position.0, position.1 - 1),
                    'R' => (position.0, position.1 + 1),
                    _ => panic!(),
                };
                number = *keypad
                    .get(position.0 as usize, position.1 as usize)
                    .filter(|&x| x != &' ')
                    .unwrap_or_else(|| {
                        position = previous_position;
                        &number
                    });
            }
            code += &number.to_string();
        }
        code
    }
}
