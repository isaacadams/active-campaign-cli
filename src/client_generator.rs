#[macro_export]
///
/// easily creates an api client
///
/// ```rust
/// generate_reqwest_client!(ApiClient, {
///     user {
///         get_by_id: get "user/{id}" id: &str,
///         delete_by_id: delete "user/{id}" id: &str,
///         create: post "user",
///         list: get "users"
///     },
///     contact {
///         get_by_id: get "contact/{id}" id: &str,
///         delete_by_id: delete "contact/{id}" id: &str,
///         create: post "contact",
///         list: get "contact"
///     }
/// });
/// ```
///
macro_rules! generate_reqwest_client {
    ($client_type:ident, {
        $(
            $resource:ident {
                $(
                    $name:ident: $method:ident $url:literal $($param:ident : $type:ty)*
                ),+
             }
        ),+
    }) => {
        /* pub struct ApiClientHelper {
            base_url: String,
            client: reqwest::blocking::Client,
        } */

        paste::paste! {

            /* pub mod [<$client_type:snake>] {
                $(
                    pub mod [<$resource:snake>] {
                        $(
                            pub fn [<$name:snake>](helper: &super::super::ApiClientHelper $(, $param: $type)*) -> reqwest::blocking::RequestBuilder {
                                let url = format!(concat!("{}/", $url), helper.base_url $(, $param = $param)*);
                                helper.client.$method(&url)
                            }
                        )+
                    }
                ),+
            } */

            pub struct [<$client_type Builder>] {
                base_url: String,
                client: reqwest::blocking::Client,
            }

            impl [<$client_type Builder>] {
                pub fn new(base_url: &str, client: Option<reqwest::blocking::Client>) -> Self {
                    Self {
                        base_url: base_url.to_string(),
                        client: client.unwrap_or(reqwest::blocking::Client::new()),
                    }
                }

                $(
                    $(
                        pub fn [<$resource _ $name>](&self $(, $param: $type)*) -> reqwest::blocking::RequestBuilder {
                            let url = format!(concat!("{}/", $url), self.base_url $(, $param = $param)*);
                            println!("{} {}", stringify!([<$method:upper>]), &url);
                            self.client.$method(&url)
                        }
                    )+
                )+
            }
        }
    };
}
