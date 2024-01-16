use dotenv::dotenv;
use muon_core::Entity;

pub mod entities;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
    println!("{}", entities::Space::build_table_schema());
}
