use anyhow::{anyhow, bail, Result};

use super::{Height, Passport};

impl Passport {
    /// Parse out a Passport entry from a string.
    ///
    /// Returns either the passport and the remainder of the string it did not parse,
    /// or an error.
    pub fn parse(i: &str) -> Result<(Self, &str)> {
        let mut i = i;
        // default = all is NONE
        let mut out = Passport::default();

        while !i.is_empty() && i.find(|c: char| !c.is_whitespace()).is_some() {
            // find the key
            let (key, new_i) = i.split_at(3);
            // Expect a colon
            let (this_better_be_a_colon, new_i) = new_i.split_at(1);
            if this_better_be_a_colon != ":" {
                bail!("there was no colon");
            }
            i = new_i;
            // do the field
            let ws_idx = i
                .find(char::is_whitespace)
                .ok_or_else(|| anyhow!("End of file reached from parse_int"))?;
            let (value, new_i) = i.split_at(ws_idx);
            i = new_i;
            let value = value.to_string();
            // match out the key
            let out_field = match key {
                "byr" => &mut out.birth_year,
                "iyr" => &mut out.issue_year,
                "eyr" => &mut out.expiration_year,
                "hgt" => &mut out.height,
                "hcl" => &mut out.hair_color,
                "ecl" => &mut out.eye_color,
                "pid" => &mut out.passport_id,
                "cid" => &mut out.country_id,
                oh_no => bail!("Unknown key `{}`", oh_no),
            };
            if out_field.is_some() {
                bail!("Field `{}` was already {:?}!", value, out_field)
            } else {
                *out_field = Some(value);
            }

            // Remove whitespace
            if i.starts_with(char::is_whitespace) {
                let content_idx = match i.find(|c: char| !c.is_whitespace()) {
                    Some(it) => it,
                    None => {
                        // out of content! aka eof
                        // overwrite i to be empty
                        // this is a hack but ehh
                        i = "";
                        break;
                    }
                };
                i = i.get(content_idx..).unwrap();
                if content_idx >= 2 {
                    // this means there were two lines of whitespace
                    // or we're out of input
                    break;
                }
            }
        }

        Ok((out, i))
    }

    /// Parse ALL the passports from a string
    pub fn parse_all(i: &str) -> Result<Vec<Self>> {
        let mut i = i;
        let mut out = Vec::new();
        while !i.is_empty() {
            let (pp, new_i) = Passport::parse(i)?;
            i = new_i;
            out.push(pp);
        }
        Ok(out)
    }
}

/// parse either base 10 or base 16 with #
fn parse_int(i: &str) -> Result<(u64, &str)> {
    let ws_idx = i
        .find(char::is_whitespace)
        .ok_or_else(|| anyhow!("End of file reached from parse_int"))?;
    let (num_str, i) = i.split_at(ws_idx);
    if num_str.starts_with('#') {
        // base 16 time!
        let num_str = num_str.get(1..).unwrap();
        let num = u64::from_str_radix(num_str, 16)?;
        Ok((num, i))
    } else {
        let num = num_str.parse::<u64>()?;
        Ok((num, i))
    }
}

fn parse_height(i: &str) -> Result<(Height, &str)> {
    let units_idx = i
        .find(|c: char| !c.is_numeric())
        .ok_or_else(|| anyhow!("Units not found in `parse_height`"))?;
    let (num_str, i) = i.split_at(units_idx);
    let num = num_str.parse()?;
    // Find the discriminator
    let (units, i) = i.split_at(2);
    let height = match units {
        "in" => Height::Inches,
        "cm" => Height::Centimeters,
        oh_no => bail!("Unknown units `{}`", oh_no),
    }(num);
    Ok((height, i))
}
