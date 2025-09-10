// use std::io;

pub mod page;
use crate::page::Page;

fn main() {
    println!("Welcome to Storage Manager");
    // let mut catalog_path = String::new();
    // if let Err(e) = io::stdin().read_line(&mut catalog_path){
    //     eprintln!("Error in Recieving input: {}", e);
    // }
    // print!("Input Recieved: {}", catalog_path);
    let page = Page::new(0);
    println!("Page ID: {}", page.id);
    println!("First byte of page: {}", page.data[0]);
}
