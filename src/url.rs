const AUTH: &str = "auth";

pub struct Url {
    pub prefix: String,
}

impl Url {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }

    pub fn auth(&self) -> String {
        format!("{}{}", self.prefix, AUTH)
    }
}
