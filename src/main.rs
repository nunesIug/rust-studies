use std::io;
fn main() {

    println!("player, enter ur name: ");

    let mut username: String = String::new();
    
    let online: bool = true;
    let playing: bool = true;
    let favorite_games: [&str; 3] = ["Red dead Redemption II", "Dying light", "Residente Evil II"];


    io::stdin()
        .read_line(&mut username)
        .expect("an error accoured.");


    if online && playing == true {
        return println!("{} is online, playing {:?}", username, favorite_games[0]);
    } 
    else {
        return println!("{} is offline, the last game he played was {}", username, favorite_games[2]);
    }
}