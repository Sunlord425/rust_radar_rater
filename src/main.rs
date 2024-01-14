use std::{io, path};
use std::collections::HashMap;
use std::fs;
use std::convert::TryFrom;
extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;
mod catagory_type;
mod saveload;
use crate::catagory_type::Catagory;
use crate::saveload::*;
fn main() 
{
    let mut current_catagory = Catagory {
        name: "None".to_string(),
        items: Vec::new()
    };
    loop
    {
        println!("Enter Command: ");
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
            
        
        match cmd.as_str().trim()
        {
            "/display" => 
            {
                if current_catagory.name == "None" 
                {
                    println!("no catagory currently loaded")
                }
                else
                {
                    println!("{:?}", current_catagory);
                }
                continue;
            },
            "/save" => 
            {
                save_catagory(&current_catagory);
                continue;
            },
            "/load" =>
            {
                let paths = get_savenames("./saves");

                println!("saves found: \n");
                let mut save_iter:u32 = 0;
                for s in &paths
                {
                    println!("\t{}. {}",save_iter,&s[8..]);
                    save_iter += 1;
                }

                println!("select save number: ");

                let mut save_number: String = String::new();

                io::stdin()
                    .read_line(&mut save_number)
                    .expect("Failed to read line");

                let save_number: u32 = match save_number.trim().parse()
                {
                    Ok(num) => num,
                    Err(_) => {println!("Error: input must be a number"); continue;},
                };

                if save_iter >= save_number
                {
                    let index = usize::try_from(save_number).unwrap();
                    println!("{}",&paths[index]);
                    current_catagory = load_catagory(&paths[index]);
                }
                else 
                {
                    println!("selection out of range");
                    continue;
                }
                

                continue;
        
            }
            "/q" => break,
            "/wq" => 
            {
                save_catagory(&current_catagory);
                break;
            },
            other => 
            {
                println!("command \'{}\' does not exist \n", other.trim());
                continue;
            }
        }
    }
}


