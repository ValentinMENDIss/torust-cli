/* torust-cli -     todo list for rustaceans */
/* published on:    https://github.com/ValentinMENDIss/torust-cli */
/* license:         GPL-3 */
///////////////////////////////////////


use std::io;


/* main loop/function  */
fn main() {
    /* printing project's info */
    println!("ToRust - To-Do CLI Tool for Rustaceans!");
    println!("Made by: ValentinMENDIss");
    
    /* declaration of variables/lists/... */
    let mut my_list: Vec<String> = Vec::new();      /* create/declare list for strings */

/* repeated loop (main code) */
loop {
        println!("\nWhich function do you want to perform? (create/read): ");
    
        let mut inputfunction = String::new();

        io::stdin()
            .read_line(&mut inputfunction)
            .expect("Failed to read line (inputfunction)");

        inputfunction.pop();    /* trim text (delete newlines(\n)) */
        println!("Your input is: {}", inputfunction);

        if inputfunction == "create" {
            loop {
                let mut inputcreate = String::new();

                println!("\nEnter a task (or type 'quit' to exit): ");
                io::stdin()
                    .read_line(&mut inputcreate)
                    .expect("Failed to read line (create todo task)");
           
                if inputcreate.trim() == "quit" {
                    break;
                }
                my_list.push(inputcreate.trim().to_string());
            }

        } else if inputfunction == "read" {
            for s in &my_list {
                println!("\n{}", s);
            }
        } else {
            println!("Sorry, there is no such function...");
        }
}

}
