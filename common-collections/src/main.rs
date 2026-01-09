use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Hello, world!");
    // can only store value of the same time
    // let v: Vec<u32> =  Vec::new();
    // push to add elements
    // to fetch either immutable reference let val = &v[3] or v.get()
    // v.get(3) returns an option type
    // let mut v = vec![1, 1, 2, "2"];
    // let third = &v[2]; immutable reference
    // get returns without panicking but the direct reference panics with a error
    //    let first = &v[0]; v.push(6);
    // not allowed s at the same time mutable and immutable ref cannot exist
    // iterating over the vector

    let v = vec![1, 2, 3];
    let third = &v[2];
    println!("elenemen is {third}");

    match v.get(2) {
        Some(third) => println!("third"),
        None => println!("none"),
    }
    let mut v = vec![3,4,5,6,5];
    for i in &mut v {
        // dereference operator to be learned more about - todo
        *i += 50;
    }

    for j in &v {
        println!("val is {}", j);
    }

    enum spead {
        Num(i32),
        Decimal(f32),
        Text(String)
    }

    // vector by defualt allows 1 type, so with enums you can have different types
    let enum_vector = vec![spead::Num(2), spead::Decimal(2.333), spead::Text(String::from("here"))];

    match &enum_vector[2] {
        spead::Num(i) => println!("{}", i),
        _ => println!("nothing")
    };
    
    // to_string to convert to string
    // appending strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    
    // does not work as string is a collection of utf 8 chars and which can range from 1-4 bytes so we need to handle indexing differently
    // let hello = "Здравствуйте";
    // let answer = &hello[0]; instead of indexing 
    // for b in "".chars() / "".bytes() {
    //     println!("{b}");
    // }
    // let mut k =  HashMap::new()
    // .insert / .or_insert when key not present

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    for g in "Зд".graphemes(true) {
        println!("{}", g);
    }

    let blue = String::from("blue");
    let red = String::from("red");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(red, 20);

    for (key, value) in &scores {
        println!("{} {}", key, value)
    }

    scores.entry(String::from("yellow")).or_insert(30); // returns a mutable reference

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // so the idea behind *count is that count initially is a mutable reference to the value in the map
    // so when the count needs to be incremented we need to dereference it first
    // map.entry(key)
    // *count += 1;
    // is required because .or_insert(0) returns a mutable reference (&mut V), not the value itself. Let’s break it down.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    // recoverable errors
    let f = File::open("file.txt");

    // error handling can be done using the Result enum
    // enum Result<T, E> { OK(T), Err(E)}
    // so the idea is that we can match on the Result enum and handle the error case

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("err is {?:}", error)
    // };
    // so we can handle the error when for eg we are reading from a file
    // let f = File::open("file.txt").unwrap();
    // let result = match f { Ok(file) =>  file, Error(error) => panic!("error")}
    // use ErrorKind to segregrate the errors, error.kind()
    // errors can also be handled by with unwrap_or_else

    // alternative to this
    // let f = File::open("file.txt").expect("file not found");
    // modify the error with .expect()
    
    // more sophisticated handling
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("file.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("err is {?:}", e)
    //         }
    //     }
    // };
    // 
    //

    // better way to handle errors - these are example functions showing error handling patterns
    // fn read() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut name = String::new();
    //     username_file.read_to_string(&mut name)?;
    //     Ok(name)
    //
    //     // we can also write it as
    //     // let name = String::new();
    //     // File::open("hello.txt")?.read_to_string(&mut name)?;
    //     // Ok(name)
    //     // ? can also be used with a Option type but we can't mix and match them
    // }

    // main() ususally returns () so in order to handle the errors there we need to use a Box trait object
    // Box<dyn Error> is a trait object that can be used to handle any type of error

    // Example of main returning Result:
    // fn main() -> Result<(), Box<dyn Error>> {
    //     let file = File::open("hello.txt")?;
    //     Ok(())
    // }

    // fn read_from_file() -> Result<String, Box<dyn Error>> {
    //     let username_file_result = File::open("hello.txt");
    //
    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(Box::new(e)),
    //     };
    //
    //     let mut username = String::new();
    //
    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(Box::new(e)),
    //     }
    //
    //     // simplied form with ?: which can only be used on a fn with return type as result
    //     // let mut username = String::new();
    //     // File::open("hello.txt")?.read_to_string(&mut username)?;
    //     // Ok(username)
    // }


    

}
