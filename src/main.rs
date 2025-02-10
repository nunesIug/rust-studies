use std::io;
fn main() {

    let error: &str = "An error accoured.";

    println!("be welcome player, enter ur name to continue:" );

    let mut player_name: String = String::new();

    io::stdin()
        .read_line(&mut player_name)
        .expect(error);


    // this code repeat: -, 20 times
    println!("{:-^40}", "welcome to your account info");


    let online: bool = false;
    let playing: bool = true;
    
    let favorite_games: [&str; 3] = ["Red dead Redemption II", "Dying light", "Resident Evil II"];


    if online && playing == true {
        return println!("{} is online playing {}", player_name.to_uppercase(), favorite_games[0]);
    }
    else {
        return println!("{} is offline, the last game he played was {}", player_name.to_uppercase(), favorite_games[2]);
    };

}