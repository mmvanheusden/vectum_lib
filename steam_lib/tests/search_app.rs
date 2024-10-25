use steam_rs::steam_api::search_app::SteamApp;

#[test]
#[should_panic]
fn test_invalid_appid() {
    println!("This test should panic. Requesting Steam API with non-existent app ID...\n");
    let appid = 33; // non-existent game.
    SteamApp::by_appid(appid).unwrap();
}

#[test]
fn test_valid_appid() {
    println!("This test should NOT panic. Requesting Steam API with existent app ID...\n");

    let appid = 264710; // Subnautica
    let app = SteamApp::by_appid(appid).unwrap();

    println!("App ID: {}", app.appid);
    println!("App Name: {}", app.name);
    println!("Is Free: {}", app.is_free());
    println!("Website: {}", app.website());
    // println!("Packages: {:?}", app.packages());
    // println!("Release Date: {}", app.release_date());
    println!("Legal Notice: {}", app.legal_notice());
}

#[test]
fn test_search_game() {
    let game_name = "Subnautica";
    println!("This test should NOT panic. Searching Steam games for \"{game_name}\"\n");

    let apps = SteamApp::by_search_result(game_name);

    for steam_app in apps {
        println!("App ID: {}", steam_app.appid);
        println!("App Name: {}", steam_app.name);
        println!("Is Free: {}", steam_app.is_free());
        println!("Website: {}", steam_app.website());
        // println!("Packages: {:?}", app.packages());
        // println!("Release Date: {}", app.release_date());
        println!("Legal Notice: {}", steam_app.legal_notice());
        print!("----------------------\n\n\n")
    }
}
