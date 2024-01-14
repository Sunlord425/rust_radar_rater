use std::{io, path};
use std::collections::HashMap;
use std::fs;
use std::convert::TryFrom;
extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;
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

#[derive(Debug, Savefile)]
struct Catagory
{
    name: String,
    items: Vec<Item>
}

#[derive(Clone, Debug, Savefile)]
struct Item
{
    name: String,
    metrics: HashMap<String, f32>
}



fn build_item(blueprint: &Item, name: String, scores: HashMap<String, f32>) -> Item 
{

    if !(metric_key_equality_check(&blueprint.metrics, &scores))
    {
        panic!("Error: blueprint field mismatch for item {}", name.trim());
    }
    let mut item_out = blueprint.clone();
    item_out.metrics = scores;
    item_out.name = name;
    
    item_out
}

fn build_blueprint(metrics_in: HashMap<String, f32>, catagory: String) -> Item
{
    let mut blueprint = Item {
        name: format!("{} blueprint", catagory),
        metrics: metrics_in,
    };
    blueprint
}

fn metric_key_equality_check(map1:&HashMap<String,f32>, map2:&HashMap<String,f32>) -> bool 
{
    let mut key_match: bool = true;
    for (key, value) in map1.iter()
    {
        if !(map2.contains_key(key)) 
        {
            key_match = false;
        }
    }

    for (key, value) in map2.iter()
    {
        if !(map1.contains_key(key)) 
        {
            key_match = false;
        }
    }
    key_match
}
fn save_catagory(catagory: &Catagory) 
{
    save_file(format!("./saves/{}_save.bin", catagory.name), 0, catagory).unwrap();
}

fn load_catagory(pathname: &String) -> Catagory
{
    load_file(format!("{}",pathname.to_string()), 0).unwrap()
}

fn get_savenames(pathname: &str) -> Vec<String>
{
    let paths = fs::read_dir(&pathname)
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path().to_string_lossy().into_owned())
                .collect::<Vec<_>>();
    paths
}