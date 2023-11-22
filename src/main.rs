use std::fs;
use rand::Rng;

fn main() {

    //Need to figure out how to take in text file
    //let my_words_list: Vec<u8> = read_file_vec("words.txt");
    let my_word_list: Vec<String> = fs::read_to_string("words.txt")    // Creates a String
                                    .expect("Failed to read input")
                                    .split("\n")                     // Takes references into the String
                                    .map(|s| s.to_string())
                                    .collect();                      // String is dropped, invalidating references
 

    let mut rng = rand::thread_rng(); 
    let my_word: &str = my_word_list[rng.gen_range(0..my_word_list.len())].as_str(); 
    let mut damage: i8 = 0;
    let mut correct: i8 = 0;
    let mut used_guesses: Vec<char> = Vec::new();
    
    //let mut test_word_list: Vec<&str> = vec!["apple","orange","hangman","coding","test"];
 
    //This is creating an array of '-' with the same length of the chosen word
    let mut dash_vec: Vec<char> = vec!['-';my_word.len()];
    
    //The & in front of my_word allows the function end_game_check to borrow the data without
    //releasing the memory allocation and ownership of the orignal function
    //This function checks to see if the game is over after each turn
    while end_game_check(&my_word,damage,correct) == false {
        
        let word_vec: Vec<char> = my_word.chars().collect(); 
        //println!("{:?}",word_vec);
        
        //This prints out the hidden word along with any correct letters chosen
        print_vec(dash_vec.clone());

        //takes single char string and converts to char to check against hangman word vector
        let mut guess_char: char = '\0';
        let mut count: i8 = 0;
        
        //Allows user to guess a letter and then checks if its valid and has not already been
        //selected
        while count == 0 {
            let mut guess: String = String::new();
            let mut picked: i8 = 0;
            println!("Guess a letter: ");
            std::io::stdin().read_line(&mut guess).unwrap();
            let trimmed_len = guess.trim_end().len();
            guess.truncate(trimmed_len);
            if guess.chars().count()!=1 {
                println!("Please enter a single letter only!");
            } else if guess.to_lowercase().chars().all(|c| matches!(c, 'a'..='z')) != true {
                println!("Please enter only letters!");
            } else {
                guess_char = guess.to_lowercase().chars().nth(0).unwrap();
                for num in &used_guesses {
                    //The * in front of num dereferences the comparison, need to look into this
                    //further for more explanation into why it works
                    if *num==guess_char {
                        picked = picked + 1;
                    }
                }
                if picked >= 1 {
                    println!("You already guessed that letter, pick another!");
                } else {
                    used_guesses.push(guess_char);
                    count = 1;
                }
            }    
        }
      
        //This informs the program if a correct letter was chosen and which indicies the letter
        //falls in. It then replaces the '-' with the correct letter.
        let mut correct_indecies_vec: Vec<i8> = Vec::new();
        let mut char_check_counter: i8 = 0;
        let hold: char = guess_char.clone(); 
        let mut i: i8 = 0;
        for check_char in word_vec {

            if check_char == guess_char {
                char_check_counter = char_check_counter+1;
                correct = correct + 1;
                correct_indecies_vec.push(i);
                dash_vec[i as usize] = hold;

            }
            i = i + 1;

        }
        
        //Prints out correct or incorrect based on selection
        if char_check_counter == 0 {
            damage = damage + 1;
            println!("Incorrect!");
            println!("");
        } else {
            println!("Correct!");
            println!("");
        }

    }
}


//Function to check if the game is over by either damage or all correct letters being chosen
fn end_game_check (my_word: &str, damage: i8, correct: i8) -> bool {
    
    //Use a switch statement to print out hangman based off of current damage
    match damage{
        0=>{
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
         },
        1=>{
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");

        },
        2=>{
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r"  |   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        },
        3=>{
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r" /|   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        },
        4=>{
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r" /|\  |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        },
        5=>{
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r" /|\  |");
            println!(r" /    |");
            println!(r"      |");
            println!(r"=========");
        },
        6=>{
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"  O   |");
            println!(r" /|\  |");
            println!(r" / \  |");
            println!(r"      |");
            println!(r"=========");
        },
        _=>{
            println!(r"  +---+");
            println!(r"  |   |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"      |");
            println!(r"=========");
        }
    } 

    if damage >= 6 {
        println!("Game over, you LOSE!");
        println!("The word was {}", my_word); 
        return true;
    } else if correct >= my_word.len() as i8 {
        println!("You WIN, Congrats!");
        println!("The word was {}", my_word);
        return true;
    } else {
        return false;
    }

}

//Prints out the gameboard (The word as '-')
fn print_vec(my_vec: Vec<char>) {
    println!("");
    println!("This is your word:");
    for x in my_vec {
        print!("{} ",x);
    }
    println!("");
    println!("");
}

