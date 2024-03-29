extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate reqwest;
use crate::base_tester::get_creds;
use crate::base_tester::TestError;
use crate::base_tester::Tester;
use crate::card_tester::GetCardsTest;
use reqwest::Client;
use std::env;
#[allow(unused_imports)]
use uuid::Uuid;

mod base_tester;
mod card_tester;

#[allow(dead_code)]
pub const REGISTER_URL: &str = "/register";
pub const GET_CARDS_URL: &str = "/df2/cards";

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
    // 10203d05285ad29de94d5c705cb8872d
    const UUID: &'static str = "00000000-0000-0000-0000-000000000002";
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
    // let register_test: RegisterTest = Tester::new(format!("{}/{}", base_url, REGISTER_URL));
    let get_cards_test: GetCardsTest = Tester::new(format!("{}/{}", base_url, GET_CARDS_URL));

    // let uuid = Uuid::new_v4();
    let uuid = RegisterTest::UUID;

    // match register_test.test(uuid.to_string().to_owned()) {
    //     Ok(_secret) => {
    //         register_test.say_ok(REGISTER_URL);

    //         match get_cards_test.test(uuid.to_string().to_owned()) {
    //             Ok(_result) => get_cards_test.say_ok(GET_CARDS_URL),
    //             Err(e) => get_cards_test.say_failed(e),
    //         }
    //     }
    //     Err(e) => {
    //         register_test.say_failed(e);
    //     }
    // };

    match get_cards_test.test(uuid.to_string().to_owned()) {
        Ok(_result) => get_cards_test.say_ok(GET_CARDS_URL),
        Err(e) => get_cards_test.say_failed(e),
    };
}
