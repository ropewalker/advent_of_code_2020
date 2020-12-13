use aoc_runner_derive::{aoc, aoc_generator};
use modinverse::*;

type Timestamp = i64;
type BusId = i64;

struct BusSchedule {
    shift: Timestamp,
    id: BusId,
}

struct Notes {
    earliest_timestamp: Timestamp,
    bus_schedules: Vec<BusSchedule>,
}

fn parse_bus_schedules(input: &str) -> Vec<BusSchedule> {
    input
        .split(',')
        .enumerate()
        .filter_map(|(index, record)| {
            if let Ok(id) = record.parse::<i64>() {
                Some(BusSchedule {
                    shift: index as i64,
                    id,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Notes {
    let mut lines = input.lines();

    Notes {
        earliest_timestamp: lines.next().unwrap().parse().unwrap(),
        bus_schedules: parse_bus_schedules(lines.next().unwrap()),
    }
}

#[aoc(day13, part1)]
fn part1(notes: &Notes) -> i64 {
    let (id, time) = notes
        .bus_schedules
        .iter()
        .map(|schedule| {
            (
                schedule.id,
                (schedule.id - notes.earliest_timestamp % schedule.id) % schedule.id,
            )
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    id * time
}

fn find_earliest_timestamp(bus_schedules: &[BusSchedule]) -> Timestamp {
    let product = bus_schedules
        .iter()
        .map(|schedule| schedule.id)
        .product::<i64>();

    bus_schedules
        .iter()
        .map(|schedule| {
            let product_without_id = product / schedule.id;
            let modular_inverse = modinverse(product_without_id, schedule.id).unwrap();
            (schedule.id - schedule.shift % schedule.id) * product_without_id * modular_inverse
        })
        .sum::<i64>()
        % product
}

#[aoc(day13, part2)]
fn part2(notes: &Notes) -> Timestamp {
    find_earliest_timestamp(&notes.bus_schedules)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_NOTES: &str = r"939
7,13,x,x,59,x,31,19";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_NOTES)), 295);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_NOTES)), 1_068_781);
        assert_eq!(
            find_earliest_timestamp(&parse_bus_schedules("17,x,13,19")),
            3417
        );
        assert_eq!(
            find_earliest_timestamp(&parse_bus_schedules("67,7,59,61")),
            754_018
        );
        assert_eq!(
            find_earliest_timestamp(&parse_bus_schedules("67,x,7,59,61")),
            779_210
        );
        assert_eq!(
            find_earliest_timestamp(&parse_bus_schedules("67,7,x,59,61")),
            1_261_476
        );
        assert_eq!(
            find_earliest_timestamp(&parse_bus_schedules("1789,37,47,1889")),
            1_202_161_486
        );
    }
}
