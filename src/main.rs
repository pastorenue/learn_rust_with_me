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
    // crate::tdd::run();
    let a = 14;
    let mut b = 6;
    compute(&a,&mut b);
    let mut c = String::new();
    println!("y: {}", as_str(&a, &mut c));
}

fn compute(input: &u32, output: &mut u32) {
    let mut temp = *output;
    if *input > 10 {
        temp = 1;
    }
    if *input > 5 {
        temp *= 2;
    }
    *output = temp;
    println!("Input: {}, Output: {}", input, output);
}

/// Lifetimes
fn as_str<'a>(x: &'a u32, y: &'a mut String) -> &'a str {
    *y = format!("{}", x);
    y.as_str()
}
