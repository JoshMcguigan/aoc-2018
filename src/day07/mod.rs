use nom::{named, do_parse, separated_list, call, error_position, eol, map_res, tag, recognize, take, parse_to};
use nom::types::CompleteStr;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

type Step = char;
type Prerequisite = char;
type StepPrerequisites = Vec<Prerequisite>;

named!(
    take_char<CompleteStr, char>,
    map_res!(take!(1), |CompleteStr(s)| char::from_str(s))
);

named!(
    prereq<CompleteStr, (Step, Prerequisite)>,
    do_parse!(
        tag!("Step ") >>
        prereq: take_char >>
        tag!(" must be finished before step ") >>
        step: take_char >>
        tag!(" can begin.") >>
        ( (step, prereq) )
    )
);

named!(
    prereqs<CompleteStr, Vec<(Step, Prerequisite)> >,
    separated_list!(eol, prereq)
);

struct Steps {
    completed: HashSet<Step>,
    prerequisites: HashMap<Step, StepPrerequisites>,
}

impl Steps {

    fn new(prerequisites: HashMap<Step, StepPrerequisites>) -> Self {
        Steps {
            completed: HashSet::new(),
            prerequisites,
        }
    }

    /// Performs a step and returns the step which was performed
    /// If no steps are possible, or all steps are complete, returns None
    fn perform_step(&mut self) -> Option<Step> {
        let non_completed_steps = self.prerequisites
            .iter()
            .filter(|(step, _prereqs)| {
                !self.completed.contains(step)
            });

        let non_completed_steps_with_all_prerequisites_satisfied = non_completed_steps
            .filter(|(_step, prerequisities)| {
                prerequisities.iter().all(|prereq| self.completed.contains(prereq))
            });

        let step_to_perform = non_completed_steps_with_all_prerequisites_satisfied
            .map(|(step, _prereqs)| *step)
            .min();

        if let Some(step) = step_to_perform { self.completed.insert(step); }

        step_to_perform
    }
}

fn generate_prereq_map(prereqs: &Vec<(Step, Prerequisite)>) -> HashMap<Step, StepPrerequisites> {
    let mut prereq_map = HashMap::new();

    for (step, prereq) in prereqs {
        let prereqs = prereq_map.entry(*step).or_insert(vec![]);
        prereqs.push(*prereq);

        if !prereq_map.contains_key(prereq) { prereq_map.insert(*prereq, vec![]); }
    }

    prereq_map
}

fn part1_impl(input: &str) -> String {
    let prereqs = prereqs(CompleteStr(input)).unwrap().1;
    let prereq_map = generate_prereq_map(&prereqs);

    let mut steps = Steps::new(prereq_map);
    let mut step_tracker = vec![];

    while let Some(completed_step) = steps.perform_step() {
        step_tracker.push(completed_step);
    }

    step_tracker.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_prereq() {
        let prereq = prereq(CompleteStr("Step C must be finished before step A can begin."))
            .unwrap().1;

        assert_eq!('A', prereq.0);
        assert_eq!('C', prereq.1);
    }

    #[test]
    fn parse_prereqs() {
        let input = fs::read_to_string("./src/day07/input-example.txt").unwrap();
        let prereqs = prereqs(CompleteStr(&input)).unwrap().1;

        assert_eq!(7, prereqs.len());

        let prereq_map = generate_prereq_map(&prereqs);

        assert_eq!(Some(&vec!['C']), prereq_map.get(&'A'));
        assert_eq!(Some(&vec![]), prereq_map.get(&'C'));
    }

    #[test]
    fn perform_step_no_steps() {
        let mut steps = Steps::new(HashMap::new());
        assert_eq!(None, steps.perform_step());
    }

    #[test]
    fn part1_example() {
        let input = fs::read_to_string("./src/day07/input-example.txt").unwrap();
        assert_eq!(String::from("CABDFE"), part1_impl(&input));
    }

    #[test]
    fn part1() {
        let input = fs::read_to_string("./src/day07/input.txt").unwrap();
        assert_eq!(String::from("BHMOTUFLCPQKWINZVRXAJDSYEG"), part1_impl(&input));
    }

}
