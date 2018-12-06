enum ReactionType {
    Any,
    Single(char),
}

fn will_react_single_letter(c1: char, c2: char, reaction_type: &ReactionType) -> bool {
    let reaction_type_match = match reaction_type {
        ReactionType::Any => true,
        ReactionType::Single(c) => c1.eq_ignore_ascii_case(&c)
    };
    c1.eq_ignore_ascii_case(&c2) && (c1 != c2) && reaction_type_match
}

fn will_react(c1: char, c2: char) -> bool {
    will_react_single_letter(c1, c2, &ReactionType::Any)
}

fn react_single_letter(input: &str, reaction_type: &ReactionType) -> String {
    if input.is_empty() { return String::new() }

    let chars = input.chars().collect::<Vec<char>>();
    let mut result = String::new();

    let mut i = 0;

    while i < (chars.len() - 1) {
        let c1 = chars[i];
        let c2 = chars[i+1];

        if will_react_single_letter(c1, c2, &reaction_type) {
            i += 2;
        } else {
            result.push(c1);

            i += 1;
        }

        let next_char_is_last = i == chars.len() - 1;
        if next_char_is_last { result.push(chars[i]); }
    }

    result
}

fn react(input: &str) -> String {
    react_single_letter(input, &ReactionType::Any)
}

fn chain_react_single_letter(input: &str, reaction_type: ReactionType) -> String {
    let result = react_single_letter(input, &reaction_type);

    if result.len() == input.len() {
        // reaction complete
        result
    } else {
        // continue reaction
        chain_react_single_letter(&result, reaction_type)
    }
}

fn chain_react(input: &str) -> String {
    chain_react_single_letter(input, ReactionType::Any)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;
    use std::fs;

    #[test]
    fn will_react_aA() {
        assert!(will_react('A', 'a'));
    }

    #[test]
    fn will_react_ab() {
        assert!(!will_react('a', 'b'));
    }

    #[test]
    fn react_aA() {
        assert_eq!("", &react("aA"));
    }

    #[test]
    fn react_abab() {
        assert_eq!("abab", &react("abab"));
    }

    #[test]
    fn react_ababa() {
        assert_eq!("ababa", &react("ababa"));
    }

    #[test]
    fn react_aAb() {
        assert_eq!("b", &react("aAb"));
    }

    #[test]
    fn react_baA() {
        assert_eq!("b", &react("baA"));
    }

    #[test]
    fn react_bbaA() {
        assert_eq!("bb", &react("bbaA"));
    }

    #[test]
    fn react_bbaAb() {
        assert_eq!("bbb", &react("bbaAb"));
    }

    #[test]
    fn react_empty() {
        assert_eq!("", &react(""));
    }

    #[test]
    fn part1_example() {
        assert_eq!("dabCBAcaDA", &chain_react("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn part1_example_2() {
        assert_eq!("oj", &chain_react("hHsSmMHhhHwWlLojYCclLy"));
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("./src/day05/input.txt").unwrap();
        assert_eq!(11814, chain_react(input.trim()).len())
    }

    #[test]
    fn part2() {
        let input_raw = fs::read_to_string("./src/day05/input.txt").unwrap();
        let input = input_raw.trim();

        let best_reaction = (b'a' ..= b'z' )
            .filter_map(|c| {
                let c = c as char;
                if c.is_alphabetic() { Some(c) } else { None }
            })
            .map(|c| {
                let input_without_char = input
                    .replace(c, "")
                    .replace(c.to_ascii_uppercase(), "");
                let length = chain_react(&input_without_char).len();
                (c, length)
            })
            .max_by(|a, b| {
                b.1.cmp(&a.1)
            })
            .unwrap();

        assert_eq!(4282, best_reaction.1);
    }
}
