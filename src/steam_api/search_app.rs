use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::fmt;
use std::fmt::{Display, Formatter};

/*/// Supported platforms for a [`SteamGame`]
pub enum Platforms {
    Windows,
    MacOS,
    Linux,
}*/


/// What the response of `https://api.steampowered.com/ISteamApps/GetAppList/v2/` looks like.
#[derive(Clone, Deserialize)]
struct AppListResponse {
    applist: Applist,
}

#[derive(Clone, Deserialize)]
struct Applist {
    apps: Vec<SteamApp>,
}

/// A Steam app.
#[derive(Clone, Serialize, Deserialize)]
pub struct SteamApp {
    pub appid: u64,
    pub name: String,
}

#[derive(Debug)]
pub struct Error {
    pub description: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error { description: msg.to_string() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}


/**
Represents a Steam game. Most fields are optional as the API is highly inconsistent and the data is different for every game.
**/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SteamGame {
    pub name: String,
    #[serde(rename = "steam_appid")]
    pub appid: Option<u64>,

    #[serde(deserialize_with = "deserialize_str_to_int")]
    pub required_age: Option<u64>,

    pub is_free: Option<bool>,
    pub detailed_description: Option<String>,
    pub about_the_game: Option<String>,
    pub short_description: Option<String>,
    pub website: Option<String>,
    pub metacritic: Option<Metacritic>,
}

/**
Represents a Metacritic rating for a Steam game.

Example source:
```json
"metacritic": {
    "score": 87,
    "url": "https://www.metacritic.com/game/pc/subnautica?ftag=MCD-06-10aaa1f"
}
```
*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metacritic {
    pub score: Option<u8>,
    pub url: Option<String>,
}

/// For example: [`required_age`] may not be an integer immediately. Sometimes it has quotes around it in the json. Here, we serialize it so it always ends up as either [`None`] or [`u32`].
fn deserialize_str_to_int<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => s.parse::<u64>().map(Some).map_err(serde::de::Error::custom),
        Value::Number(n) => n.as_u64().map(|u| Some(u)).ok_or_else(|| serde::de::Error::custom("Expected a `u64` number")),
        Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom("Invalid type. should be either a JSON number or string")),
    }
}

impl SteamGame {
    /**
    Constructs a [`SteamGame`] instance from the given Steam app ID by fetching data from the Steam Store API.

    # Returns
    A `Result` which is either:
    * `Ok(SteamGame)` - The SteamGame struct instance containing detailed information about the app.
    * `Err(Error)` - An error message if the app ID could not be found or fetched.
    */
    pub async fn from_appid(app_id: u64) -> Result<SteamGame, Error> {
        let response = reqwest::get(format!("https://store.steampowered.com/api/appdetails?appids={}", app_id)).await.unwrap().text().await.unwrap();
        let data: Value = serde_json::from_str(&response).unwrap();
        let data = &data[app_id.to_string()];

        if data["success"] != true {
            // println!("{}", data["success"]);
            return Err(Error::new(&format!("Couldn't find app with id {}", app_id)));
        }
        let data = &data["data"];

        let steam_app: SteamGame = serde_json::from_value(data.clone()).unwrap();
        Ok(steam_app)
    }

    pub async fn from_query(search_query: &str) -> Vec<SteamGame> {
        let response = reqwest::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/").await.unwrap().text().await.unwrap();

        let data: AppListResponse = serde_json::from_str(response.as_str()).unwrap();

        let mut app_ids: Vec<u64> = Vec::new();
        let mut steam_apps: Vec<SteamGame> = Vec::new();

        for game in data.applist.apps {
            if game.name.contains(search_query) {
                app_ids.push(game.appid);
            }
        }

        for app in app_ids {

            // The app being in the steam app list doesn't mean we can get its details with SteamApp::from_appid(), for example when the app ID points to a DLC.
            // When we can't get the app ID's details, skip it and continue to check the next query.
            let app = match SteamGame::from_appid(app).await {
                Ok(steam_app) => steam_app,
                _ => {
                    continue
                }
            };
            steam_apps.push(app);
        }
        steam_apps
    }
}

/// Returns a [`Vec<App>`] with all Steam apps. Queried through the Steam Web API
pub async fn get_steamapps() -> Vec<SteamApp> {
    let response = reqwest::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/").await.unwrap();
    let response: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    let app_list_response: AppListResponse = serde_json::from_value(response).unwrap();
    app_list_response.applist.apps
}