fn main() {
    println!("Hello, world!");
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

use test_suite::Rectangle;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_rectangle() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle {width: 10, ..rect1 };
        assert!(rect1.isLargerThan(&rect2),
        "not greater");
    }

    #[test]
    #[should_panic(expected = "rect small")]
    fn test_panic() {
        let rect1 = Rectangle { width: 5, height: 50 };
        rect1.is_small();
    }

    #[test]
    fn test_with_Result() -> Result<(), String> {
        if (add(1, 4) == 5) {
            Ok(())
        } else {
            Err(String::from("unexpected"))
        }
    }
}