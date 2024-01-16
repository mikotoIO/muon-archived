use dotenv::dotenv;
use muon_core::Entity;

use crate::entities::Space;

pub mod entities;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
    Space::scratch();
}
