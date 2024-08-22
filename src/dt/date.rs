use chrono::{NaiveDate, NaiveDateTime};
use std::ops::AddAssign;
use std::sync::{Arc, Mutex};
use std::fmt::Display;
use std::ops::Deref;

pub fn get_native_date() -> NaiveDateTime {
    let dt = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(13, 10, 50).unwrap();
    println!("{}", dt);
    println!("Date: {}", dt.date());
    println!("Time: {}", dt.time());
    println!("Formatted Datetime: {}", dt.format("%Y_%m_%dT%H_%M_%S").to_string());

    // parsing from string
    let d = NaiveDateTime::parse_from_str("2024-12-23 12:56:34", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("{}", d);
    d
}

pub fn working_with_mutex() {
    let f = test_custom_box();
    println!("Deref of LocalGuardianf: {}", *f);
    let m = Arc::new(Mutex::new(5));
    let c = Arc::clone(&m);

    let _r = std::thread::spawn(move || {
        let mut _lock = c.lock().unwrap();
        *_lock = 10;
    }).join();

    let _lock = m.lock().unwrap_or_else(|mut e| {
        **e.get_mut() = 1;
        m.clear_poison();
        e.into_inner()
    });
    assert_eq!(m.is_poisoned(), false);
    assert_eq!(*_lock, 10);
    println!("{:?}", m);
}

/// Using Sized
/// https://doc.rust-lang.org/std/marker/trait.Sized.html
/// 
fn using_sized<T: Sized>(x: T) -> T
    where T: PartialOrd + Display + AddAssign<i32> {
    x
}

#[derive(Debug)]
struct LocalGuardian<T> {
    data: T,
    is_legal: bool
}

impl<T> Deref for LocalGuardian<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> LocalGuardian<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            is_legal: true
        }
    }
}

fn test_custom_box() -> LocalGuardian<i32> {
    let l = LocalGuardian::new(10);
    l
}