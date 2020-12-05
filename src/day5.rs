use aoc_runner_derive::{aoc, aoc_generator};

struct BoardingPass(usize);

impl From<&str> for BoardingPass {
    fn from(code: &str) -> Self {
        BoardingPass(
            usize::from_str_radix(
                unsafe {
                    std::str::from_utf8_unchecked(
                        &code[0..7]
                            .bytes()
                            .map(|c| match c {
                                b'F' => b'0',
                                b'B' => b'1',
                                _ => unreachable!(),
                            })
                            .chain(code[7..10].bytes().map(|c| match c {
                                b'L' => b'0',
                                b'R' => b'1',
                                _ => unreachable!(),
                            }))
                            .collect::<Vec<_>>(),
                    )
                },
                2,
            )
            .unwrap(),
        )
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<BoardingPass> {
    input.lines().map(BoardingPass::from).collect()
}

#[aoc(day5, part1)]
fn part1(boarding_passes: &[BoardingPass]) -> usize {
    boarding_passes.iter().map(|pass| pass.0).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(boarding_passes: &[BoardingPass]) -> usize {
    let mut ids: Vec<usize> = boarding_passes.iter().map(|pass| pass.0).collect();
    ids.sort_unstable();

    ids[(0..ids.len() - 1)
        .find(|&i| ids[i] + 1 != ids[i + 1])
        .unwrap()]
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    impl BoardingPass {
        fn row(&self) -> usize {
            self.0 / 8
        }

        fn column(&self) -> usize {
            self.0 % 8
        }
    }

    #[test]
    fn part1_example() {
        let boarding_pass = BoardingPass::from("FBFBBFFRLR");

        assert_eq!(boarding_pass.row(), 44);
        assert_eq!(boarding_pass.column(), 5);
        assert_eq!(boarding_pass.0, 357);

        let boarding_pass = BoardingPass::from("BFFFBBFRRR");

        assert_eq!(boarding_pass.row(), 70);
        assert_eq!(boarding_pass.column(), 7);
        assert_eq!(boarding_pass.0, 567);

        let boarding_pass = BoardingPass::from("FFFBBBFRRR");

        assert_eq!(boarding_pass.row(), 14);
        assert_eq!(boarding_pass.column(), 7);
        assert_eq!(boarding_pass.0, 119);

        let boarding_pass = BoardingPass::from("BBFFBBFRLL");

        assert_eq!(boarding_pass.row(), 102);
        assert_eq!(boarding_pass.column(), 4);
        assert_eq!(boarding_pass.0, 820);
    }
}
