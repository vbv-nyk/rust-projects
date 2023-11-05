use std::clone;

struct GameInfo {
    hours_played: u16,
    skill_level: String,
    played_since: String,
}

enum GamesData {
    Terraria(GameInfo),
    Undertale(GameInfo),
    DDLC(GameInfo),
}

struct User {
    favoriteGame: GamesData,
    username: String,
    premium: bool,
}

fn main() {
    let terraria = GamesData::Terraria(GameInfo {
        hours_played: 12,
        skill_level: String::from("Mediocre"),
        played_since: String::from("2018"),
    });

    let user1 = User {
        favoriteGame: terraria,
        username: "vbvnyk".to_string(),
        premium: false,
    };

    match user1.favoriteGame {
        GamesData::Terraria(gameInfo) => {
            println!("{} likes to play {}. His skill level at the game is {} and has been playing since {}", user1.username, "terraria".to_string(), gameInfo.skill_level, gameInfo.hours_played );
        }
        GamesData::DDLC(gameInfo) => {
            println!("{} likes to play {}. His skill level at the game is {} and has been playing since {}", user1.username, "terraria".to_string(), gameInfo.skill_level, gameInfo.hours_played );
        }
        GamesData::Undertale(gameInfo) => {
            println!("{} likes to play {}. His skill level at the game is {} and has been playing since {}", user1.username, "terraria".to_string(), gameInfo.skill_level, gameInfo.hours_played );
        }
    }

    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        // _ became s
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };
    println!("{:?}", opt);

    let submitted: Option<i8> = Some(30);
    if let Some(score) = submitted {
        println!("Your assignment score is {score}");
    } else {
        println!("You still have to submit your assignment");
    }
}
