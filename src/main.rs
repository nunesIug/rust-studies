fn main() {
    let username: &str = "Nu S";

    // user info
    let playing: bool = true;
    let online: bool = false;

    let game_library: [&str; 2] = ["Red dead Redemption II", "Resident Evil II"];

    let time_played: (u32, u32, u32) = (10, 13, 15);

    
    // verify if player is online and playing
    if playing && online == true {
        return println!("{} is online, playing {}", username, game_library[0]);
    }
    else {
        return println!("{} is offline, the last game he played was {}. He played for {} hours.", username, game_library[0], time_played.2);
    };
}