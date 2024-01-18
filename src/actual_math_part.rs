use crate::catagory_type::{Item, Catagory};

pub fn distance(item1: &Item, item2: &Item) -> f32
{
    let mut distance: f32 = 0.0;
    for (k,v) in &item1.metrics 
    {
        distance += (v - item2.metrics[k]).powi(2)
    }
    distance.sqrt()
}
pub fn min_distance<'a>(item: &Item, catagory: & 'a Catagory) ->  Option<&'a Item>
{
    let mut min_item: Option<&Item> = None;
    let mut current_min = f32::MAX;
    for i in &catagory.items
    {
        if i.name == item.name
        {
            continue;
        }
        else 
        {
            if distance(item, &i) < current_min
            {
                current_min = distance(item, &i);
                min_item = Some(&i);
            }
        }
    }
    min_item
}

pub fn max_distance<'a>(item: &Item, catagory: & 'a Catagory) ->  Option<&'a Item>
{
    let mut max_item: Option<&Item> = None;
    let mut current_max = f32::MIN;
    for i in &catagory.items
    {
        if i.name == item.name
        {
            continue;
        }
        else 
        {
            if distance(item, &i) > current_max
            {
                current_max = distance(item, &i);
                max_item = Some(&i);
            }
        }
    }
    max_item
}