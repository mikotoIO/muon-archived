use dotenv::dotenv;
use muon_core::Entity;

pub mod entities;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
    dbg!(entities::Space::get_scylla_schema());
}
