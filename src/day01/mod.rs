use nom::{
    named,
    alt,
    tag,
    error_position,
    do_parse,
    call,
    eol,
    separated_list,
    map_res,
    recognize,
};
use nom::types::CompleteStr;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Sign {
    Pos,
    Neg,
}

named!(
    sign<CompleteStr, Sign>,
    alt!(
        tag!("+") => { |_| Sign::Pos } |
        tag!("-") => { |_| Sign::Neg }
    )
);

#[derive(Debug, PartialEq)]
struct Change {
    pub sign: Sign,
    pub value: u32,
}

named!(
    change<CompleteStr, Change>,
    do_parse!(
        sign: sign >>
        value: map_res!(recognize!(nom::digit), |CompleteStr(s)| u32::from_str(s)) >>
        (Change { sign, value })
    )
);

named!(
    changes<CompleteStr, Vec<Change> >,
    separated_list!(eol, change)
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::collections::HashSet;

    #[test]
    fn sign_positive() {
        assert_eq!(Sign::Pos, sign(CompleteStr("+")).unwrap().1);
    }

    #[test]
    fn sign_negative() {
        assert_eq!(Sign::Neg, sign(CompleteStr("-")).unwrap().1);
    }

    #[test]
    fn change_pos() {
        assert_eq!(
            Change {
                sign: Sign::Pos,
                value: 123
            },
            change(CompleteStr("+123")).unwrap().1
        );
    }

    #[test]
    fn change_neg() {
        assert_eq!(
            Change {
                sign: Sign::Neg,
                value: 456
            },
            change(CompleteStr("-456")).unwrap().1
        );
    }

    #[test]
    fn change_multiple() {
        let expected = vec![
            Change {
                sign: Sign::Neg,
                value: 456
            },
            Change {
                sign: Sign::Pos,
                value: 123
            },
        ];

        assert_eq!(
            expected,
            changes(CompleteStr("-456\n+123")).unwrap().1
        );
        assert_eq!(
            expected,
            changes(CompleteStr("-456\n+123\n")).unwrap().1
        );
    }

    #[test]
    fn part1(){
        let input = fs::read_to_string("./src/day01/input-part1.txt").unwrap();

        let changes : Vec<Change> = changes(CompleteStr(&input)).unwrap().1;

        let total = changes.iter()
            .fold(0i32, |acc, change| {
                match change.sign {
                    Sign::Pos => acc + change.value as i32,
                    Sign::Neg => acc - change.value as i32
                }
            });

        assert_eq!(538, total);
    }

    #[test]
    fn part2(){
        let input = fs::read_to_string("./src/day01/input-part1.txt").unwrap();

        let changes : Vec<Change> = changes(CompleteStr(&input)).unwrap().1;

        let mut frequencies = HashSet::new();
        let mut acc = 0;

        let mut frequency_reached_twice = None;

        while frequency_reached_twice.is_none() {

            for change in &changes {

                if frequencies.contains(&acc) {
                    frequency_reached_twice = Some(acc);
                    break;
                } else {
                    frequencies.insert(acc);
                }

                acc = match change.sign {
                    Sign::Pos => acc + change.value as i32,
                    Sign::Neg => acc - change.value as i32
                };
            }
        }

        assert_eq!(Some(77271), frequency_reached_twice);
    }
}
