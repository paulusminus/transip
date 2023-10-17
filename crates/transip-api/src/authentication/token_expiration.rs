use std::fmt::Display;

#[allow(dead_code)]
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
