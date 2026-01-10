use std::{collections::HashSet, fs};

const INPUT_FILE_NAME: &str = "day3.txt";

fn main() {
    let solution_input = fs::read_to_string(format!("./input_files/{}", INPUT_FILE_NAME))
        .expect(format!("Input file {} should exist.", INPUT_FILE_NAME).as_str());

    let part_1_solution = part_1(&solution_input);
    println!("Solution to part 1:\n{}", part_1_solution);

    let part_2_solution = part_2(&solution_input);
    println!("Solution to part 2:\n{:?}", part_2_solution);
}

fn part_1(input_directions: &str) -> u32 {
    let mut santas_current_position = [0, 0];
    let mut visited = HashSet::<[i32; 2]>::from([santas_current_position]);

    input_directions.bytes().into_iter().for_each(|direction| {
        match direction {
            b'>' => santas_current_position[0] += 1,
            b'<' => santas_current_position[0] -= 1,
            b'^' => santas_current_position[1] += 1,
            b'v' => santas_current_position[1] -= 1,
            _ => panic!("bad input!")
        };

        visited.insert(santas_current_position);
    });

    visited.len().try_into().unwrap()
}

fn part_2(input_directions: &str) -> u32 {
    let mut santas_current_position = [0, 0];
    let mut robo_santas_current_position = [0, 0];
    let mut visited = HashSet::<[i32; 2]>::from([santas_current_position]);

    input_directions.bytes().enumerate().for_each(|(index, direction)| {
        let position_to_mutate = if index % 2 == 0 {
            &mut santas_current_position
        } else {
            &mut robo_santas_current_position
        };

        match direction {
            b'>' => position_to_mutate[0] += 1,
            b'<' => position_to_mutate[0] -= 1,
            b'^' => position_to_mutate[1] += 1,
            b'v' => position_to_mutate[1] -= 1,
            _ => panic!("bad input!")
        };

        visited.insert(*position_to_mutate);
    });

    visited.len().try_into().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};


    #[test]
    fn part_1_passes_examples() {
        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn part_2_passes_examples() {
        assert_eq!(part_2("^v"), 3);
        assert_eq!(part_2("^>v<"), 3);
        assert_eq!(part_2("^v^v^v^v^v"), 11);
    }
}