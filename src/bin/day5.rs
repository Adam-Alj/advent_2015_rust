use std::{collections::{HashMap}, fs, };


pub fn main() {
    let file_contents =fs::read_to_string("./input_files/day5.txt").unwrap(); 
    let input = file_contents.lines().collect::<Vec<&str>>();

    let part_1_result = part_1(&input);
    println!("Part 1 result: {}", part_1_result);

    let part_2_result = part_2(&input);
    println!("Part 2 result: {}", part_2_result);
}

#[derive(Debug)]
struct WordValidator {
    previous_char_byte: Option<u8>,
    has_duplicate_chars: bool,
    has_ok_combos: bool,
    vowels: Vec<u8>
}

impl WordValidator {
    pub fn new() -> Self {
        Self {
            previous_char_byte: None,
            has_duplicate_chars: false,
            has_ok_combos: true,
            vowels: vec![]
        }
    }

    pub fn expand_word(&mut self, character: u8) -> bool {

        if let Some(previous_char) = self.previous_char_byte && previous_char == character {
            self.has_duplicate_chars = true;
        };

        if Self::char_is_vowel(character) {
            self.vowels.push(character.try_into().unwrap());
        }

        let result = if let Some(previous_char) = self.previous_char_byte {
            self.ok_combo(&[previous_char, character])
        } else {
            true
        };

        self.previous_char_byte = character.into();

        result
    }

    pub fn is_valid(self) -> bool {
        self.vowels.len() >= 3 && self.has_duplicate_chars && self.has_ok_combos
    }

    fn char_is_vowel(character: u8) -> bool {
        match character {
            b'a' | 
            b'e' |
            b'i' |
            b'o' |
            b'u' => true,
            _ => false
        }
    }

    fn ok_combo(&mut self, combo: &[u8; 2]) -> bool {
        match combo {
            b"ab" |
            b"cd" |
            b"pq" |
            b"xy" => {
                self.has_ok_combos = false;
                false
            },
            _ => true
        }
    }
}

fn part_1(strings: &Vec<&str>) -> usize {
    strings.iter().copied().filter(|s| {
        let mut validator: WordValidator = WordValidator::new();
        for char in s.bytes() {
            if !validator.expand_word(char) {
                break;
            }
        }
        validator.is_valid()
    }).collect::<Vec<&str>>().len()
}

fn part_2(strings: &Vec<&str>) -> usize {

    strings.iter().copied().filter(|s| {
        let mut pair_set = HashMap::<String, Vec<[usize; 2]>>::new();
        let mut has_separated_pair = false;
        let mut has_duplicate_pair= false;
        let string_arr = s.bytes().collect::<Vec<u8>>();

        for char_index in 0..string_arr.len() {
            if has_duplicate_pair && has_separated_pair {
                break;
            }

            if let Some(&separated_char) = string_arr.get(char_index + 2) && separated_char == string_arr[char_index] {
                has_separated_pair = true;
            } 

            if !has_duplicate_pair {
            if string_arr.get(char_index+1).is_some() && let Ok(seq_pair) = String::from_utf8(string_arr[char_index..=char_index+1].to_vec()) {
                has_duplicate_pair = pair_set.get(seq_pair.as_str()).filter(|indices| {
                    indices.into_iter().find(|indice| {
                        indice[0] != char_index && indice[0] != char_index + 1
                        && indice[1] != char_index && indice[1] != char_index + 1
                    }).is_some()
                }).is_some();

                pair_set.entry(seq_pair).or_default().push([char_index, char_index+1]);
            }
        }

        }
        has_duplicate_pair && has_separated_pair
    }).collect::<Vec<&str>>().len()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    #[test]
    fn part_1_examples_pass() {
        assert_eq!(part_1(&["ugknbfddgicrmopn"].into()), 1);
        assert_eq!(part_1(&["aaa"].into()), 1);
        assert_eq!(part_1(&["jchzalrnumimnmhp"].into()), 0);
        assert_eq!(part_1(&["haegwjzuvuyypxyu"].into()), 0);
        assert_eq!(part_1(&["dvszwmarrgswjxmb"].into()), 0);
    }

    #[test]
    fn part_2_examples_pass() {
        assert_eq!(part_2(&["qjhvhtzxzqqjkmpb"].into()), 1);
        assert_eq!(part_2(&["xxyxx"].into()), 1);
        assert_eq!(part_2(&["uurcxstgmygtbstg"].into()), 0);
        assert_eq!(part_2(&["ieodomkazucvgmuy"].into()), 0);
        assert_eq!(part_2(&["aaa"].into()), 0);
        assert_eq!(part_2(&["baba"].into()), 1);
        assert_eq!(part_2(&["bbabababababaaba"].into()), 1);
        assert_eq!(part_2(&["daad"].into()), 0);
        assert_eq!(part_2(&["tatat"].into()), 1);
                // Rule 1: Pair appears twice without overlapping
        // "xyxy" has pair "xy" twice (indices 0,1 and 2,3). No overlap.
        // Rule 2: "xyx" (indices 0,2) satisfies repeat with one between.
        assert_eq!(part_2(&["xyxy"].into()), 1);
        assert_eq!(part_2(&["xyxyz"].into()), 1);
        assert_eq!(part_2(&["xyxyabefe"].into()), 1);
        assert_eq!(part_2(&["aamnaaefe"].into()), 1);

        // "aabcdefgaa"
        // Rule 1: "aa" at start and end. No overlap. OK.
        // Rule 2: fails? 'aab', 'abc', 'bcd'... no repeat with one between unless I missed one.
        // Wait, "aabcdefgaa" -> 'a'...'a' (index 0 and 1) is adjacent.
        // The rule is "repeat with exactly one letter between them".
        // "aabcdefgaa":
        //  0123456789
        //  aabcdefgaa
        // Checking for X_X pattern.
        // No X_X pattern here. Should FAIL part_2 logic.
        assert_eq!(part_2(&["aabcdefgaa"].into()), 0);

        // "aaa"
        // Rule 1: "aa" (0,1) and "aa" (1,2). Overlap! Fails rule 1.
        // Rule 2: "a_a" (0,2). Passes rule 2.
        // Result: 0 (must pass both).
        assert_eq!(part_2(&["aaa"].into()), 0);

        // "aaaa"
        // Rule 1: "aa" (0,1) and "aa" (2,3). No overlap. Passes.
        // Rule 2: "a_a" (0,2). Passes.
        // Result: 1.
        assert_eq!(part_2(&["aaaa"].into()), 1);

        // Rule 2: Repeat with one letter between (X_X)
        // "qjhvhtzxzqqjkmpb"
        // Rule 1: "qj" ... "qj". Passes.
        // Rule 2: "vht" (h_h nope), "zxz" (Check!). Passes.
        assert_eq!(part_2(&["qjhvhtzxzqqjkmpb"].into()), 1);

        // "uurcxstgmygtbstg"
        // Rule 1: "tg" ... "tg". Passes.
        // Rule 2: No X_X pattern found?
        // u_r, u_c, r_x, c_s, x_t, s_g, t_m, g_y, m_g, y_t, g_b, t_s, b_d...
        // No. Fails rule 2.
        assert_eq!(part_2(&["uurcxstgmygtbstg"].into()), 0);

        // "ieodomkazucvgmuy"
        // Rule 1: No repeated pair?
        // ie, eo, od, do, om, mk, ka, az, zu, uc, cv, vg, gm, mu, uy
        // No pair repeats. Fails rule 1.
        // Rule 2: "odo" (o_o). Passes rule 2.
        assert_eq!(part_2(&["ieodomkazucvgmuy"].into()), 0);
    }
}