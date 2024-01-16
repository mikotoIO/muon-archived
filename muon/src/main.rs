use dotenv::dotenv;
use muon_core::Entity;

use crate::entities::build_schemas;

pub mod entities;

fn main() {
    dotenv().ok();
    build_schemas();
}
