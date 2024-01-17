use std::{collections::HashMap, fmt::Error, num::ParseFloatError};

use crate::terminal_interface::get_input;
#[derive(Debug, Savefile)]
pub struct Catagory
{
    pub name: String,
    pub items: Vec<Item>
}

#[derive(Clone, Debug, Savefile)]
pub struct Item
{
    pub name: String,
    pub metrics: HashMap<String, f32>
}



// pub fn build_item(blueprint: &Item, name: String, scores: HashMap<String, f32>) -> Item 
// {

//     if !(metric_key_equality_check(&blueprint.metrics, &scores))
//     {
//         panic!("Error: blueprint field mismatch for item {}", name.trim());
//     }
//     let mut item_out = blueprint.clone();
//     item_out.metrics = scores;
//     item_out.name = name;
    
//     item_out
// }

pub fn build_blueprint(metrics_in: HashMap<String, f32>, catagory: String) -> Item
{
    let mut blueprint = Item {
        name: format!("{}_blueprint", catagory),
        metrics: metrics_in,
    };
    blueprint
}

fn metric_key_equality_check(map1:&HashMap<String,f32>, map2:&HashMap<String,f32>) -> bool 
{
    let mut key_match: bool = true;
    for (key, _value) in map1.iter()
    {
        if !(map2.contains_key(key)) 
        {
            key_match = false;
        }
    }

    for (key, _value) in map2.iter()
    {
        if !(map1.contains_key(key)) 
        {
            key_match = false;
        }
    }
    key_match
}
pub fn build_catagory(cat_name: String) -> Catagory
{
    let catagory_out = Catagory {
        name: cat_name.clone(),
        items: vec![build_blueprint(HashMap::new(), cat_name)],
    };
    catagory_out
}

pub fn add_item(catagory: &mut Option<Catagory>, name: String)
{
    match catagory 
    {
        None => println!("No catagory selected"),
        Some(cat) => 
        {
            let mut blueprint: Option<Item> = None;
            for (i) in cat.items.iter()
            {
                if i.name == format!("{}_blueprint",cat.name) 
                {
                    blueprint = Some(i.clone());
                    break;
                }
            }

            let mut item_in = match blueprint 
            {
                None => {println!("Catagory has no blueprint. creating...");
                        let mut new_blueprint = build_blueprint(HashMap::new(), cat.name.clone());
                        if cat.items.len() != 0
                        {
                            
                            for (i) in &cat.items 
                            {
                                if metric_key_equality_check(&cat.items[0].metrics, &i.metrics) == false 
                                {panic!("Invalid save");}
                            }
                            for (k,_v) in &cat.items[0].metrics 
                            {
                                new_blueprint.metrics.insert(k.to_string(), 0.0);
                            }
                        }
                        cat.items.push(new_blueprint.clone());
                        new_blueprint
                },
                Some(item) => item
            };
                
            
            item_in.name = name;

            for (key, value) in item_in.metrics.clone().iter()
            {
                let mut score= 0.0;

                loop 
                {
                let input: Result<f32, ParseFloatError> = get_input(format!("Enter score for '{}'", key)).trim().parse();
                let input = match input 
                {
                    Ok(out) => out,
                    Err(e) => {println!("input must be a number");
                            continue;}
                };
                score = input;
                break;
                }
                item_in.metrics.insert(key.clone(), score);
            }

            cat.items.push(item_in);
        }
    }
}