pub mod settings {
    use std::env;
    use sendgrid::v3::Email;
    use serde::{Deserialize, Serialize};
    use ssql::prelude::tiberius::{AuthMethod, Config, EncryptionLevel};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct Settings {
        database_server: String,
        database_name: String,
        database_username: String,
        database_password: String,
        log_webhook_uri: String,
        sendgrid_api_key: String,
        email_from_name: String,
        email_from_address: String,
        email_to_addresses: String
    }

    impl Settings {
        //! It's a `new` function.
        pub fn new() -> Result<Settings, String> {
            match Self::get_settings() {
                Ok(settings) => Ok(settings),
                Err(error) => Err(error)
            }
        }

        fn get_settings() -> Result<Settings, String> {
            let secret_blob = match env::var("SecretBlob") {
                Ok(s) => s,
                Err(e) => return Err(format!("Error getting env variable: {}", e.to_string())),
            };

            let sett: Settings = match serde_json::from_str(&secret_blob) {
                Ok(s) => s,
                Err(e) => return Err(format!("Could not deserialize settings blob: {}", e.to_string())),
            };

            Ok(sett)
        }

        //! Returns config for SQL settings.  Requires &str setting the application name
        pub fn get_sql_settings(&self, application_name: &str) -> Result<Config, String> {
            let mut sql_settings = Config::new();
            sql_settings.host(&self.database_server);
            sql_settings.application_name(application_name);
            sql_settings.database(&self.database_name);
            sql_settings.authentication(AuthMethod::sql_server(&self.database_username, &self.database_password));
            sql_settings.encryption(EncryptionLevel::Off);
            sql_settings.trust_cert();

            Ok(sql_settings)
        }

        pub fn get_email_destinations(&self) -> Result<Vec<Email>, String> {
            match self.email_to_addresses.split(",").map(|x| {
                Email::new(x)
            }).collect() {
                Ok(r) => Ok(r),
                Err(e) => Err(e)
            }
        }
    }
}
