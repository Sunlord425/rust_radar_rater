use crate::catagory_type::Item;

fn distance(item1: &Item, item2: &Item) -> f32
{
    let mut distance: f32 = 0.0;
    for (k,v) in &item1.metrics 
    {
        distance += (v - item2.metrics[k]).powi(2)
    }
    distance = distance.sqrt();
    distance
}