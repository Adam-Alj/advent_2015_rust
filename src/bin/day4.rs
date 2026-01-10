fn main() {
    println!("Day 1 solution: {}", part_1("yzbqklnj"));
    println!("Day 2 solution: {}", part_2("yzbqklnj"));
}

fn part_1(input: &str) -> u64 {
    let mut affix_number: u64 = 1;

    loop {
        let digest = md5::compute(format!("{}{}", input, affix_number));
        
        if digest[0] == 0x00 && digest[1] == 0x00 && digest[2] < 0x10 {
            break;
        }
        
        affix_number += 1;
    }

    affix_number
}

fn part_2(input: &str) -> u64 {
    let mut affix_number: u64 = 1;

    loop {
        let digest = md5::compute(format!("{}{}", input, affix_number));
        
        if digest[0] == 0x00 && digest[1] == 0x00 && digest[2] == 0x00 {
            break;
        }
        
        affix_number += 1;
    }

    affix_number
}


#[cfg(test)]
mod test {
    use crate::part_1;

    #[test]
    fn part_1_passes_examples() {
        assert_eq!(
            part_1("abcdef"), 609043
        );
        assert_eq!(
            part_1("pqrstuv"), 1048970
        );
    }
}