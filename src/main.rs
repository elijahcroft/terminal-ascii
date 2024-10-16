use std::fs;
use colored::*;

fn main(){
   let art = fs::read_to_string("/home/elijahcroft49/rust_projects/ascii_art/art.txt")
       .expect("Something went wrong reading the file");


   println!("{}", art.red());
}