use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default)]
struct PassportEntry {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<PassportEntry> {
    let entries = input.split("\n\n");
    let mut result = Vec::new();

    for entry in entries {
        let mut passport_entry = PassportEntry::default();

        for token in entry.split_whitespace() {
            let mut key_value = token.split(':');
            match key_value.next() {
                Some("byr") => passport_entry.byr = Some(key_value.next().unwrap().to_owned()),
                Some("iyr") => passport_entry.iyr = Some(key_value.next().unwrap().to_owned()),
                Some("eyr") => passport_entry.eyr = Some(key_value.next().unwrap().to_owned()),
                Some("hgt") => passport_entry.hgt = Some(key_value.next().unwrap().to_owned()),
                Some("hcl") => passport_entry.hcl = Some(key_value.next().unwrap().to_owned()),
                Some("ecl") => passport_entry.ecl = Some(key_value.next().unwrap().to_owned()),
                Some("pid") => passport_entry.pid = Some(key_value.next().unwrap().to_owned()),
                Some("cid") => (),
                _ => unreachable!(),
            }
        }

        result.push(passport_entry);
    }

    result
}

fn validate_password_part1(entry: &PassportEntry) -> bool {
    entry.byr.is_some()
        && entry.iyr.is_some()
        && entry.eyr.is_some()
        && entry.hgt.is_some()
        && entry.hcl.is_some()
        && entry.ecl.is_some()
        && entry.pid.is_some()
}

#[aoc(day4, part1)]
fn part1(entries: &[PassportEntry]) -> usize {
    entries
        .iter()
        .filter(|&entry| validate_password_part1(entry))
        .count()
}

fn validate_yr(maybe_year: &Option<String>, min_year: u32, max_year: u32) -> bool {
    lazy_static! {
        static ref YR_REGEX: Regex = Regex::new(r"^(?x)\d{4}$").unwrap();
    }

    maybe_year
        .as_ref()
        .filter(|yr| YR_REGEX.is_match(yr))
        .filter(|yr| {
            let year = yr.parse::<u32>().unwrap();
            (min_year..=max_year).contains(&year)
        })
        .is_some()
}

fn validate_byr(byr: &Option<String>) -> bool {
    validate_yr(byr, 1920, 2002)
}

fn validate_iyr(iyr: &Option<String>) -> bool {
    validate_yr(iyr, 2010, 2020)
}

fn validate_eyr(eyr: &Option<String>) -> bool {
    validate_yr(eyr, 2020, 2030)
}

fn validate_hgt(hgt: &Option<String>) -> bool {
    lazy_static! {
        static ref HGT_REGEX: Regex = Regex::new(r"^\d*(cm|in)$").unwrap();
    }

    hgt.as_ref()
        .filter(|hgt| HGT_REGEX.is_match(hgt))
        .filter(|hgt| {
            let height = hgt[0..hgt.len() - 2].parse::<i32>().unwrap();

            match &hgt[hgt.len() - 2..hgt.len()] {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => unreachable!(),
            }
        })
        .is_some()
}

fn validate_hcl(hcl: &Option<String>) -> bool {
    lazy_static! {
        static ref HCL_REGEX: Regex = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
    }

    hcl.as_ref().filter(|clr| HCL_REGEX.is_match(clr)).is_some()
}

fn validate_ecl(ecl: &Option<String>) -> bool {
    lazy_static! {
        static ref ECL_REGEX: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    }

    ecl.as_ref().filter(|clr| ECL_REGEX.is_match(clr)).is_some()
}

fn validate_pid(pid: &Option<String>) -> bool {
    lazy_static! {
        static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    pid.as_ref().filter(|pid| PID_REGEX.is_match(pid)).is_some()
}

fn validate_password_part2(entry: &PassportEntry) -> bool {
    validate_byr(&entry.byr)
        && validate_iyr(&entry.iyr)
        && validate_eyr(&entry.eyr)
        && validate_hgt(&entry.hgt)
        && validate_hcl(&entry.hcl)
        && validate_ecl(&entry.ecl)
        && validate_pid(&entry.pid)
}

#[aoc(day4, part2)]
fn part2(entries: &[PassportEntry]) -> usize {
    entries
        .iter()
        .filter(|&entry| validate_password_part2(entry))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert!(validate_password_part1(
            &parse_input(
                r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"
            )[0]
        ));
        assert!(!validate_password_part1(
            &parse_input(
                r"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"
            )[0]
        ));
        assert!(validate_password_part1(
            &parse_input(
                r"hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm"
            )[0]
        ));
        assert!(!validate_password_part1(
            &parse_input(
                r"hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            )[0]
        ));
        assert_eq!(
            part1(&parse_input(
                r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert!(validate_byr(&Some("2002".to_string())));
        assert!(!validate_byr(&Some("2003".to_string())));

        assert!(validate_hgt(&Some("60in".to_string())));
        assert!(validate_hgt(&Some("190cm".to_string())));
        assert!(!validate_hgt(&Some("190in".to_string())));
        assert!(!validate_hgt(&Some("190".to_string())));

        assert!(validate_hcl(&Some("#123abc".to_string())));
        assert!(!validate_hcl(&Some("#123abz".to_string())));
        assert!(!validate_hcl(&Some("123abc".to_string())));

        assert!(validate_ecl(&Some("brn".to_string())));
        assert!(!validate_ecl(&Some("wat".to_string())));

        assert!(validate_pid(&Some("000000001".to_string())));
        assert!(!validate_pid(&Some("0123456789".to_string())));

        assert!(!validate_password_part2(
            &parse_input(
                r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
            )[0]
        ));
        assert!(!validate_password_part2(
            &parse_input(
                r"iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946"
            )[0]
        ));
        assert!(!validate_password_part2(
            &parse_input(
                r"hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
            )[0]
        ));
        assert!(!validate_password_part2(
            &parse_input(
                r"hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            )[0]
        ));

        assert!(validate_password_part2(
            &parse_input(
                r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
            )[0]
        ));
        assert!(validate_password_part2(
            &parse_input(
                r"eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
            )[0]
        ));
        assert!(validate_password_part2(
            &parse_input(
                r"hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022"
            )[0]
        ));
        assert!(validate_password_part2(
            &parse_input(r"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")
                [0]
        ));
    }
}
