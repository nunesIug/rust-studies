use std::io;

fn main () {

    println!("{:-^40}", "enter ur data, player: ");

    let mut username: String = String::new();
    let mut email: String = String::new();
    let mut password: String = String::new();



    io::stdin()
        .read_line(&mut username)
        .expect("an error accoured.");

    io::stdin()
        .read_line(&mut email)
        .expect("an error accoured.");

    io::stdin()
        .read_line(&mut password)
        .expect("an error accoured.");



    fn info_player (username: String, email: String, password: String, authenticated: bool) {
        return println!("Info player account: \n username: {} \n email: {} \n password: {} \n is authenticated: {}", username, email, password, authenticated);
    }
    


    let info: () = info_player(username, email, password, true);


    let favorite_games: [&str; 3] = ["Red dead Redemption II", "Resident Evil II Remake", "Dying light"];


    println!("{:?} \n favorite games: \n {:?} \n last game played: {:?}", info, favorite_games, favorite_games[2]);

}