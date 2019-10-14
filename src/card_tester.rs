extern crate reqwest;
#[macro_use]
use crate::base_tester::get_creds;
use crate::base_tester::TestError;
use crate::base_tester::Tester;
use reqwest::Client;

#[derive(Deserialize, Debug)]
struct GetCardsResponse {
    #[serde(rename = "scaleId")]
    scale_id: String,
    #[serde(rename = "clusterName")]
    cluster_name: String,
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "imageUrl")]
    image_url: String,
    description: String,
}

pub struct GetCardsTest {
    url: String,
}

impl GetCardsTest {
    const VERSION: &'static str = "7110";

    fn check_image(&self, image_url: &str) -> Result<String, TestError> {
        let response = match Client::new().get(image_url).send() {
            Ok(resp) => resp,
            Err(e) => {
                return Err(TestError {
                    url: image_url.to_owned(),
                    request: " ".to_owned(),
                    error: format!("Failed to get image: {:?}", e).to_owned(),
                })
            }
        };

        if response.status().is_success() {
            println!("==> {} [OK]", image_url);
            return Ok(" ".to_owned());
        } else {
            return Err(TestError {
                url: image_url.to_owned(),
                request: " ".to_owned(),
                error: format!("Failed to get image: {:?}", image_url).to_owned(),
            });
        }
    }
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
            let card_list: Vec<GetCardsResponse> = match response.json() {
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

            for card in card_list {
                df2_check_params!(
                    "clusterName"; card.cluster_name.trim(),
                    "scaleId"; card.scale_id.trim(),
                    "description"; card.description.trim(),
                    "contentId"; card.content_id.trim(),
                    "imageUrl"; card.image_url.trim()
                );

                match self.check_image(card.image_url.trim()) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }

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
