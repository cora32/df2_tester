#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate reqwest;
use reqwest::Client;
use reqwest::Error;
use std::env;
use uuid::Uuid;

const REGISTER_URL: &str = "/register";
const GET_CARDS_URL: &str = "/df2/cards";

struct Credentials {
    user: String,
    pass: String,
}

fn get_creds() -> Credentials {
    Credentials {
        user: env::var("DF2_USER").expect("DF2_USER not found"),
        pass: env::var("DF2_PASS").expect("DF2_PASS not found"),
    }
}

// Main trait

trait Tester {
    fn new(url: String) -> Self;
    fn test(&self, uuid: String) -> Result<String, TestError>;
    fn say_ok(&self, code: &'static str) {
        println!("{}: OK", code);
    }
    fn say_failed(&self, error: TestError) {
        println!(
            "{}\nrequest: ```{}```\nerror: `{:?}`",
            error.url, error.request, error.error
        );
    }
}

#[derive(Debug)]
struct TestError {
    url: String,
    request: String,
    error: String,
}

// GetCardsTest test impl

#[derive(Deserialize, Debug)]
struct GetCardsResponse {
    scaleId: String,
}

struct GetCardsTest {
    url: String,
}

impl GetCardsTest {
    const VERSION: &'static str = "0";
}

impl Tester for GetCardsTest {
    fn new(url: String) -> GetCardsTest {
        GetCardsTest { url }
    }

    fn test(&self, uuid: String) -> Result<String, TestError> {
        let creds = get_creds();

        let body = json!({
            "uuid": uuid,
            "version": GetCardsTest::VERSION
        });

        println!("==> {}\n {}", self.url, body);
        let mut response = match Client::new()
            .post(&self.url)
            .basic_auth(creds.user, Some(creds.pass))
            .json(&body)
            .send()
        {
            Ok(resp) => resp,
            Err(e) => {
                return Err(TestError {
                    url: self.url.clone(),
                    request: format!("{:?}", body.to_string()),
                    error: format!("Failed to send: {:?}", e).to_owned(),
                })
            }
        };

        if response.status().is_success() {
            // server does not return content_length
            // match response.content_length() {
            //     Some(length) => {
            // if length == 0 {
            //     return Err(TestError {
            //         url: self.url,
            //         request: format!("{:?}", body.to_string()),
            //         error: format!("content_length == 0; status: {:?}", response.status())
            //             .to_owned(),
            //     });
            // } else {
            // let resp: RegisterResponse = match response.json() {
            //     Ok(json) => json,
            //     Err(e) => {
            //         return Err(TestError {
            //             url: self.url,
            //             request: format!("{:?}", body.to_string()),
            //             error: format!(
            //                 "Failed to parse json: {:?}\n response: {}",
            //                 e,
            //                 response.text().unwrap()
            //             )
            //             .to_owned(),
            //         })
            //     }
            // };
            // println!("<== {:?}", resp);

            // return Ok(uuid);
            // }
            //     }
            //     None => {
            //         return Err(TestError {
            //             url: self.url,
            //             request: format!("{:?}", body.to_string()),
            //             error: format!("content_length == null; status: {:?}", response.status())
            //                 .to_owned(),
            //         })
            //     }
            // }

            let resp: Vec<GetCardsResponse> = match response.json() {
                Ok(json) => json,
                Err(e) => {
                    return Err(TestError {
                        url: self.url.clone(),
                        request: format!("{:?}", body.to_string()),
                        error: format!(
                            "Failed to parse json: {:?}\n response: {}",
                            e,
                            response.text().unwrap()
                        )
                        .to_owned(),
                    })
                }
            };
            println!("<== {:?}", resp);

            return Ok(uuid);
        } else if response.status().is_server_error() {
            return Err(TestError {
                url: self.url.clone(),
                request: format!("{:?}", body.to_string()),
                error: format!("Server error: {:?}", response.status()).to_owned(),
            });
        } else {
            return Err(TestError {
                url: self.url.clone(),
                request: format!("{:?}", body.to_string()),
                error: format!("Unknsown error: {:?}", response.status()).to_owned(),
            });
        }
    }
}

// Register tester impl

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    secret: String,
}

struct RegisterTest {
    url: String,
}

impl RegisterTest {
    const BUILD_TYPE: &'static str = "tester";
    const UUID: &'static str = "00000000-0000-0000-0000-000000000000";
    const VERSION: &'static str = "0";
}

impl Tester for RegisterTest {
    fn new(url: String) -> RegisterTest {
        RegisterTest { url }
    }

    fn test(&self, uuid: String) -> Result<String, TestError> {
        let creds = get_creds();

        let body = json!({
            "buildType": RegisterTest::BUILD_TYPE,
            "uuid": uuid,
            "version": RegisterTest::VERSION
        });

        println!("==> {}\n {}", self.url, body);
        let mut response = match Client::new()
            .post(&self.url)
            .basic_auth(creds.user, Some(creds.pass))
            .json(&body)
            .send()
        {
            Ok(resp) => resp,
            Err(e) => {
                return Err(TestError {
                    url: self.url.clone(),
                    request: format!("{:?}", body.to_string()),
                    error: format!("Failed to send: {:?}", e).to_owned(),
                })
            }
        };

        if response.status().is_success() {
            let resp: RegisterResponse = match response.json() {
                Ok(json) => json,
                Err(e) => {
                    return Err(TestError {
                        url: self.url.clone(),
                        request: format!("{:?}", body.to_string()),
                        error: format!(
                            "Failed to parse json: {:?}\n response: {}",
                            e,
                            response.text().unwrap()
                        )
                        .to_owned(),
                    })
                }
            };
            println!("<== {:?}", resp);

            Ok(resp.secret)
        } else if response.status().is_server_error() {
            return Err(TestError {
                url: self.url.clone(),
                request: format!("{:?}", body.to_string()),
                error: format!("Server error: {:?}", response.status()).to_owned(),
            });
        } else {
            return Err(TestError {
                url: self.url.clone(),
                request: format!("{:?}", body.to_string()),
                error: format!("Unknsown error: {:?}", response.status()).to_owned(),
            });
        }
    }
}

///

fn main() {
    let base_url = env::var("DF2_BASE_URL").expect("DF2_BASE_URL not found");
    let register_test: RegisterTest = Tester::new(format!("{}/{}", base_url, REGISTER_URL));
    let get_cards_test: GetCardsTest = Tester::new(format!("{}/{}", base_url, GET_CARDS_URL));

    let uuid = Uuid::new_v4();
    // let uuid = RegisterTest::UUID;

    match register_test.test(uuid.to_string().to_owned()) {
        Ok(_secret) => {
            register_test.say_ok(REGISTER_URL);

            match get_cards_test.test(uuid.to_string().to_owned()) {
                Ok(_result) => get_cards_test.say_ok(GET_CARDS_URL),
                Err(e) => get_cards_test.say_failed(e),
            }
        }
        Err(e) => {
            register_test.say_failed(e);
        }
    };
}
