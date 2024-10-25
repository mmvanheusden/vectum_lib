use steam_rs::steam_api::search_app::SteamApp;

fn main() {
    let search_results = SteamApp::by_search_result("Subnautica");
    let subnautica = search_results.first().unwrap();

    println!("{:?}", subnautica);
}
