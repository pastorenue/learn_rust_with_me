use std::collections::HashMap;


pub fn run_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"),  50);
    scores.insert(String::from("Green"), 30);


    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
    println!("{:?}", scores.capacity());

    // get all the keys
    let keys = &scores.keys();
    println!("{:?}", keys);

    // get all values
    let values = &scores.values();
    println!("{:?}", values);

    // get all items
    let items = &scores.iter();
    println!("{:?}", items);

    // you can also create a hashmap from an iterator
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    let map = HashMap::from([
        (String::from("Blue"), 10),
        (String::from("Yellow"), 50),
    ]);
    println!("{:?}", map);
}

