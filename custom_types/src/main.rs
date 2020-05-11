#![allow(dead_code)]//annotating the entire file to allow dead code.

enum Status{
    Rich,Poor,
}
fn main() {
    //!Custom data types mainly using "struct" and "enum"
    //! Constants using "const" and "static"

    /*
      STRUCTURES
      3 types - tuple structs, C structs, Unit struts
     */
    #[derive(Debug)]
    struct Person<'a> {
        //'a defines a lifetime
        name: &'a str,
        age: u8,
    }

    //unit struct
    struct Empty;
    //tuple struct
    struct Pair(i32, i32);
    //two fields
    struct Point {
        x: f32,
        y: f32,
    }

    struct Rectange {
        top_left: Point,
        bottom_right: Point,
    }

    let peter = Person { name: "Peter", age: 28 };
    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };
    println!("Coordinates ({},{})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("Bottom right coordinates ({},{})", bottom_right.x, bottom_right.y);

    //destructure
    let Point { x: top_edge, y: left_edge } = point;//creates variable top_edge and bottom edge assings  values form points - x and y

    let _rectangle = Rectange {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let nil = Empty;

    let pair = Pair(2, 4);
    println!("Pair - ({},{})", pair.0, pair.1);

    //destructure pair
    let Pair(first, second) = pair;
    println!("destructured -> {},{}", first,second);

    //ENUMS
    enum WebEvent{
        //unit type
        PageLoad,
        PageUnload,
        //tuple type
        Keypress(char),
        Paste(String),
        //c type
        Click{x: i64, y:i64}
        //so any valid struct can be a type of enum
    }

    fn on_event(event: WebEvent){
        match event {
            WebEvent::PageLoad => println!("Page loaded"),
            WebEvent::PageUnload => println!("Page unloaded"),
            //destructure  tuple
            WebEvent::Keypress(keyed_char)=> println!("Keyed {}",keyed_char),
            WebEvent::Paste(clipboard)=> println!("Pasted \"{}\"",clipboard),
            //destructure c type
            WebEvent::Click {x,y} => {
                println!("Clicked at {},{}", x,y);
            }
        }
    }

    let page_load = WebEvent::PageLoad;
    let pasted = WebEvent::Paste("sample text".to_owned());//owned string from a string slice
    let click = WebEvent::Click{x: 33, y:23};
    on_event(page_load);
    on_event(pasted);
    on_event(click);

    //type aliases enum
    enum VeryLongEnumOfThingsSample{
        Add,Subtract,
    }
    type Operations = VeryLongEnumOfThingsSample;
    let add = Operations::Add;

    impl Operations{
        fn run(&self, x:i32, y:i32) -> i32{
            match  self {
                Self::Add => x+y,
                Self::Subtract => x-y,
            }
        }
    }

    println!("Operation Result {}", Operations::run(&Operations::Add, 10,2));

    /*USE*/
    use crate::Status::*;//import from unnamed module
    let status = Rich;

    match status {
        Rich => println!("Rich"),
        Poor => println!("Poor"),
    }

    /*C-LIKE*/
    // enum with implicit discriminator (starts at 0)
    enum Number {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    println!("Zero is {}",Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    /*TESTCASE LINED-LIST*/
    enum List{
        Cons(u32, Box<List>),
        Nil,
    }

    impl List{
        fn new() -> List{
            List::Nil
        }

        fn prepend(self, elem: u32) -> List{
            List::Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32{
            // `self` has to be matched, because the behavior of this method
            // depends on the variant of `self`
            // `self` has type `&List`, and `*self` has type `List`, matching on a
            // concrete type `T` is preferred over a match on a reference `&T`
            match *self {//no idea right now
                // Can't take ownership of the tail, because `self` is borrowed;
                // instead take a reference to the tail
                List::Cons(_, ref tail) => 1+tail.len(),
                List::Nil => 0
            }
        }

        fn stringify(&self) -> String{
           match *self {
                List::Cons(head, ref tail) => {
                    format!("{},{}", head, tail.stringify())
                },
               List::Nil => format!("Nil"),
           }
        }
    }

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(4);
    println!("{} of len {}", list.stringify(), list.len());

    /*CONSTANTS*/
    //two types const and static
    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {n > THRESHOLD}

    let n =16;
    println!("This is {}", LANGUAGE);
    println!("{} is {}",n, if is_big(n) {"big"} else{"small"});
    //THRESHOLD = 5;//const cannot be modified.
}