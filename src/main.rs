use std::io;
use rand::Rng; // 0.8.5

fn main() {
    //Still want to play?
    let mut play = 1;
    while play == 1 {
        println!("Lets play rock paper scissors!");
        rockpaperscissors();
        println!("still want to play? (y/n)");
        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
        println!();

        match response.trim (){
            "y" => println!("Ok!"),
            "n" => play = 0,
            _ => println!("Invalid input."),
        }

    }
    println!("Thanks for playing!");


    fn rockpaperscissors() {
        // Generate computer weapon
        let num = rand::thread_rng().gen_range(0..2);
        let mut comp_weapon = "";
        if num == 0 {
            comp_weapon = "Rock";
        }else if num == 1 {
            comp_weapon = "Paper";
        }else {
            comp_weapon = "scissors";
        }


        // Generate Player weapon
        println!("Please Choose your weapon. r p s ");

        let mut player_weapon = String::new();

        io::stdin()
            .read_line(&mut player_weapon)
            .expect("Failed to read line");
        println!("You chose: {player_weapon}");

// Verify Winner
        //scissors
        if num == 2 {
            match player_weapon.trim() {
                "r" => println!("Computer has chosen {comp_weapon}. \n You lose"),
                "p" => println!("Computer has chosen {comp_weapon}.\n You won!"),
                "s" => println!("Computer has chosen {comp_weapon}.\n You tie!"),
                _ => println!("Invalid input."),
            }
        }
        //paper
        else if num == 1 {
            match player_weapon.trim(){
                "r"=> println!("Computer has chosen {comp_weapon}.\n You lose!"),
                "p"=> println!("Computer has chosen {comp_weapon}.\n You tie!"),
                "s"=> println!("Computer has chosen {comp_weapon}.\n You win!"),
                _=> println!("Invalid input."),
            }
            //rock
        }else if num == 0 {
            match player_weapon.trim() {
                "r" => println!("Computer has chosen {comp_weapon}.\n You tie!"),
                "p" => println!("Computer has chosen {comp_weapon}.\n You win!"),
                "s" => println!("Computer has chosen {comp_weapon}.\n You lose!"),
                _ => println!("Invalid input."),
            }
        }else {
            println!("Computer Error")
        }

    }
}
