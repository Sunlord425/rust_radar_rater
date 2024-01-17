use std::io::{self, Write};

use crate::{catagory_type::Catagory, saveload::{get_savenames, load_catagory}};

pub fn get_input(message: String) -> String 
{
    print!("{}: ", message);
    std::io::stdout().flush().unwrap();
    let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
    cmd
}

pub fn display(catagory: &Option<Catagory>) 
{
    match catagory 
    {
        None => println!("No catagory selected"),
        Some(cat) => println!("{:#?}",cat)
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
                    let mut save_number: String = get_input("select save number".to_string()).trim().parse().expect("Please Enter a number");

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