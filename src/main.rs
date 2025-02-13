use std::io;

fn main() {

    let mut username: String = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("an error accoured.");


    if username.trim().to_lowercase() != "nu s" {
        return println!("sorry {}, but ur credentials r invalid. Try again.", username);
    };


    struct Player <'a> {
        username: String,
        skills: u32,
        online: bool,
        playing: bool,
        favorite_games: [&'a str; 3],
        game_library: [&'a str; 5]
    }


    let player: Player = Player {
        username: username,
        skills: 0,
        online: online(),
        playing: true,
        favorite_games: ["Red dead Redemption II", "Dying light", "RE II Remake"],
        game_library: ["Red dead Redemption II", "Dying light", "RE II Remake", "Minecraft", "RE IV Remake"]
    };


    if player.online == false {
        return println!("{} is offline, the last game played was {}", player.username, player.favorite_games[0]);
    }


    println!("Hello, player. welcome {}", player.username);
    println!("here's ur account informations.. \n skills: {} \n is online: {} \n is playing: {} \n ur game library: {:?} \n ur favorite games: {:?}", player.skills, player.online, player.playing, player.game_library, player.favorite_games);

}

fn online() -> bool {
    return true;
}