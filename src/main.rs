use std::io;
use std::collections::HashMap;
extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;
fn main() 
{
    let mut pizza_metrics: HashMap<String, f32>= HashMap::new(); 

        pizza_metrics.insert("Flavor".to_string(), 0.0);
        pizza_metrics.insert("Texture".to_string(), 0.0);
        pizza_metrics.insert("Novelty".to_string(), 0.0);
        pizza_metrics.insert("Popularity".to_string(), 0.0);

    let pizza_blueprint = build_blueprint(pizza_metrics, "Pizza".to_string());
    let mut pizza_chocies:Vec<Item> = Vec::new();

    let mut pizza_cheese_ratings: HashMap<String, f32> = HashMap::new();
        pizza_cheese_ratings.insert("Flavor".to_string(), 5.5);
        pizza_cheese_ratings.insert("Texture".to_string(), 5.0);
        pizza_cheese_ratings.insert("Novelty".to_string(), 1.0);
        pizza_cheese_ratings.insert("Popularity".to_string(), 8.5);

    let mut pizza_pepperoni_ratings: HashMap<String, f32> = HashMap::new();
        pizza_pepperoni_ratings.insert("Flavor".to_string(), 5.8);
        pizza_pepperoni_ratings.insert("Texture".to_string(), 6.2);
        pizza_pepperoni_ratings.insert("Novelty".to_string(), 2.5);
        pizza_pepperoni_ratings.insert("Popularity".to_string(), 7.2);

    let mut pizza_pineapple_ratings: HashMap<String, f32> = HashMap::new();
        pizza_pineapple_ratings.insert("Flavor".to_string(), 2.1);
        pizza_pineapple_ratings.insert("Texture".to_string(), 2.0);
        pizza_pineapple_ratings.insert("Novelty".to_string(), 6.0);
        pizza_pineapple_ratings.insert("Popularity".to_string(), 4.5);

    pizza_chocies.push(build_item(&pizza_blueprint, "Cheese".to_string(), pizza_cheese_ratings ));
    pizza_chocies.push(build_item(&pizza_blueprint, "Pepperoni".to_string(), pizza_pepperoni_ratings ));
    pizza_chocies.push(build_item(&pizza_blueprint, "Pineapple".to_string(), pizza_pineapple_ratings ));

    let pizza_types: Catagory = Catagory { name: "Pizzas!".to_string(), items: pizza_chocies };
    save_catagory(&pizza_types);
    println!("{:?}", pizza_types);
    let pizza_types = load_catagory(String::from("Pizzas!"));
    println!("{:?}", pizza_types);
    loop
    {
        println!("Enter Command: ");
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
            
        
        match cmd.as_str().trim()
        {
            "/display" => continue,
            "/quit" => break,
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
fn load_catagory(name: String) -> Catagory
{
    load_file(format!("./saves/{}_save.bin",name.to_string()), 0).unwrap()
}