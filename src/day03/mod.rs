use nom::{named, do_parse, separated_list, call, error_position, eol, map_res, tag, recognize};
use nom::types::CompleteStr;
use std::str::FromStr;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
