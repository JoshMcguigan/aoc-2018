type MarbleValue = u64;
type Score = u64;

struct MarbleCircle {
    pub circle: Vec<MarbleValue>,
    pub active_marble_index: usize,
    pub last_marble_placed: MarbleValue,
}

impl MarbleCircle {
    fn new() -> Self {
        MarbleCircle {
            circle: vec![0],
            active_marble_index: 0,
            last_marble_placed: 0,
        }
    }

    /// returns the points scored on that turn
    fn place_next_marble(&mut self) -> Score {
        let marble_to_be_placed = self.last_marble_placed + 1;
        let score = if marble_to_be_placed % 23 == 0 {
            let removed_marble_index = {
                let removed_marble_index = self.active_marble_index as i64 - 7;
                if removed_marble_index < 0 { self.circle.len() as i64 + removed_marble_index }
                    else { removed_marble_index }
            } as usize;
            let removed_marble = self.circle.remove(removed_marble_index);

            self.active_marble_index = removed_marble_index;

            marble_to_be_placed + removed_marble
        } else {
            let next_marble_index = {
                let next_marble_index = self.active_marble_index + 2;
                if next_marble_index > self.circle.len() {
                    next_marble_index - self.circle.len()
                } else {
                    next_marble_index
                }
            };
            self.circle.insert(next_marble_index, marble_to_be_placed);
            self.active_marble_index = next_marble_index;

            0
        };

        self.last_marble_placed = marble_to_be_placed;

        score
    }
}

fn part1_impl(num_players: usize, max_marble_value: u64) -> Score {
    let mut marble_circle = MarbleCircle::new();
    let mut players = vec![0u64; num_players];
    let mut active_player = 0usize;
    for i in 0..max_marble_value {
        players[active_player] += marble_circle.place_next_marble();
        active_player = (active_player + 1) % players.len();
    }

    *players.iter().max().unwrap()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn marble_circle_new() {
        let marble_circle = MarbleCircle::new();
        assert_eq!(vec![0], marble_circle.circle);
        assert_eq!(0, marble_circle.active_marble_index);
        assert_eq!(0, marble_circle.last_marble_placed);
    }

    #[test]
    fn marble_circle_add_marble() {
        let mut marble_circle = MarbleCircle::new();
        let points = marble_circle.place_next_marble();

        assert_eq!(0, points);
        assert_eq!(vec![0, 1], marble_circle.circle);
        assert_eq!(1, marble_circle.active_marble_index);
        assert_eq!(1, marble_circle.last_marble_placed);

        let points = marble_circle.place_next_marble();

        assert_eq!(0, points);
        assert_eq!(vec![0, 2, 1], marble_circle.circle);
        assert_eq!(1, marble_circle.active_marble_index);
        assert_eq!(2, marble_circle.last_marble_placed);

        let points = marble_circle.place_next_marble();

        assert_eq!(0, points);
        assert_eq!(vec![0, 2, 1, 3], marble_circle.circle);
    }

    #[test]
    fn marble_circle_add_marble_23() {
        let mut marble_circle = MarbleCircle::new();
        for i in 0..22 {
            marble_circle.place_next_marble(); // place the first 22 marbles
        }

        let points = marble_circle.place_next_marble();

        assert_eq!(32, points);
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            marble_circle.circle
        );
        assert_eq!(6, marble_circle.active_marble_index);
        assert_eq!(23, marble_circle.last_marble_placed);
    }

    #[test]
    fn part1_example() {
        assert_eq!(32u64, part1_impl(9, 25));
    }

    #[test]
    fn part1() {
        assert_eq!(408679u64, part1_impl(424, 71482));
    }
}
