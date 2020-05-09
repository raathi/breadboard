/// Doc comments type 1.
fn main() {
    //! Doc comments type 2.
    println!("Hello, world!");
    println!("I'm a Rustacean");
    // Single line comment
    /*
     * Multiline Comment
     */
    let x = 5 + /*90 + */ 5;
    println!("Is x '10' or '100'? x = {}", x);

    /*print formats*/
    println!("{} months", 12);
    //positioned arguments
    println!("{0}, this is {1}. {1}, this is {0}", "alice", "bob");
    //named arguments
    println!(
        "{subject} {verb} {object}",
        subject = "the quick brown fox",
        object = "the lazt dog",
        verb = "jump over"
    );
    //special formatt
    println!("{} knows only {:b}", "Computer", 2);
    //padding
    println!("{number:>width$}", number = 5, width = 4);
    println!("{number:>0width$}", number = 5, width = 4);
    #[allow(dead_code)]
    struct Structure(i32);
    //println!("This sruct '{}' wont print...", Structure(33));
    //precision
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);
    // Hello {next arg ("x")} is {second of next two args (0.01) with precision
    //                          specified in first of next two args (5)}
    println!("Hello {} is {:.*}", "x", 5, 0.01);

    println!(
        "{}, `{name:.*}` has 3 characters",
        "Hello",
        3,
        name = "1234.56"
    );
    println!(
        "{}, `{name:>8.*}` has 3 right-aligned characters",
        "Hello",
        3,
        name = "1234.56"
    );
    /*std::fmt::Debug*/
    struct Unprintable(i32);
    #[derive(Debug)]
    struct DebugPrintable(i32);
    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("{:?}", DebugPrintable(33));
    //nested and pretty printing
    println!("{:#?}", Deep(DebugPrintable(7)));
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let peter = Person {
        name: "Peter",
        age: 22,
    };
    println!("{:#?}", peter);
    use std::fmt::{self, Formatter, Display};
    /*std::fmt::Display*/
    struct DisplayableStructure(i32);

    impl fmt::Display for DisplayableStructure {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("{}", DisplayableStructure(78));

    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
            write!(f, "[{},{}]", self.0, self.1)
        }
    }

    println!("{}", MinMax(334, 54));
    println!("{}", MinMax(334, 54));

    #[derive(Debug)]
    struct Complex {
        real: f64,
        img: f64,
    }

    // Similarly, implement `Display` for `Point2D`
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "{} + {}i", self.real, self.img)
        }
    }

    let complex = Complex { real: 33.2, img: 4.1 };
    println!("{}", complex);
    println!("{:?}", complex);


    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", count, v)?
            }
            write!(f, "]")
        }
    }
    println!("{}", List(vec![1, 2, 3, 4]));

    /*FORMATTING*/
   struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(f, "{}: {:.3}° {} {:.3}° {}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
    }


}
