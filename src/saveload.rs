extern crate savefile;
use std::fs;

use savefile::prelude::*;
use crate::catagory_type::Catagory;

pub fn save_catagory(catagory: &Catagory) 
{
    save_file(format!("./saves/{}_save.bin", catagory.name), 0, catagory).unwrap();
}

pub fn load_catagory(pathname: &String) -> Catagory
{
    load_file(format!("{}",pathname.to_string()), 0).unwrap()
}

pub fn get_savenames(pathname: &str) -> Vec<String>
{
    let paths = fs::read_dir(&pathname)
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path().to_string_lossy().into_owned())
                .collect::<Vec<_>>();
    paths
}