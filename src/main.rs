mod pentry;

use core::ascii;
use std::vec;

use crate::pentry::prompt;
use crate::pentry::read_password_from_file;

fn clr() {
    print!("{}[2J",27 as char);

}

fn main(){
    clr();
    let ascii = r#"
     
    ______            ___  _______   __ __|  |_/  |_ 
    \____ \    ______ \  \/ /\__  \ |  |  \  |\   __\
    |  |_> >  /_____/  \   /  / __ \|  |  /  |_|  |  
    |   __/             \_/  (____  /____/|____/__|  
    |__|                          \/                 
    "#;
    
    println!("{ascii}");

    loop {
        println!("Pass Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entry");
        println!("3. Search Entry");
        println!("4. Quit");
        


        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    
                )

            }
            "2" =>{
                clr();
                let services = read_password_from_file().unwrap_or_else(|err|{
                    eprintln!("Error reading password:{}", err);
                    Vec::new()
                });
                for item in &services{
                    println!(
                    "Service = {}
                    - Username : {}
                    - Password : {}",
                    item.service, item.username, item.password
                );
            }


            }
            "3" => {
                clr();
                let services = read_password_from_file().unwrap_or_else(|err|{
                    eprintln!("Error reading password:{}", err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services{
                    if item.service.as_str() == search.as_str(){
                        println!(
                            "Service = {}
                            - Username : {}
                            - Password : {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" =>{
                clr();
                println!("Goodbye!");
                break;

            }
            _ => println!("Invalid Choice.")
        }
        println!("\n\n")






    }


}