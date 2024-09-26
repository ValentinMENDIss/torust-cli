/* torust-cli -     todo list for rustaceans */
/* published on:    https://github.com/ValentinMENDIss/torust-cli */
/* license:         GPL-3 */
///////////////////////////////////////


use std::io::{self, BufRead, BufWriter, Write};
use std::fs::File;
use std::path::{Path, PathBuf};

/* main loop/function  */
fn main() {
    /* printing project's info */
    println!("ToRust - To-Do CLI Tool for Rustaceans!");
    println!("Made by: ValentinMENDIss");
    
    /* declaration of variables/lists/... */
    let mut my_list: Vec<String> = Vec::new();      /* create/declare list for strings */

/* repeated loop (main code) */
loop {
        println!("\nWhich function do you want to perform? (create/read/readtxt): ");
    
        let mut inputfunction = String::new();

        io::stdin()
            .read_line(&mut inputfunction)
            .expect("Failed to read line (inputfunction)");

        inputfunction.pop();    /* trim text (delete newlines(\n)) */
        println!("Your input is: {}", inputfunction);

        if inputfunction == "create" {
            /* declaration of variables */
            let path = PathBuf::from("./src/todolist/list.txt");
            let file = File::create(path).expect("Could not create list.txt file");
            let mut writer = BufWriter::new(file);

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
                
                writer.write(inputcreate.as_bytes()).expect("Could not write to list.txt file");
                /* Flush the writer to ensure that all data is written to the file */
                writer.flush().expect("Could not flush writer");
            }

        } else if inputfunction == "read" {
            for s in &my_list {
                println!("\n{}", s);
            }
        } else if inputfunction == "readtxt" {
            let path = Path::new("./src/todolist/list.txt");
            let file = File::open(path).unwrap();
            let reader = io::BufReader::new(file);

            let mut lines: Vec<String> = Vec::new();

            for line in reader.lines() {
                let line = line.unwrap();
                lines.push(line.to_string());
            }

            // You can now use the `lines` vector as output
            for line in &lines {
                println!("{}", line);
            }           
        }


        else {
            println!("Sorry, there is no such function...");
        }
}

}
