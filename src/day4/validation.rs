use super::Passport;
use anyhow::Result;

impl Passport {
    /// Validate this passport as per part 2.
    /// returns `true` if it's valid, `false` otherwise
    pub fn validate_part2(&self) -> bool {
        macro_rules! unwrap_to_bool {
            ( $( $field:ident ),* ) => {
                $(
                    let $field = match self.$field {
                        Some(ref it) => it,
                        None => return false,
                    };
                )*
            }
        }
        unwrap_to_bool!(
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id
        );

        validate_year(&birth_year, 1920, 2002)
            && validate_year(&issue_year, 2010, 2020)
            && validate_year(&expiration_year, 2020, 2030)
            && validate_height(&height)
            && validate_hair(&hair_color)
            && validate_eye(&eye_color)
            && validate_pid(&passport_id)
    }
}

fn validate_year(year: &str, min: u16, max: u16) -> bool {
    let year: u16 = match year.parse() {
        Ok(it) => it,
        Err(_heck) => return false,
    };
    min <= year && year <= max
}

fn validate_height(height: &str) -> bool {
    let units_idx = match height.find(|c: char| !c.is_numeric()) {
        Some(it) => it,
        None => {
            return false;
        }
    };
    let (num_str, units) = height.split_at(units_idx);
    let (min, max) = match units {
        "in" => (59, 76),
        "cm" => (150, 193),
        _ => return false,
    };
    let num: u16 = match num_str.parse() {
        Ok(it) => it,
        Err(_) => return false,
    };
    min <= num && num <= max
}

fn validate_hair(hair: &str) -> bool {
    let (this_better_be_a_hash, color) = hair.split_at(1);
    if this_better_be_a_hash != "#" {
        return false;
    }
    u64::from_str_radix(color, 16).is_ok()
}

fn validate_eye(eye: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye)
}

fn validate_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.parse::<u64>().is_ok()
}

#[test]
fn test_invalids() -> Result<()> {
    let invalids_str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;
    let invalids = Passport::parse_all(invalids_str)?;
    for iv in invalids {
        assert!(!iv.validate_part2());
    }

    Ok(())
}

#[test]
fn test_valids() -> Result<()> {
    let valids_str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
    let valids = Passport::parse_all(valids_str)?;
    for valid in valids {
        assert!(valid.validate_part2());
    }

    Ok(())
}
