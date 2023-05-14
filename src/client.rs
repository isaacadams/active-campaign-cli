use crate::ac_builder::ActiveCampaignBuilder;
use reqwest::{blocking::Body, header, StatusCode};

/// <https://developers.activecampaign.com/reference/overview>
pub fn init() -> Client {
    Client::default()
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

fn create_builder() -> ActiveCampaignBuilder {
    ActiveCampaignBuilder::new(
        &crate::config::load_env_var("ACTIVECAMPAIGN_API_BASE_URL"),
        Some(init_client()),
    )
}

pub struct Client {
    builder: ActiveCampaignBuilder,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            builder: create_builder(),
        }
    }
}

impl Client {
    /// <https://developers.activecampaign.com/reference/list-all-contacts>
    ///
    /// ```
    /// let client = active_campaign::new();
    /// let response = client.contacts_list().unwrap();
    /// assert_eq!(response.status(), reqwest::StatusCode::OK);
    /// ```
    pub fn contacts_list(&self) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder.contact_search().send()
    }

    pub fn contact_find_by_email(
        &self,
        email: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder
            .contact_search()
            .query(&[("email", email)])
            .send()
    }

    /// <https://developers.activecampaign.com/reference/create-a-new-contact>
    pub fn contact_create<T: Into<Body>>(
        &self,
        payload: T,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder.contact_create().body(payload).send()
    }

    /// <https://developers.activecampaign.com/reference/delete-contact>
    pub fn contact_delete(&self, id: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder.contact_delete(id).send()
    }

    /// <https://developers.activecampaign.com/reference/sync-a-contacts-data>
    pub fn contact_sync<T: Into<Body>>(
        &self,
        payload: T,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.builder.contact_sync().body(payload).send()
    }
}

pub fn find_and_delete_by_email(client: &Client, email: &str) -> Result<(), reqwest::Error> {
    let response = client.contact_find_by_email(email)?;

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

    client.contact_delete(id)?;

    println!("{} was deleted!", email);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::*;
    use reqwest::StatusCode;

    /* #[test]
    fn try_to_break() {
        let client = init();

        let create = || {
            let contact = Contact::default();
            let payload = contact.to_request().unwrap();
            let response = client.contact_create(payload).unwrap();
            dbg!(response.text().unwrap());
        };

        create();
        create();
    } */

    #[test]
    fn list_contacts() {
        let client = init();
        let response = client.contacts_list().unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn find_contact_by_email() {
        let client = init();
        let response = client.contact_find_by_email("test@test.com").unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn create_and_delete_contact() {
        let client = init();

        let contact = Contact::default();

        // if contact already exists, then delete
        find_and_delete_by_email(&client, &contact.email).unwrap();

        let payload = contact.to_request().unwrap();
        let response = client.contact_create(payload).unwrap();

        match response.status() {
            StatusCode::CREATED => {
                assert!(true);
                let data = response.json::<serde_json::Value>().unwrap();

                // delete the new contact for cleanup
                if let Some(id) = data["contact"]["id"].as_str() {
                    assert!(
                        client.contact_delete(id).is_ok(),
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

    #[test]
    fn contact_sync_works() {
        let contact = Contact {
            email: "".to_string(),
            first_name: Some("".to_string()),
            last_name: None,
            phone: None,
            field_values: None,
        };

        let client = init();
    }
}
