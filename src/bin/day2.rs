use std::fs;

const INPUT_FILE_NAME: &str = "day2.txt";

#[derive(Debug)]
struct RectangularCuboid {
    length: u32,
    width: u32,
    height: u32
}

impl RectangularCuboid {
    fn surface_area(&self) -> u32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn smallest_side(&self) -> u32 {
        *[self.length * self.width, self.width * self.height, self.height * self.length].iter().min().expect("There should be a min value")
    }

    fn cubic_feet(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn shortest_perimeter(&self) -> u32 {
        if self.length <= self.height && self.width <= self.height {
            2 * self.length + 2 * self.width
        } else if self.height <= self.width && self.length <= self.width {
            2 * self.length + 2 * self.height
        } else {
            2 * self.height + 2 * self.width
        }
    }
}

impl From<&str> for RectangularCuboid {
    fn from(value: &str) -> Self {
        let values = value.split("x").collect::<Vec<&str>>();

        Self { length: values[0].parse::<u32>().unwrap(), width: values[1].parse::<u32>().unwrap(), height: values[2].parse::<u32>().unwrap() }
    }
}

fn main() {
    let solution_input = fs::read_to_string(format!("./input_files/{}", INPUT_FILE_NAME))
        .expect(format!("Input file {} should exist.", INPUT_FILE_NAME).as_str())
        .lines()
        .map(RectangularCuboid::from)
        .collect::<Vec<RectangularCuboid>>();

    let part_1_solution = part_1(&solution_input);
    println!("Solution to part 1:\n{}", part_1_solution);

    let part_2_solution = part_2(&solution_input);
    println!("Solution to part 2:\n{:?}", part_2_solution);
}

fn part_1(s: &Vec<RectangularCuboid>) -> u32 {
    s.iter().fold(0, |wrapping_paper_sum, cuboid| wrapping_paper_sum + cuboid.surface_area() + cuboid.smallest_side())
}

fn part_2(s: &Vec<RectangularCuboid>) -> u32 {
    s.iter().fold(0, |ribbon_sum, cuboid| {
        ribbon_sum + cuboid.cubic_feet() + cuboid.shortest_perimeter()
    })
}

#[cfg(test)]
mod test {
    use crate::{RectangularCuboid, part_1, part_2};

    #[test]
    fn part_1_passes_examples() {
        assert_eq!(part_1(&vec![
            RectangularCuboid {
                length: 2,
                width: 3,
                height: 4
            }
        ]), 58);
        assert_eq!(part_1(&vec![
            RectangularCuboid {
                length: 1,
                width: 1,
                height: 10
            }
        ]), 43);
    }

    #[test]
    fn part_2_passes_examples() {
        assert_eq!(part_2(&vec![
            RectangularCuboid {
                length: 2,
                width: 3,
                height: 4
            }
        ]), 34);
        assert_eq!(part_2(&vec![
            RectangularCuboid {
                length: 1,
                width: 1,
                height: 10
            }
        ]), 14);
    }
}