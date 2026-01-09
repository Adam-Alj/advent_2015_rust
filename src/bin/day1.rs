use std::fs;

const INPUT_FILE_NAME: &str = "day1.txt";

fn main() {
    let solution_input = fs::read_to_string(format!("./input_files/{}", INPUT_FILE_NAME)).expect(format!("Input file {} should exist.", INPUT_FILE_NAME).as_str());

    let part_1_solution = part_1(&solution_input);
    println!("Solution to part 1:\n{}", part_1_solution);

    let part_2_solution = part_2(&solution_input);
    println!("Solution to part 2:\n{:?}", part_2_solution);
}

fn part_1(s: &str) -> i32 {
    s.bytes().into_iter().fold(0i32, |floor_sum, char| {
        if char == b'(' {
            floor_sum + 1
        } else {
            floor_sum - 1
        }
    })

}

fn part_2(s: &str) -> Option<i32> {
    let mut floor: i32 = 0;
    for (index, value) in s.bytes().into_iter().enumerate() {
       if value == b'(' {
        floor += 1;
       } else {
        floor -= 1;
       }

       if floor == -1 {
        return Some((index + 1).try_into().expect("Should be able to convert usize to i32"));
       }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    #[test]
    fn part_1_passes_examples() {
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("()()()"), 0);
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1(")())())"), -3);
    }

    #[test]
    fn part_2_passes_examples() {
        assert_eq!(part_2(")").unwrap(), 1);
        assert_eq!(part_2("()())").unwrap(), 5);
    }
}