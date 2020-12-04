use anyhow::Result;

mod parsing;
mod validation;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Default)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    /// Hex color
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

#[derive(Debug)]
enum Height {
    Inches(u16),
    Centimeters(u16),
}

/*
[
    &pp.birth_year,
    &pp.country_id,
    &pp.expiration_year,
    &pp.eye_color,
    &pp.hair_color,
    &pp.height,
    &pp.issue_year,
    &pp.passport_id,
]
*/

#[test]
fn part1() -> Result<()> {
    let passports = Passport::parse_all(INPUT)?;

    // Count how many have Some for all *except* cid
    let valid_count = passports
        .iter()
        .filter(|pp| {
            [
                &pp.birth_year,
                &pp.expiration_year,
                &pp.eye_color,
                &pp.hair_color,
                &pp.height,
                &pp.issue_year,
                &pp.passport_id,
            ]
            .iter()
            .all(|field| field.is_some())
        })
        .count();
    println!("valid count: {}", valid_count);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let passports = Passport::parse_all(INPUT)?;
    let valid_count = passports.iter().filter(|pp| pp.validate_part2()).count();
    println!("valid count: {}", valid_count);

    Ok(())
}
