pub(crate) fn slice() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {word}");
    // fails because we first take a immutable ref but clear() needs a mutable ref and at a time only 1 possible

    // so we can also modify the first word method to take string slice as an output &str is string slice(works with literals), &String is just a string
}

fn first_word(val: &String) -> &str {
    let bytes  = val.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
        // &str[0..index] is a reference to the string slice so no case of dangling references
        // the first index is inclusive only
        // &str for slice and String for string
         return &val[0..i];
       }
    }
    return &val;
}