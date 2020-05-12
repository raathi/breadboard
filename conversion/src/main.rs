use std::convert::{TryFrom, TryInto};
use std::fmt::Formatter;

fn main() {
    /*FROM AND INTO*/
    //inherently linked, shared same trait.

    //From trait
    let string = "hello";//str
    let my_string = String::from(string);

    //use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    let number = Number::from(43i32);
    println!("From number is {:?}", number);
    let into_number: Number = 65i32.into();//i32 to num for free since from implemented
    println!("To number is {:?}", into_number);

    /*TRYFROM AND TRYINTO*/
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8i32)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    /*TO AND FROM STRINGS*/
    //To String to impl ToString trait,
    //better option is to impl Display trait
    struct Circle {
        radius: i32,
    }
    ;

    impl std::fmt::Display for Circle {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            writeln!(f, "Radius is {}", self.radius)
        }
    }

    println!("Circle -> {}",Circle{radius:6}.to_string());
    //parse from string
    let parsed:i32 = "5".parse().unwrap();//string to integer
    let turbo_parsed = "50".parse::<i32>().unwrap();//turbofish syntax
    println!("Sum is {}", parsed+turbo_parsed);

}


