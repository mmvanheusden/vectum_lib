use std::io::{stdin, stdout, Write};

fn main() {
    search();
}


fn search() {
    let array = steam_rs::steam_api::search_app::get_steamapps();


    let mut s = String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    for game in array {
        if game.name.contains(s.trim()) {
            println!("{}", game.name.trim())
        }
    }
}