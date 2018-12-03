use nom::{named, do_parse, separated_list, call, error_position, eol, map_res, tag, recognize};
use nom::types::CompleteStr;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct FabricSection {
    id: u16,
    left_pos: u16,
    top_pos: u16,
    width: u16,
    height: u16,
}

named!(
    take_u16<CompleteStr, u16>,
    map_res!(recognize!(nom::digit), |CompleteStr(s)| u16::from_str(s))
);

named!(
    fabric_section<CompleteStr, FabricSection>,
    do_parse!(
        tag!("#") >>
        id: take_u16 >>
        tag!(" @ ") >>
        left_pos: take_u16 >>
        tag!(",") >>
        top_pos: take_u16 >>
        tag!(": ") >>
        width: take_u16 >>
        tag!("x") >>
        height: take_u16 >>
        (FabricSection { id, left_pos, top_pos, width, height })
    )
);

named!(
    fabric_sections<CompleteStr, Vec<FabricSection> >,
    separated_list!(eol, fabric_section)
);

fn calc_used_fabric(sections: Vec<FabricSection>) -> HashMap<(u16, u16), u16> {
    // map tuple of left_pos, top_pos fabric location to quantity of sections which use it
    let mut used_fabric = HashMap::new();

    for section in sections {
        let FabricSection {
            left_pos, top_pos, width, height, ..
        } = section;

        for left in left_pos..(left_pos+width) {
            for top in top_pos..(top_pos+height) {
                let count = used_fabric
                    .entry((left, top))
                    .or_insert(0);
                *count += 1;
            }
        }
    }

    used_fabric
}

fn calc_double_used_fabric(used_fabric: HashMap<(u16, u16), u16>) -> u64 {
    let mut double_used = 0;

    for &usages in used_fabric.values() {
        if usages > 1 { double_used += 1; }
    }

    double_used
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn fabric_section_parse() {
        let input = CompleteStr(
            "#1 @ 286,440: 19x24\n#2 @ 430,120: 20x14"
        );

        let expected = vec![
            FabricSection {
                id: 1,
                left_pos: 286,
                top_pos: 440,
                width: 19,
                height: 24,
            },
            FabricSection {
                id: 2,
                left_pos: 430,
                top_pos: 120,
                width: 20,
                height: 14
            },
        ];

        assert_eq!(expected, fabric_sections(input).unwrap().1);
    }

    #[test]
    fn map_used_fabric_single() {
        let input = CompleteStr("#1 @ 1,3: 1x1");
        let sections = fabric_sections(input).unwrap().1;

        let used_fabric = calc_used_fabric(sections);

        assert_eq!(Some(&1), used_fabric.get(&(1, 3)));
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("./src/day03/input-part1.txt").unwrap();
        let sections = fabric_sections(CompleteStr(&input)).unwrap().1;

        let used_fabric = calc_used_fabric(sections);

        assert_eq!(115242, calc_double_used_fabric(used_fabric));
    }
}
