pub(crate) fn flow() {
    let a = [2, 3, 5];
    for val in a.iter().rev() {
        println!("val is {val}");
    }
    let x: (f64, u16, String) = (500.55, 45, "hi".to_string());
    let update = x.0;
    println!("update is {update}");

    let y = {
        let z = 1;
        z + 7
    };

    println!("y is {y}");

    let _num = if y == 8 { 1 } else {2};

    // loop labels

    let mut count = 0;
    // loop nameing with '
    // end break 'loop_name var name to return if any
    let result = 'first_loop: loop {
        let mut other = 10;

        loop {
            println!("count is {count}");
            if other == 0 {
                break 'first_loop other;
            }

            if (count == 12) {
                break;
            }

            other -= 1;
             
        }
        count += 1;

    };
}