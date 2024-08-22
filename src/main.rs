#![allow(unused_imports)]

mod collectionz;
mod io;
mod taxonomy;
mod dt;
mod traits;
mod tdd;

use collectionz::using_vec::run;
use collectionz::using_hashmap::run_hashmap;
use collectionz::using_strings::run_strings;
use taxonomy::plant::litter::{grow, PlantClassification, Creeper, PlantInfo};
use dt::date::{get_native_date, working_with_mutex};
use crate::traits::UserAge;

fn main() {
    // run();
    // run_hashmap();
    // run_strings();
    // get_native_date();
    // working_with_mutex();
    // let user_age = crate::traits::try_from(10);
    // let user_age_2 = crate::traits::try_into(4);
    // let user_age_3: UserAge = 6.into();
    // println!("User age: {}", user_age.unwrap());
    // println!("User age: {}", user_age_2.unwrap());
    // println!("User age: {}",UserAge::from(5));
    // println!("User age: {}", user_age_3);
    crate::tdd::run();
}
