use std::thread;
use std::time;
use std::io;
use rand::Rng;
use colorized::*;

fn main() {

    let mut loop_number = 100;

    while loop_number != 0 {

        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

        loop_number -= 1;

    }
    
    thread::sleep(time::Duration::from_millis(250));
    println!("{}Numberism{}", Colors::BrightGreenFg.value(), Colors::Reset.value());
    thread::sleep(time::Duration::from_millis(250));
    println!("--------------------------------------------------");
    thread::sleep(time::Duration::from_millis(250));
    println!("{}Welcome to Numberism! This game is all about guessing the number.", Colors::BrightCyanFg.value());
    thread::sleep(time::Duration::from_millis(250));
    println!("If you think this is a normal guessing game, it\'s not.");
    thread::sleep(time::Duration::from_millis(250));
    println!("You only get 10 guesses. If all are wrong, you lose. But if they\'re right, you win.");
    thread::sleep(time::Duration::from_millis(250));
    println!("I forgot to mention that you have to guess a number from a range of 1 to 10000.");
    thread::sleep(time::Duration::from_millis(250));
    println!("All I want to say is ... Good Luck!{}", Colors::Reset.value());
    thread::sleep(time::Duration::from_millis(500));

    let mut guess1 = String::new();
    let mut guess2 = String::new();
    let mut guess3 = String::new();
    let mut guess4 = String::new();
    let mut guess5 = String::new();
    let mut guess6 = String::new();
    let mut guess7 = String::new();
    let mut guess8 = String::new();
    let mut guess9 = String::new();
    let mut guess10 = String::new();

    println!(" ");
    println!("Input your first guess:");

    io::stdin().read_line(&mut guess1).expect("Failed to read line");

    println!(" ");
    println!("Input your second guess:");

    io::stdin().read_line(&mut guess2).expect("Failed to read line");

    println!(" ");
    println!("Input your third guess:");

    io::stdin().read_line(&mut guess3).expect("Failed to read line");

    println!(" ");
    println!("Input your fourth guess:");

    io::stdin().read_line(&mut guess4).expect("Failed to read line");

    println!(" ");
    println!("Input your fifth guess:");

    io::stdin().read_line(&mut guess5).expect("Failed to read line");

    println!(" ");
    println!("Input your sixth guess:");

    io::stdin().read_line(&mut guess6).expect("Failed to read line");

    println!(" ");
    println!("Input your seventh guess:");

    io::stdin().read_line(&mut guess7).expect("Failed to read line");
    
    println!(" ");
    println!("Input your eighth guess:");

    io::stdin().read_line(&mut guess8).expect("Failed to read line");

    println!(" ");
    println!("Input your ninth guess:");

    io::stdin().read_line(&mut guess9).expect("Failed to read line");

    println!(" ");
    println!("Input your tenth guess:");

    io::stdin().read_line(&mut guess10).expect("Failed to read line");


    let guess1: u32 = guess1.trim().parse().expect("That is not a number!");
    let guess2: u32 = guess2.trim().parse().expect("That is not a number!");
    let guess3: u32 = guess3.trim().parse().expect("That is not a number!");
    let guess4: u32 = guess4.trim().parse().expect("That is not a number!");
    let guess5: u32 = guess5.trim().parse().expect("That is not a number!");
    let guess6: u32 = guess6.trim().parse().expect("That is not a number!");
    let guess7: u32 = guess7.trim().parse().expect("That is not a number!");
    let guess8: u32 = guess8.trim().parse().expect("That is not a number!");
    let guess9: u32 = guess9.trim().parse().expect("That is not a number!");
    let guess10: u32 = guess10.trim().parse().expect("That is not a number!");

    let answer = rand::thread_rng().gen_range(1..10000);

    println!("\n");
    println!("First Guess -> {guess1}");
    println!("Second Guess -> {guess2}");
    println!("Third Guess -> {guess3}");
    println!("Fourth Guess -> {guess4}");
    println!("Fifth Guess -> {guess5}");
    println!("Sixth Guess -> {guess6}");
    println!("Seventh Guess -> {guess7}");
    println!("Eighth Guess -> {guess8}");
    println!("Ninth Guess -> {guess9}");
    println!("Tenth Guess -> {guess10}");
    println!("\nThe answer was {answer}.");
    println!(" ");

    if guess1 == answer {
        println!("The first guess was correct!");
    } 
    
    if guess2 == answer {
        println!("The second guess was correct!");
    }
    
    if guess3 == answer {
        println!("The third guess was correct!");
    }

    if guess4 == answer {
        println!("The fourth guess was correct!");
    } 
    
    if guess5 == answer {
        println!("The fifth guess was correct!");
    }
    
    if guess6 == answer {
        println!("The sixth guess was correct!");
    }

    if guess7 == answer {
        println!("The seventh guess was correct!");
    } 
    
    if guess8 == answer {
        println!("The eigth guess was correct!");
    }
    
    if guess10 == answer {
        println!("The ninth guess was correct!");
    }

    if guess9 == answer {
        println!("The ninth guess was correct!");
    }

    if guess10 == answer {
        println!("The tenth guess was correct!");
    }

    if guess1 != answer {
        println!("The first guess was incorrect!");
    }

    if guess2 != answer {
        println!("The second guess was incorrect!");
    }

    if guess3 != answer {
        println!("The third guess was incorrect!");
    } 
    
    if guess4 != answer {
        println!("The fourth guess was incorrect!");
    }
    
    if guess5 != answer {
        println!("The fifth guess was incorrect!");
    }

    if guess6 != answer {
        println!("The sixth guess was incorrect!");
    } 
    
    if guess7 != answer {
        println!("The seventh guess was incorrect!");
    }
    
    if guess8 != answer {
        println!("The eighth guess was incorrect!");
    }

    if guess9 != answer {
        println!("The ninth guess was incorrect!");
    }

    if guess10 != answer {
        println!("The tenth guess was incorrect!");
    }

    let mut j = String::new();

    println!("\n\n\n");
    
    io::stdin().read_line(&mut j).expect("Failed to read line");

}