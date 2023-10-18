use std::{fmt::Display, str::FromStr};

use crate::Error;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum TokenExpiration {
    Seconds(u8),
    Minutes(u8),
    Hours(u8),
}

fn fmt(count: &u8, unit: &'static str) -> String {
    format!("{} {}", count, unit) + if count <= &1 { "" } else { "s" }
}

impl Display for TokenExpiration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenExpiration::Seconds(seconds) => fmt(seconds, "second"),
                TokenExpiration::Minutes(minutes) => fmt(minutes, "minute"),
                TokenExpiration::Hours(hours) => fmt(hours, "hour"),
            }
        )
    }
}

impl FromStr for TokenExpiration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split(' ').collect::<Vec<_>>();
        if splitted.len() != 2 {
            return Err(Error::ParseExpiration("String should contain 2 words"));
        };
        let first_word = splitted[0].trim().parse::<u8>()?;
        let second_word = splitted[1].trim();

        const IS_SECONDS: [&str; 2] = ["second", "seconds"];
        if IS_SECONDS.contains(&second_word.to_lowercase().as_str()) {
            return Ok(TokenExpiration::Seconds(first_word));
        }

        const IS_MINUTES: [&str; 2] = ["minute", "minutes"];
        if IS_MINUTES.contains(&second_word.to_lowercase().as_str()) {
            return Ok(TokenExpiration::Minutes(first_word));
        }

        const IS_HOURS: [&str; 2] = ["hour", "hours"];
        if IS_HOURS.contains(&second_word.to_lowercase().as_str()) {
            return Ok(TokenExpiration::Hours(first_word));
        }

        Err(Error::ParseExpiration("Invalid"))
    }
}

#[cfg(test)]
mod test {
    use super::TokenExpiration;

    #[test]
    fn seconds_ok() {
        let s = "5 seconds";
        let expiration = s.parse::<TokenExpiration>().unwrap();
        assert_eq!(expiration, TokenExpiration::Seconds(5));
    }

    #[test]
    fn seconds_error() {
        assert!("10 secconds".parse::<TokenExpiration>().is_err());
        assert!("five seconds".parse::<TokenExpiration>().is_err());
    }

    #[test]
    fn minutes_ok() {
        assert_eq!(
            "10 minutes".parse::<TokenExpiration>().unwrap(),
            TokenExpiration::Minutes(10),
        );
    }

    #[test]
    fn minutes_err() {
        assert!("10 minuttes".parse::<TokenExpiration>().is_err());
        assert!("ten minutes".parse::<TokenExpiration>().is_err());
    }

    #[test]
    fn hours_ok() {
        assert_eq!(
            "60 hours".parse::<TokenExpiration>().unwrap(),
            TokenExpiration::Hours(60),
        );
        assert_eq!(
            "60 HOUR".parse::<TokenExpiration>().unwrap(),
            TokenExpiration::Hours(60),
        );
    }

    #[test]
    fn hours_err() {
        assert!("10 uur".parse::<TokenExpiration>().is_err());
        assert!("ten hours".parse::<TokenExpiration>().is_err());
    }
}
