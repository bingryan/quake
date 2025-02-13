use figment::providers::Yaml;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Serialized};
use rocket::figment::{Figment, Profile};
use rocket::fs::FileServer;
use rocket::{routes, Build, Config, Rocket};
use serde_derive::{Deserialize, Serialize};

use quake_core::QuakeConfig;

mod action_api;
mod entry_api;
mod helper;
mod transflow_api;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiString {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSuccess {
    pub content: String,
}

pub fn quake_rocket() -> Rocket<Build> {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Yaml::file(".quake.yaml"))
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("workspace", "."))
        .select(Profile::from_env_or("server_location", "web"));

    let server: String = figment.extract_inner("server_location").unwrap();
    rocket::custom(figment)
        .mount("/", FileServer::from(server))
        .mount(
            "/entry",
            routes![
                entry_api::get_entries,
                entry_api::get_entries_csv,
                entry_api::get_entries_from_csv,
                entry_api::get_entry,
                entry_api::create_entry,
                entry_api::update_entry
            ],
        )
        .mount(
            "/action",
            routes![action_api::parse_query, action_api::suggest],
        )
        .mount(
            "/transflow",
            routes![
                transflow_api::transflow_defines,
                transflow_api::transfunc_script,
                transflow_api::translate
            ],
        )
        .attach(AdHoc::config::<QuakeConfig>())
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use super::quake_rocket;

    #[cfg(feature = "webserver")]
    #[test]
    fn hello_world() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[cfg(feature = "webserver")]
    #[test]
    fn get_todo_entry() {
        let client = Client::tracked(quake_rocket()).expect("valid rocket instance");
        let response = client.get("/entry/todo/1").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "{\"title\":\"time support\",\"author\":\"\",\"content\":\"\",\"created_date\":\"2021-11-24 19:14:10\",\"updated_date\":\"2021-11-24 19:14:10\",\"id\":1,\"content\":\"\\n\\nahaha\\n\"}")
    }
}
