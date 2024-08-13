mod collectionz;
mod io;
mod taxonomy;

use collectionz::using_vec::run;
use collectionz::using_hashmap::run_hashmap;
use collectionz::using_strings::run_strings;
use taxonomy::plant::litter::{grow, PlantClassification, Creeper, PlantInfo};

fn main() {
    // run();
    // run_hashmap();
    // run_strings();
    let basil = PlantClassification::Creeper(Creeper {
        info: PlantInfo {
            botanica_name: String::from("Basil"),
            age: 1,
            is_grown: true,
            species: String::from("Creeper"),
            family: String::from("Creeper"),
            description: "This is a simple description for a creeper".to_string(),
        }
    });
    println!("{:?}", basil);
    grow(Box::new(basil));

}
