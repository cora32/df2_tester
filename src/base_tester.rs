#![macro_use]
use std::env;

#[macro_export]
macro_rules! df2_check_params {
    ( $( $x:expr; $y:expr),* ) => {{
        $(
        if $y.is_empty() {
            println!("{:?} is empty.", $x);
            return Err(TestError {
                url: " ".to_owned(),
                request: " ".to_owned(),
                error: format!("{:?} is empty.", $x).to_owned(),
            });
        }
        )*
    }};
}

pub fn get_creds() -> Credentials {
    Credentials {
        user: env::var("DF2_USER").expect("DF2_USER not found"),
        pass: env::var("DF2_PASS").expect("DF2_PASS not found"),
    }
}

pub struct Credentials {
    pub user: String,
    pub pass: String,
}

// Main trait

pub trait Tester {
    fn new(url: String) -> Self;
    fn test(&self, uuid: String) -> Result<String, TestError>;
    fn say_ok(&self, code: &'static str) {
        println!("[OK]: {}", code);
    }
    fn say_failed(&self, error: TestError) {
        println!(
            "[FAIL]: {}\nrequest: ```{}```\nerror: `{:?}`",
            error.url, error.request, error.error
        );
    }
}

#[derive(Debug)]
pub struct TestError {
    pub url: String,
    pub request: String,
    pub error: String,
}
