#![allow(dead_code)]

/// Creating a vector.
fn creating_vec() {
    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{}", x);
    }

    // iterating over a mutable vector
    let mut v = vec![1, 2, 3];
    for x in &mut v {
        *x += 10;
        println!("{}", x);
    }

    // using the mutable iterator
    for x in v.iter_mut() {
        *x += 10;
        println!("{}", x);
    }
}

/// Vec can only store a single type. So you can't push an integer to a vector of strings.
/// But to navigate this, you can use an enum. 
#[derive(Debug)]
enum MixedVec {
    IntVec(i32),
    StrVec(String),
    FloatVec(f32),
}

#[allow(unreachable_patterns)]
fn creating_mixed_vec() {
    let v = vec![
        MixedVec::IntVec(1),
        MixedVec::StrVec(String::from("Hello")),
        MixedVec::FloatVec(1.0),
    ];

    for i in v.iter() {
        match i {
            MixedVec::IntVec(value) => println!("{:?}", value),
            MixedVec::StrVec(value) => println!("{:?}", value),
            MixedVec::FloatVec(value) => println!("{:?}", value),
            _ => println!("Something else"),
        }
    }
    println!("{:?}", v);
}
/// Pushing to a vector.
fn pushing_to_and_extending_vec() {
    let mut v = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    v.extend(v2);
    println!("{:?}", v);

    // push to v
    v.push(7);
    v.push(8);
    v.push(9);

    println!("Printing vec after push: {:?}", v)
}

fn getting_elements_in_vec() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Get the 2 to 4 element
    let second_to_fourth = &v[1..4];
    println!("The second to fourth element is {:?}", second_to_fourth);

    // Get the last element
    let last_ = &v[v.len() - 1];
    println!("The last element is {}", last_);

    // Reverse the list and print it
    let mut v1 = v.clone();
    v1.reverse();
    println!("The reversed list is {:?}", v1);

    // pop the last element from the list
    v1.pop();
    println!("The list after pop is {:?}", v1);

    // print the list with the enumerated values
    println!("The list with enumerated values is: ");
    for (i,v) in v1.iter().enumerate() {
        println!("{}: {}", i, v);
    };
    println!("{:?}", v1.ends_with(&[2]));
    
    // append a mutable list to the list
    v1.append(&mut vec![10,13,11]);
    println!("{:?}", v1);
    
    // get element using `get` method to get the value by the index or return None
    let does_not_exist = v1.get(100);
    let does_exist = v1.get(2);
    println!("{:?}", does_not_exist);
    println!("{:?}", does_exist.unwrap());
    
    // getting error when trying to get an element that doesn't exist
    // let does_not_exist = v1[100];
    // println!("{:?}", does_not_exist);

    // convert the list to a string.
    let stra = v1.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
    println!("{:?}", stra);
}

/// Running the program.
pub fn run() {
    // creating_vec();
    // pushing_to_and_extending_vec();
    getting_elements_in_vec();
    // creating_mixed_vec();
}