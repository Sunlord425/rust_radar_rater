use std::io::{self, Write};

use crate::{catagory_type::{Catagory, Item},
saveload::{get_savenames, load_catagory},
actual_math_part::{min_distance, max_distance}};

pub fn get_input(message: String) -> String 
{
    print!("{}: ", message);
    std::io::stdout().flush().unwrap();
    let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
    cmd.trim().to_string()
}

pub fn display(catagory: &Option<Catagory>) 
{
    match catagory 
    {
        None => println!("No catagory selected"),
        Some(cat) => println!("{:#?}",cat)
    }    

}
pub fn find_min(current_catagory: & Option<Catagory>) 
{
    match current_catagory {
        None => println!("No catagory selected"),
        Some(cat) => loop {
            println!("Find most similar item to which item: \n");
            let mut cat_iter:u32 = 0;
            for s in &cat.items
            {
                println!("\t{}. {}",cat_iter, s.name);
                cat_iter += 1;
            }
            let cat_number: String = get_input("select item number".to_string()).trim().parse().expect("Please Enter a number");

            let cat_number: u32 = match cat_number.trim().parse()
            {
                Ok(num) => num,
                Err(_) => {println!("input must be a number \n"); continue;},
            };
            
        
        
            if cat_iter >= cat_number+1
            {
                let index = usize::try_from(cat_number).unwrap();
                match min_distance(&cat.items[index],cat) 
                {
                    None => {println!("Nothing to compare to")},
                    Some(item) => 
                    {
                        println!("the most simlar item to {} is {}",cat.items[index].name, item.name);
                    }
                };
                break;
            }
            else 
            {
            println!("selection out of range \n");
            continue;
            }
        }
    }
}

pub fn find_max(current_catagory: & Option<Catagory>) 
{
    match current_catagory {
        None => println!("No catagory selected"),
        Some(cat) => loop {
            println!("Find most similar item to which item: \n");
            let mut cat_iter:u32 = 0;
            for s in &cat.items
            {
                println!("\t{}. {}",cat_iter, s.name);
                cat_iter += 1;
            }
            let cat_number: String = get_input("select item number".to_string()).trim().parse().expect("Please Enter a number");

            let cat_number: u32 = match cat_number.trim().parse()
            {
                Ok(num) => num,
                Err(_) => {println!("input must be a number \n"); continue;},
            };
            
        
        
            if cat_iter >= cat_number+1
            {
                let index = usize::try_from(cat_number).unwrap();
                match max_distance(&cat.items[index],cat) 
                {
                    None => {println!("Nothing to compare to")},
                    Some(item) => 
                    {
                        println!("the least simlar item to {} is {}",cat.items[index].name, item.name);
                    }
                };
                break;
            }
            else 
            {
            println!("selection out of range \n");
            continue;
            }
        }
    }
}

pub fn loader(current_catagory: &mut Option<Catagory>) 
{
    let paths = get_savenames("./saves");  
                loop {
                    println!("saves found: \n");
                    let mut save_iter:u32 = 0;
                    for s in &paths
                    {
                        println!("\t{}. {}",save_iter,&s[8..]);
                        save_iter += 1;
                    }
                    let save_number: String = get_input("select save number".to_string()).trim().parse().expect("Please Enter a number");

                    let save_number: u32 = match save_number.trim().parse()
                    {
                        Ok(num) => num,
                        Err(_) => {println!("input must be a number \n"); continue;},
                    };
                    
                
                
                    if save_iter >= save_number+1
                    {
                        let index = usize::try_from(save_number).unwrap();
                        println!("{} loaded",&paths[index]);
                        *current_catagory = Some(load_catagory(&paths[index]));
                        break;
                    }
                    else 
                    {
                    println!("selection out of range \n");
                    continue;
                    }
                }
                
}