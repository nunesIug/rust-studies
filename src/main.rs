fn main() {
    let mut username: &str = "Nu S";

    username = "Guilherme";

    let playing: bool = false;
    let online: bool = false;
    let game: &str = "Red Dead Redemption II";

    if playing && online == true {
        println!("{} is online, playing {}.", username, game);
    }
    else {
        println!("{} is offline, last game he played was {}", username, game);
    }
}