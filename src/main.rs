extern crate savefile;
use catagory_type::{build_catagory, add_item};
use terminal_interface::{get_input, display, loader, find_min, find_max};

#[macro_use]
extern crate savefile_derive;
mod catagory_type;
mod saveload;
mod terminal_interface;
mod actual_math_part;
use crate::catagory_type::Catagory;
use crate::saveload::*;

fn main() 
{
    let mut current_catagory: Option<Catagory>  = None;
    loop
    {
        
        let mut cmd = get_input(String::from("Enter Command"));
        
            
        
        match cmd.as_str().trim()
        {
            "/display" => 
            {
                display(&current_catagory);
                continue;
            },
            "/save" => 
            {
                save_catagory(&current_catagory);
                continue;
            },
            "/load" => loader(&mut current_catagory),
            "/addC" => current_catagory = Some(build_catagory(get_input("Enter new catagory name".to_string()))),
            "/addI" => add_item(&mut current_catagory, get_input("Enter Item Name".to_string())),
            "/compMin" => find_min(&current_catagory),
            "/compMax" => find_max(&current_catagory),
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


