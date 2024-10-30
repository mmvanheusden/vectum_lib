use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    search().await;
}


async fn search() {
    let all_apps = vectum_lib::steam_api::search_app::get_steamapps().await;

    let mut s = String::new();
    print!("Enter Steam search query: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    for game in all_apps {
        if game.name.contains(s.trim()) {
            println!("{}", game.name.trim())
        }
    }
}