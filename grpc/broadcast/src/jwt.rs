lazy_static::lazy_static! {
    static ref SECRET: String = "social-engineering".to_string();
}

pub fn signing(data: impl AsRef<str>) -> String {
    let mut sp: std::str::Split<'_, &str> = data.as_ref().split("+");
    let token = format!("{}+{}", sp.next().unwrap(), sp.next().unwrap());

    token
}

pub fn parse(token_str: &impl AsRef<str>) -> (String, String) {
    let token_str = token_str.as_ref().strip_prefix("Bearer ").unwrap();
    let mut sp = token_str.split("+");
    (
        sp.next().unwrap().to_string(),
        sp.next().unwrap().to_string(),
    )
}

pub fn verification() -> bool {
    true
}
