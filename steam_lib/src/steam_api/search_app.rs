use std::error::Error;
use std::fmt;
use std::ops::{Deref, DerefMut};
use derive_getters::Getters;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum Platforms {
    Windows,
    MacOS,
    Linux,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// What the response of `https://api.steampowered.com/ISteamApps/GetAppList/v2/` looks like.
pub struct AppListResponse {
    pub applist: Applist,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Applist {
    pub apps: Vec<App>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct App {
    pub appid: u64,
    pub name: String,
}

#[derive(Debug)]
pub struct SteamError {
    details: String
}

impl SteamError {
    pub fn new(msg: &str) -> SteamError {
        SteamError { details: msg.to_string() }
    }
}

impl fmt::Display for SteamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SteamError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Getters)]
pub struct SteamApp {
    app: App,
    is_free: bool,
    website: String,
    // packages: Vec<u32>,
    // release_date: DateTime<Local>,
    legal_notice: String,
}

impl Deref for SteamApp {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl DerefMut for SteamApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}

impl SteamApp {
    pub fn by_appid(app_id: u32) -> Result<SteamApp, SteamError> {
        let http = blocking::Client::new();

        let response = http.get(format!("https://store.steampowered.com/api/appdetails?appids={}", app_id)).send().unwrap();
        let data: Value = serde_json::from_str(&*response.text().unwrap()).unwrap();
        let data = &data[app_id.to_string()];

        if data["success"] != true {
            // println!("{}", data["success"]);
            return Err(SteamError::new(&format!("Couldn't find app with id {}", app_id)));
        }
        let data = &data["data"];

        // println!("{}", data["release_date"]["date"].as_str().unwrap());
        Ok(SteamApp {
            app: App {
                appid: app_id as u64,
                name: data["name"].to_string(),
            },
            // app_id,
            // app_name: data["name"].to_string(),
            is_free: data["is_free"].as_bool().unwrap(),
            website: data["website"].to_string(),
            // packages: data["packages"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u32).collect(),
            legal_notice: data["legal_notice"].to_string(),
        })
    }

    pub fn by_search_result(query: &str) -> Vec<SteamApp> {
        let http = blocking::Client::new();

        let response = http.get("https://api.steampowered.com/ISteamApps/GetAppList/v2/").send().unwrap().text().unwrap();
        let data: AppListResponse = serde_json::from_str(response.as_str()).unwrap();

        // println!("{:?}", data);

        let mut app_ids: Vec<u64> = Vec::new();
        let mut steam_apps: Vec<SteamApp> = Vec::new();

        for game in data.applist.apps {
            if game.name.contains(query) {
                // println!("{:?}", game["name"].to_string());
                app_ids.push(game.appid);
            }
        }

        for app in app_ids {
            steam_apps.push(SteamApp::by_appid(app as u32).unwrap())
        }
        steam_apps
    }
}