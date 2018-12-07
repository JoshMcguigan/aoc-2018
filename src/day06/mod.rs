use nom::{named, do_parse, separated_list, call, error_position, eol, map_res, tag, recognize};
use nom::types::CompleteStr;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

type Grid = Vec<Vec<Option<Position>>>;

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

fn create_grid(positions: &Vec<Position>) -> Grid {
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

fn find_edge_points(grid: &Grid) -> HashSet<Position> {
    let mut positions = HashSet::new();

    for row_index in 0..grid.len() {
        let row = &grid[row_index];

        // the first and last rows are entirely edge
        if row_index == 0 || row_index == grid.len() - 1 {
            // push every point
            for column_index in 0..row.len() {
                if let Some(p) = &grid[row_index][column_index] {
                    positions.insert(*p);
                }
            }
        } else {
            // other rows only push first and last element
            for column_index in vec![0usize, row.len() - 1] {
                if let Some(p) = &grid[row_index][column_index] {
                    positions.insert(*p);
                }
            }
        }
    }

    positions
}

fn count_point_instances_from(grid: &Grid) -> HashMap<Position, u32> {
    grid.iter()
        .flat_map(|row | { row })
        .fold(HashMap::new(), |mut acc, position| {
            if let Some(p) = position {
                let count = acc.entry(*p).or_insert(0);
                *count += 1;
            }

            acc
        })
}

fn part1_solve(positions: &Vec<Position>) -> u32 {
    let grid = create_grid(positions);
    let counts = count_point_instances_from(&grid);
    let edge_points = find_edge_points(&grid);

    counts.iter()
        .filter(|(position, _count)| {
            !edge_points.contains(position)
        })
        .map(|(position, count)| *count)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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

        assert_eq!(
            vec![
                vec![Some(p1), None],
                vec![None, Some(p2)]
            ],
            grid
        );
    }

    #[test]
    fn create_grid_medium() {
        let p1 = Position {
            x: 0, y: 0
        };

        let p2 = Position {
            x: 2, y: 2
        };

        let positions = vec![p1, p2];

        let grid = create_grid(&positions);

        assert_eq!(
            vec![
                vec![Some(p1), Some(p1), None],
                vec![Some(p1), None, Some(p2)],
                vec![None, Some(p2), Some(p2)],
            ],
            grid
        );
    }

    #[test]
    fn create_grid_three() {
        let p1 = Position {
            x: 0, y: 0
        };

        let p2 = Position {
            x: 2, y: 2
        };

        let p3 = Position {
            x: 1, y: 1
        };

        let positions = vec![p1, p2, p3];

        let grid = create_grid(&positions);

        assert_eq!(
            vec![
                vec![Some(p1), None, None],
                vec![None, Some(p3), None],
                vec![None, None, Some(p2)],
            ],
            grid
        );
    }

    #[test]
    fn find_edge_points_five() {
        let p1 = Position {
            x: 0, y: 0
        };

        let p2 = Position {
            x: 1, y: 2
        };

        let p3 = Position {
            x: 1, y: 1
        };

        let p4 = Position {
            x: 2, y: 0
        };

        let p5 = Position {
            x: 2, y: 1
        };

        let grid = vec![
            vec![Some(p1), None, Some(p4)],
            vec![None, Some(p3), Some(p5)],
            vec![None, None, Some(p2)],
        ];

        let mut expected = HashSet::new();
        expected.insert(p1);
        expected.insert(p2);
        expected.insert(p4);
        expected.insert(p5);

        assert_eq!(expected, find_edge_points(&grid));
    }

    #[test]
    fn part1_example() {
        let input = CompleteStr("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9");
        let positions = positions(input).unwrap().1;

        let grid = create_grid(&positions);

        let counts = count_point_instances_from(&grid);

        assert_eq!(17, *counts.get(&Position {x: 5, y: 5}).unwrap());

        assert_eq!(17, part1_solve(&positions));
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("./src/day06/input.txt").unwrap();
        let positions = positions(CompleteStr(&input)).unwrap().1;

        assert_eq!(5532, part1_solve(&positions));
    }

}
