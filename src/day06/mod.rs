use nom::{named, do_parse, separated_list, call, error_position, eol, map_res, tag, recognize};
use nom::types::CompleteStr;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: u16,
    y: u16,
}

named!(
    take_u16<CompleteStr, u16>,
    map_res!(recognize!(nom::digit), |CompleteStr(s)| u16::from_str(s))
);

named!(
    position<CompleteStr, Position>,
    do_parse!(
        x: take_u16 >>
        tag!(", ") >>
        y: take_u16 >>
        (Position { x, y })
    )
);

named!(
    positions<CompleteStr, Vec<Position> >,
    separated_list!(eol, position)
);

impl Position {
    fn distance_from(&self, other: Self) -> u32 {
        ( (self.x as i32 - other.x as i32).abs() +
            (self.y as i32 - other.y as i32).abs() ) as u32
    }
}

enum MinDistance {
    Unique(Position, u32),
    NonUnique(u32),
    None
}

fn closest(target: Position, positions: &Vec<Position>) -> Option<Position> {
    let distances_per_point = positions.iter()
        .map(|&p| {
            (p, p.distance_from(target))
        }).collect::<Vec<(Position, u32)>>();

    let mut min_distance_tracker = MinDistance::None;

    for (p, distance) in distances_per_point {
        match min_distance_tracker {
            MinDistance::Unique(_, min_distance) => {
                if distance < min_distance {
                    min_distance_tracker = MinDistance::Unique(p, distance);
                } else if distance == min_distance {
                    min_distance_tracker = MinDistance::NonUnique(distance);
                }
            },
            MinDistance::NonUnique(min_distance) => {
                if distance < min_distance {
                    min_distance_tracker = MinDistance::Unique(p, distance);
                }
            },
            MinDistance::None => { min_distance_tracker = MinDistance::Unique(p, distance); },
        }
    }

    match min_distance_tracker {
        MinDistance::Unique(p, _) => Some(p),
        _ => None, // return none if closest point is not unique
    }
}

fn create_grid(positions: &Vec<Position>) -> Vec<Vec<Option<Position>>> {
    let max_position = positions.iter()
        .fold(Position { x: 0, y: 0}, |acc, &pos| {
            Position {
                x: acc.x.max(pos.x),
                y: acc.y.max(pos.y),
            }
        });

    let mut grid = vec![];
    for y in 0..=max_position.y {

        grid.push(vec![]);

        for x in 0..=max_position.x {
            let target = Position { x, y };
            grid[y as usize].push(closest(target, &positions));
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_positions() {
        let input = CompleteStr(
            "1, 2\n3, 4"
        );

        let expected = vec![
            Position {
                x: 1, y: 2
            },
            Position {
                x: 3, y: 4
            },
        ];

        assert_eq!(expected, positions(input).unwrap().1);
    }

    #[test]
    fn distance_from_itself() {
        let p1 = Position {
            x: 1, y: 2
        };

        let p2 = Position {
            x: 1, y: 2
        };

        assert_eq!(0, p1.distance_from(p2));
    }

    #[test]
    fn distance_from_other() {
        let p1 = Position {
            x: 0, y: 0
        };

        let p2 = Position {
            x: 1, y: 1
        };

        assert_eq!(2, p1.distance_from(p2));
    }

    #[test]
    fn closest_two_points() {
        let p1 = Position {
            x: 1, y: 2
        };

        let p2 = Position {
            x: 3, y: 4
        };

        let positions = vec![p1, p2];

        let target = Position {
            x: 5, y: 6
        };

        assert_eq!(Some(p2), closest(target, &positions));
    }

    #[test]
    fn create_grid_small() {
        let p1 = Position {
            x: 0, y: 0
        };

        let p2 = Position {
            x: 1, y: 1
        };

        let positions = vec![p1, p2];

        let grid = create_grid(&positions);

        println!("{:?}", grid);

        assert_eq!(
            vec![
                vec![Some(p1), None],
                vec![None, Some(p2)]
            ],
            grid
        );
    }
}
