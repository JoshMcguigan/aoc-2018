use nom::{named, do_parse, call, error_position, eol, map_res, tag, take, alt, terminated, tuple, tuple_parser, separated_list, eof, recognize};
use nom::types::CompleteStr;

use std::str::FromStr;
use std::collections::HashMap;

// datetime is much better off handled by the Chrono crate in a production application
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8
}

named!(
    parse_u16<CompleteStr, u16>,
    map_res!(take!(4), |CompleteStr(s)| u16::from_str(s))
);

named!(
    parse_u8<CompleteStr, u8>,
    map_res!(take!(2), |CompleteStr(s)| u8::from_str(s))
);

named!(
    dt<CompleteStr, DateTime>,
    do_parse!(
        year: parse_u16 >>
        tag!("-") >>
        month: parse_u8 >>
        tag!("-") >>
        day: parse_u8 >>
        tag!(" ") >>
        hour: parse_u8 >>
        tag!(":") >>
        minute: parse_u8 >>
        ( DateTime {year, month, day, hour, minute} )
    )
);

type GuardId = u16;

#[derive(Debug, PartialEq)]
enum Action {
    FallsAsleep,
    WakesUp,
    StartsShift(GuardId),
}

named!(
    action<CompleteStr, Action>,
    alt!(
        tag!("falls asleep") => { |_| Action::FallsAsleep } |
        tag!("wakes up") => { |_| Action::WakesUp } |
        do_parse!(
            tag!("Guard #") >>
            id: map_res!(recognize!(nom::digit), |CompleteStr(s)| u16::from_str(s)) >>
            tag!(" begins shift") >>
            ( Action::StartsShift(id) )
        )
    )
);

#[derive(Debug, PartialEq)]
struct Event {
    pub datetime: DateTime,
    pub action: Action
}

named!(
    event<CompleteStr, Event>,
    do_parse!(
        tag!("[") >>
        datetime: dt >>
        tag!("] ") >>
        action: action >>
        ( Event { datetime, action } )
    )
);

named!(
    events<CompleteStr, Vec<Event> >,
    separated_list!(eol, event)
);

struct Nap {
    start_min: u8,
    end_min: u8,
}

impl Nap {
    fn duration(&self) -> u8 {
        self.end_min - self.start_min
    }
}

fn calc_naps(mut events: Vec<Event>) -> HashMap<GuardId, Vec<Nap>> {
    let mut naps = HashMap::new();
    events.sort_by(|a, b| a.datetime.cmp(&b.datetime));

    let mut active_guard = match events.get(0).unwrap().action {
        Action::StartsShift(guard_id) => guard_id,
        _ => panic!("first event must be start shift event")
    };

    let mut nap_start_min = 0; // assume data is valid and wakes up is always preceded by falls asleep

    events.iter().for_each(|event| {
        match event.action {
            Action::WakesUp => {
                let naps_for_active_guard = naps
                    .entry(active_guard)
                    .or_insert(vec![]);
                naps_for_active_guard.push(Nap {start_min: nap_start_min, end_min: event.datetime.minute});
            },
            Action::FallsAsleep => nap_start_min = event.datetime.minute,
            Action::StartsShift(guard_id) => active_guard = guard_id,
        };
    });

    naps
}

fn most_time_napping(naps_per_guard: &HashMap<GuardId, Vec<Nap>>) -> GuardId {
    *naps_per_guard.iter()
        .max_by_key(|&(guard_id, naps)| total_nap_time(naps)).unwrap().0
}

fn total_nap_time(naps: &Vec<Nap>) -> u16 {
    naps.iter()
        .fold(0u16, |acc, nap| acc + nap.duration() as u16 )
}

fn most_common_minute(naps: &Vec<Nap>) -> u8 {
    let mut minute_counts = [0u32; 60];

    for nap in naps {
        for min in nap.start_min..nap.end_min {
            minute_counts[min as usize] += 1;
        }
    }

    minute_counts.iter().enumerate().max_by_key(|&(_, &item)| item).unwrap().0 as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_dt() {
        let expected = DateTime {
            year: 1518,
            month: 7,
            day: 31,
            hour: 0,
            minute: 54
        };

        let input = CompleteStr("1518-07-31 00:54");

        let actual = dt(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_action_falls_asleep() {
        let expected = Action::FallsAsleep;
        let input = CompleteStr("falls asleep");

        let actual = action(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_action_wakes_up() {
        let expected = Action::WakesUp;
        let input = CompleteStr("wakes up");

        let actual = action(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_action_starts_shift() {
        let expected = Action::StartsShift(12);
        let input = CompleteStr("Guard #12 begins shift");

        let actual = action(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_event() {
        let expected = Event {
            datetime: DateTime {
                year: 1518,
                month: 7,
                day: 31,
                hour: 0,
                minute: 54
            },
            action: Action::WakesUp
        };
        let input = CompleteStr("[1518-07-31 00:54] wakes up");

        let actual = event(input).unwrap().1;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example() {
        let input = CompleteStr(
"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up");

        let parsed_events : Vec<Event> = events(input).unwrap().1;
        assert_eq!(17, parsed_events.len());

        let naps_per_guard: HashMap<GuardId, Vec<Nap>> = calc_naps(parsed_events);

        let guard_id_most_time_napping = most_time_napping(&naps_per_guard);

        assert_eq!(10, guard_id_most_time_napping);

        let naps_for_guard_ten = naps_per_guard.get(&10).unwrap();

        assert_eq!(50, total_nap_time(naps_for_guard_ten));
        assert_eq!(24, most_common_minute(naps_for_guard_ten));
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("./src/day04/input.txt").unwrap();
        let parsed_events = events(CompleteStr(&input)).unwrap().1;

        let naps_per_guard: HashMap<GuardId, Vec<Nap>> = calc_naps(parsed_events);

        let guard_id_most_time_napping = most_time_napping(&naps_per_guard);

        assert_eq!(1487, guard_id_most_time_napping);

        let naps_for_guard = naps_per_guard.get(&1487).unwrap();

        assert_eq!(551, total_nap_time(naps_for_guard));
        assert_eq!(34, most_common_minute(naps_for_guard));

        assert_eq!(50558, 1487 * 34);
    }
}
