use nom::{named, do_parse, separated_list, call, error_position, eol, map_res, tag, recognize, take, parse_to, count, ws, sep, wrap_sep};
use nom::types::CompleteStr;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

type Metadata = Vec<u16>;
type Nodes = Vec<Box<Node>>;

#[derive(Debug, PartialEq)]
struct Node {
    metadata: Metadata,
    children: Nodes,
}

named!(
    take_u16<CompleteStr, u16>,
    map_res!(recognize!(nom::digit), |CompleteStr(s)| u16::from_str(s))
);

named!(
    node<CompleteStr, Box<Node> >,
    do_parse!(
        num_children: take_u16 >>
        tag!(" ") >>
        num_metadata: take_u16 >>
        children: count!(ws!(node), num_children as usize) >>
        metadata: count!(ws!(take_u16), num_metadata as usize) >>
        (Box::new(Node { metadata, children }))
    )
);

fn sum_metadata(node: &Node) -> u64 {
    node.metadata.iter().map(|metadata| *metadata as u64).sum::<u64>() +
        node.children.iter()
            .fold(0, |acc, node| {
                acc + sum_metadata(&**node)
            })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_node_no_children() {
        let node = node(CompleteStr("0 3 10 11 12")).unwrap().1;
        let expected = Node {
            metadata: vec![10, 11, 12],
            children: vec![]
        };

        assert_eq!(expected, *node);
    }

    #[test]
    fn parse_nodes() {
        let node = node(CompleteStr("1 1 0 1 99 2")).unwrap().1;
        let expected = Node {
            metadata: vec![2],
            children: vec![
                Box::new(
                    Node {
                        metadata: vec![99],
                        children: vec![]
                    }
                )
            ]
        };

        assert_eq!(expected, *node);
        assert_eq!(101, sum_metadata(&node));
    }

    #[test]
    fn parse_nodes_example() {
        let input = fs::read_to_string("./src/day08/input-example.txt").unwrap();
        let node = node(CompleteStr(&input)).unwrap().1;

        assert_eq!(138, sum_metadata(&node));
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("./src/day08/input.txt").unwrap();
        let node = node(CompleteStr(&input)).unwrap().1;

        assert_eq!(41454, sum_metadata(&node));
    }

}
