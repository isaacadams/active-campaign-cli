use crate::ActiveCampaignBuilder;
use reqwest::{blocking::Body, header, StatusCode};

/// https://developers.activecampaign.com/reference/overview
pub fn init() -> ActiveCampaignApiClient {
    ActiveCampaignApiClient::default()
}

fn init_client() -> reqwest::blocking::Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Api-Token",
        crate::config::load_env_var("ACTIVECAMPAIGN_API_KEY")
            .parse()
            .expect("failing to build active campaign client"),
    );

    reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub struct ActiveCampaignApiClient {
    builder: ActiveCampaignBuilder,
}

impl Default for ActiveCampaignApiClient {
    fn default() -> Self {
        let builder = ActiveCampaignBuilder::new(
            &crate::config::load_env_var("ACTIVECAMPAIGN_API_BASE_URL"),
            Some(init_client()),
        );

        Self { builder }
    }
}

impl ActiveCampaignApiClient {
    /// https://developers.activecampaign.com/reference/list-all-contacts
    pub fn list_contacts(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder.contact_search().send()
    }

    pub fn find_contact_by_email(
        &self,
        email: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder
            .contact_search()
            .query(&[("email", email)])
            .send()
    }

    /// https://developers.activecampaign.com/reference/create-a-new-contact
    pub fn create_contact<T: Into<Body>>(
        &self,
        payload: T,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder.contact_create().body(payload).send()
    }

    /// https://developers.activecampaign.com/reference/delete-contact
    pub fn delete_contact(&self, id: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder.contact_delete(id).send()
    }
}

pub fn find_and_delete_by_email(
    client: &ActiveCampaignApiClient,
    email: &str,
) -> Result<(), reqwest::Error> {
    let response = client.find_contact_by_email(email)?;

    let data = match response.status() {
        StatusCode::OK => response.json::<serde_json::Value>().unwrap(),
        _ => {
            println!("request failed: {}", response.status());
            if let Ok(text) = response.text() {
                println!("{}", text);
            }
            return Ok(());
        }
    };

    let id = match data["contacts"][0]["id"].as_str() {
        Some(id) => id,
        _ => {
            println!("{} could not be found", email);
            return Ok(());
        }
    };

    client.delete_contact(id)?;

    println!("{} was deleted!", email);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::*;
    use reqwest::StatusCode;

    #[test]
    fn list_contacts() {
        let client = init();
        let response = client.list_contacts().unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn find_contact_by_email() {
        let client = init();
        let response = client.find_contact_by_email("test@spotpet.com").unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn create_and_delete_contact() {
        let client = init();

        let contact = Contact::default();

        // if contact already exists, then delete
        find_and_delete_by_email(&client, &contact.email).unwrap();

        let payload = contact.to_request().unwrap();
        let response = client.create_contact(payload).unwrap();

        match response.status() {
            StatusCode::CREATED => {
                assert!(true);
                let data = response.json::<serde_json::Value>().unwrap();

                // delete the new contact for cleanup
                if let Some(id) = data["contact"]["id"].as_str() {
                    assert!(
                        client.delete_contact(id).is_ok(),
                        "failed to delete the contact in cleanup phase"
                    );
                }
            }
            _ => {
                println!("{:#?}", response.text());
                assert!(false);
            }
        }
    }
}
