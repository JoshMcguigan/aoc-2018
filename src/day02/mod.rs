#[cfg(test)]
mod tests {
    use std::fs;
    use std::collections::HashMap;

    fn keep_common_letters(s1: &str, s2: &str) -> String {
        s1.chars()
            .zip(s2.chars())
            .fold(String::new(),
                  |mut acc, (c1, c2)| {
                      if c1.eq(&c2) { acc.push(c1); }

                      acc
                  }
            )
    }

    fn find_one_letter_off(input: String) -> Option<String> {
        for (index, line) in input.lines().enumerate() {
            for line2 in input.lines().skip(index) {
                let common_letters = keep_common_letters(line, line2);
                let char_diff = line.len() - common_letters.len();
                if char_diff == 1 { return Some(common_letters); }
            }
        }

        None
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("./src/day02/input-part1.txt").unwrap();
        let mut has_two_count = 0;
        let mut has_three_count = 0;

        for line in input.lines() {
            let mut letter_quantity = HashMap::new();
            for letter in line.chars() {
                match letter_quantity.get(&letter) {
                    Some(quantity) => letter_quantity.insert(letter, quantity + 1),
                    None => letter_quantity.insert(letter, 1),
                };
            }

            let mut has_two = false;
            let mut has_three = false;

            for &quanity in letter_quantity.values() {
                if quanity == 2 { has_two = true; }
                if quanity == 3 { has_three = true; }
            }

            if has_two { has_two_count += 1; }
            if has_three { has_three_count += 1; }
        }

        assert_eq!(7688, has_two_count * has_three_count);
    }

    #[test]
    fn keep_common_letters_same() {
        assert_eq!(String::from("abc"), keep_common_letters("abc", "abc"));
    }

    #[test]
    fn keep_common_letters_diff() {
        assert_eq!(String::from("ac"), keep_common_letters("abc", "azc"));
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./src/day02/input-part1.txt").unwrap();

        assert_eq!(Some(String::from("lsrivmotzbdxpkxnaqmuwcchj")), find_one_letter_off(input));
    }
}
