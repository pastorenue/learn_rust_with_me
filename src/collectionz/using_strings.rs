pub fn run_strings() {
    creating_new_string();
}

fn creating_new_string() {
    let mut s = String::new();
    s.push_str("foo");
    println!("{}", s);

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    // using format
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    let s4 = format!("{s1}-{s2}-{s3}"); // shorthand
    println!("{}", s4);
    println!("{}", s);

    // indexing
    let s = String::from("hello");
    let h = s.chars().nth(0).unwrap();
    println!("{}", h);

    // slicing
    let s = String::from("Hello, World!");
    let f = &s[0..1];
    let hello = &s[0..5];
    let world = &s[6..13];
    println!("{} {} {}", hello, world, f);
}