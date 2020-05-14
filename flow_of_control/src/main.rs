

fn main() {
    /*IF ELSE*/
    //no parentheses required
    let n = 5;
    if n < 0 {
        println!("{} is negative.", n);
    } else if n > 0 {
        println!("{} is positive.", n);
    } else {
        println!("{} is neutral.", n);
    }
    //with expression block
    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    /*LOOP*/
    let mut count = 0u32;
    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 500 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }

    /*NESTING AND LABELS*/
    'outer: loop{
        println!("Entered the outer loop");
        'inner: loop{
            println!("Entered the inner loop");
            //break;//inner loop break;
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    /*RETURN FROM LOOPS*/
    let mut counter = 0;

    let result = loop{
        counter +=1;
        if counter==10{
            break counter*2;//to return a result on break
        }
    };
    println!("loopbreak return {}", result);

    /*WHILE*/
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    /*FOR*/
    //for and range
    //"for in" to iterate through n itrator,
    //easiest iterator creation is range notation "a..b"

    //a(inclusive)..b(exclusive)
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..20 {//1 to 19
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    //a(inclusive)..=b(inclusive)
    for n in 1..=20 {//1 to 20
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    //for and iterator
    //by default for in applies into_iter() to collection
    //into_iter, iter, iter_mut converts collection to iterator
    let names = vec!["Bob", "Frank", "Ferris"];

    //iter()
    for name in names.iter(){//borrows each element of the collection on each iteration
        match name {
            &"Ferris" => {
                println!("There is a rustacean amoung us!");
            }
            _ => println!("Hello {}",name),
        }
    }
    println!("{:?}",names);

    //into_iter()
    for name in names.into_iter(){//moves the collection within  the loop once iterated
        match name{
            "Ferris" => println!("There is a rustaean among us!"),
            _ => println!("Hello {}",name),
        }
    }
   // println!("{:?}",names); //compilation error here, since value moved into the loop.

   //iter_mut()
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for mut name in names.iter_mut(){//should be mutable collection, since we borrowing as mutable.
        *name = match name{
            &mut "Ferris" => "There is a rustacean amoung us!",
            _ => "Hello ",
        }
    }
    println!("{:?}",names);//updated vector


    /*MATCH*/
    let number = 11;
    println!("Any idea about number {}", number);

    //single value match
    match number{
        1 => println!("One"),
        2 | 3 | 5 | 7| 11 => println!("This is a prime"),
        //matching an inclusive range
        13..=19 => print!("A teen"),
        //default cases
        _ => println!("Ain't Special"),
    }

    let boolean = true;
    //match as an expression
    let binary = match boolean{
        true => 1,
        false => 0,
    };
    println!("{}->{}",boolean,binary);

    /*DESTRUCTURING*/
    //multiple ways - tuples, enums, pointers, structures

    //tuple destructing
    let pair = (0,-2);
    println!("Pair is {:?}",pair);

    match pair{//using match to destructure a tuple
        //desctructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),//means don't bind to a variable.
    }

    //enum destructuring
    #[allow(dead_code)]
    enum Color{
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
        // Don't need another arm because all variants have been examined
    }

    //pointer destructuring
    //dereference is *
    //destructuring uses &, ref and ref mut


    //reference on primitives
    let mut integer = &4;
    let mut copied_integer = integer;//reference copied
    //println!("{:p} ={} {:p} = {}", integer,integer, copied_integer,copied_integer);
    //reference on objects
    let reference = String::from("were");
    let s = reference;
   // println!("reference is alive {}",reference);//reference moved, compile time error

    let reference = &4;
    match reference{
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val=> println!("Got the value via destructuring {:?}",val)
    }

    match *reference {//derefence to avoid &
        val => println!("Got a value via dereferencing {:?}",val),
    }

    //what if the right side is not reference but we need a reference
    let not_a_reference = 3;
    //lets create a reference using ref on eft side
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;
    //use ref keyword to create a reference in match
    match value{
        //create a reference for integer using ref in match
        ref value_ref => println!("Got a reference from a value {:?}", value_ref),
    }

    //using ref mut
    match mut_value{
        ref mut m => {
            //got a reference m mutable
            *m +=11;// then dereference it to add or change the value
            println!("Changed the value as {}",m);
        }
    }

    //structs dereferencing
    struct Foo{
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo{x:(1,2),y:2};
    match foo{
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y);
    }

    //guards dereferencing
    //match guard can be added to further filter the arm
    let pair = (2, -2);

    match pair{
        (x,y) if x==y => println!("They are twins"),
        //              ^ if condition is a guard
        (x,y) if x+y == 0 => println!("Antimatter kaboom!"),
        (x,_) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    //binding
    //indirect access to variable make it impossible to brnach
    //like a variable return by a method()
    fn age() -> u32{
        15
    }

    match age(){//indirect variable access, unless we assign the return type to a variable
        0           =>  println!("I'm not born yet I guess"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),

    }

    //binding to destructure
    fn some_number() -> Option<u32>{
        Some(42)
    }

    match some_number(){
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _            => (),
    }
}
