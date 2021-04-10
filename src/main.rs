/*

In this project i'll be expermienting with the String slice data type. 

*/


use std::io; //needed to get user inputs


fn main() {
    loop { //allowing the user to run this program in a loop.
        let mut text = String::new(); //defining a new variable called text which is an empty string.
        println!("Please type in some text");
        io::stdin() //getting some input
            .read_line(&mut text)
            .expect("Unable to read line");

        println!("You inputted: {}", text);
        println!("Please now type the index ranges which you would like to access");
        let mut index1 = String::new();
        let mut index2 = String::new();
        io::stdin()
            .read_line(&mut index1)
            .expect("Unable to read line");
        println!("Awesome! Now type in the second index");
        io::stdin()
            .read_line(&mut index2)
            .expect("Unable to read line");

        let index1: usize = match index1.trim().parse() { //doing some basic string parsing to the type of "uszie"
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Index 1");
                break;
            }
        };

        let index2: usize = match index2.trim().parse() { //doing some basic string parsing to the type of "uszie"
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Index 1");
                break;
            }
        };

        let indexed = &text[index1..index2]; //actually indexing the text with the 2 indexes which are of the usize data type.
        println!("Your indexed value is {}", indexed); //printing our final result
        
    }
}