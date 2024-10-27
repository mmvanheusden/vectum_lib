use steam_rs::steam_api::search_app::SteamGame;

fn print_attributes(app: &SteamGame) {
    println!("App ID: {}", app.appid.unwrap_or_default());
    println!("App Name: {}", app.name.clone());
    println!("Is Free: {}", app.is_free.unwrap_or_default());
    println!("Website: {}", app.clone().website.unwrap_or_else(|| "N/A".to_string()));
    println!("About game: {}", app.clone().about_the_game.unwrap_or_else(|| "N/A".to_string()));
    println!("Detailed description: {}", app.clone().detailed_description.unwrap_or_else(|| "N/A".to_string()));
    println!("Required age: {}", app.required_age.unwrap_or_default());
    if let Some(metacritic) = &app.metacritic {
        println!("Metacritic score: {}", metacritic.score.unwrap_or_default());
        println!("Metacritic URL: {}", metacritic.url.clone().unwrap_or_else(|| "N/A".to_string()));
    } else {
        println!("Metacritic score: N/A");
        println!("Metacritic URL: N/A");
    }
}


#[test]
#[should_panic(expected = "Couldn't find app with id 33")]
fn test_invalid_appid() {
    println!("This test should panic. Requesting Steam API with non-existent app ID...\n");

    let appid = 33; // non-existent game.
    SteamGame::from_appid(appid).unwrap();
}

#[test]
fn test_valid_appid() {
    println!("This test should NOT panic. Requesting Steam API with existent app ID...\n");

    let appid: u64 = 271590; // GTA V
    let app = SteamGame::from_appid(appid).unwrap();
    print_attributes(&app);
}

#[test]
fn test_search_games() {
    println!("This test should NOT panic. Searching Steam games for \"Grand Theft Auto\", \"Subnautica\"\n");

    let game_name = "Grand Theft Auto";
    let apps = SteamGame::from_query(game_name);
    for app in apps {
        println!("----------------------");
        print_attributes(&app);
        println!("----------------------\n");
    }

    let game_name = "Subnautica";
    let apps = SteamGame::from_query(game_name);
    for app in apps {
        println!("----------------------");
        print_attributes(&app);
        println!("----------------------\n");
    }
}

#[test]
fn test_lookup_game_with_missing_attribute() {
    println!("This test should NOT panic. The app doesn't have a Metacritic field and so it should be `None`.\n");

    let app = SteamGame::from_appid(848450);
    assert_eq!(app.unwrap().metacritic.is_none(), true);
}