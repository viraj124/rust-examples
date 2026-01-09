#[derive(Debug)]

//struct {var: type, ...}
// impl struct_name { /
//     &self reference to the struct instance, for two instances mention that explicitly
//     fn method_name(&self) -> return_type {
//         // body
//     }
// }
// print => dbg!(&strct)
// struct_object::impl_fun_name(...args)
// copying struct
// let s1 = User {name: vir, ...s1}
// tuple structs have no name only types
// struct color(u32, u32, u64)
// let c = color(2, 3, 45)
struct rect {
    length: u64,
    width: u64
}

impl rect {
    fn area(&self) -> u64 {
        self.length * self.width
    }

    fn compare(&self, rect: &rect) -> bool {
        self.length > rect.length && self.width > rect.width
    }
}

impl rect {
    fn sq(val: u64) -> Self {
        Self {
            length: val,
            width: val
    }
    }
}

pub(crate) fn structs() {
 let rect1 = rect {
        length: 34,
        width: 45
    };

    let rect2 = rect {
        length: 35,
        width: 40 
    };

    println!("rect is {rect1:#?}");
    dbg!(&rect1);

    rect1.compare(&rect2);

    let sq = rect::sq(4);


       struct User {
        name: String,
        value: u64,
        active: bool
    }

    let mut user = User {
        name: String::from("value"),
        value: 34,
        active: true
    };
    user.name = String::from("anotheremail@example.com");

    let mut user1 = User {
        active: true,
        ..user
    };

    struct color(u64, u32);

    let c = color(6, 8);
}