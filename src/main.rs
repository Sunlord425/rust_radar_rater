use std::io;
use std::collections::HashMap;
fn main() 
{
    loop
    {
        println!("Enter Command: ");
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
            
        
        match cmd.as_str().trim()
        {
            "/quit" => break,
            other => 
            {
                println!("command \'{}\' does not exist \n", other.trim());
                continue;
            }
        }
    }
}

struct Catagory
{
    name: String,
    items: Vec<Item>
}

struct Item
{
    name: String,
    metrics: HashMap<String, f32>
}



fn build_item(blueprint: Item, name: String, scores: HashMap<String, f32>) -> Item 
{

    if !(metric_key_equality_check(&blueprint.metrics, &scores))
    {
        panic!("Error: blueprint field mismatch for item {}", name.trim());
    }
    let mut item_out = blueprint;
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
