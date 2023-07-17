pub mod day3 {
    use crate::input_to_vec;

    pub fn do_it() {
        println!("{}", count_triangles());
        println!("{}", count_triangles_vertically());
    }

    fn count_triangles() -> usize {
        let mut triangles = 0;

        for line in input_to_vec(3) {
            if is_triangle_possible(line_to_triplet(line)) {
                triangles += 1;
            }
        }

        triangles
    }

    fn count_triangles_vertically() -> usize {
        let mut triangles = 0;

        let mut three_lines: Vec<Vec<u32>> = Vec::new();
        for line in input_to_vec(3) {
            if three_lines.len() == 3 {
                for triplet in vec![
                    vec![three_lines[0][0], three_lines[1][0], three_lines[2][0]],
                    vec![three_lines[0][1], three_lines[1][1], three_lines[2][1]],
                    vec![three_lines[0][2], three_lines[1][2], three_lines[2][2]],
                ] {
                    if is_triangle_possible(triplet) {
                        triangles += 1;
                    }
                }
                three_lines.clear();
            }
            three_lines.push(line_to_triplet(line));
        }

        triangles + 3
    }

    fn line_to_triplet(line: String) -> Vec<u32> {
        line.split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect()
    }

    fn is_triangle_possible(mut triplet: Vec<u32>) -> bool {
        triplet.sort();
        triplet[0] + triplet[1] > triplet[2]
    }
}
