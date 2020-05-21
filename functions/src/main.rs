use std::alloc::System;

//#![feature(never_type)]
fn main() {
    /*FUNCTIONS*/
    is_divisible_by(10,3);
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);

    // Function that returns a boolean value
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        // Corner case, early return
        if rhs == 0 {
             return false;
        }

        // This is an expression, the `return` keyword is not necessary here
        lhs % rhs == 0
    }

    // Functions that "don't" return a value, actually return the unit type `()`
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // When a function returns `()`, the return type can be omitted from the
// signature
    fn fizzbuzz_to(n: u32) {
        for n in 1..n + 1 {
            fizzbuzz(n);
        }
    }

    /*METHODS*/
    //functions attached to objects access the data of the objects using self keyword

    struct Point {
        x: f64,
        y: f64,
    }

    // Implementation block, all `Point` methods go in here
    impl Point {
        // This is a static method
        // Static methods don't need to be called by an instance
        // These methods are generally used as constructors
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // Another static method, taking two arguments:
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // This is an instance method
        // `&self` is sugar for `self: &Self`, where `Self` is the type of the
        // caller object. In this case `Self` = `Rectangle`
        fn area(&self) -> f64 {
            // `self` gives access to the struct fields via the dot operator
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            // `abs` is a `f64` method that returns the absolute value of the
            // caller
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // This method requires the caller object to be mutable
        // `&mut self` desugars to `self: &mut Self`
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // `Pair` owns resources: two heap allocated integers
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // This method "consumes" the resources of the caller object
        // `self` desugars to `self: Self`
        fn destroy(self) {
            // Destructure `self`
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);

            // `first` and `second` go out of scope and get freed
        }
    }

    let rectangle = Rectangle{
        p1 : Point::origin(),
        p2: Point::new(3.0,4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());//instance methods invoke by dot operator
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle{//to invoke mutable method translate get mutable object square
        p1: Point::origin(),
        p2: Point::new(1.0,1.0),
    };

    square.translate(2.0,2.0);//method requires caller to be mutable

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();//value moved here, by destructuring
    // pair.destroy();.//compiled time error


    /*CLOSURE*/
    //closure in rust is lambdas
    // they are fn with "||" around input variables
    //optional boday delimination
    //ability to capture outer env variable

    fn increment_function(i: i32) -> i32 {i+1};

    let increment_closure_annotated = |i:i32| ->i32 {i+1};
    //|i| i + 1 optional delimination
    let increment_closure_inferred = |i/*:infered data type*/|/*-> infered return type*/ i+1;//input and return type inferred

    let i = 1;
    // Call the function and closures.
    println!("function: {}", increment_function(i));
    println!("closure_annotated: {}", increment_closure_annotated(i));
    println!("closure_inferred: {}", increment_closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;//return type infered as i32
    println!("infered no arg func return: {}",one());

    //capturing variable - closure
    //capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing
    let color = String::from("green");

    let print = || println!("Color: {}",color);//captured(borrowed) immutable reference

    print();
    let _reborrow = &color;//allowed immutable borrow any times
    //let _move = color; //move not allowed here since print having immutable reference
    print();
    let _move = color; //move allowed here after final print()

    let mut count = 0;

    // A closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately
    // borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`.
    let mut inc = || {
      count +=1;
        println!("Count is {}",count);
    };

    inc();//here borrow inc as mutable doesnot works since it is a mutable method
    //hence inc is marked mut func
    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    //inc();//here compile time error, since we cannot create multiple mutable ref to count.

    // A non-copy type.
    let movable = Box::new(3);
    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        std::mem::drop(movable);
    };
    // `consume` consumes the variable so this can only be called once.
    consume();
    //consume();
    // ^ TODO: Try uncommenting this line.


    //Using move before vertical pipes forces closure to take ownership of captured variables:
    // `Vec` has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // println!("{:?}", haystack);//compile time errorm, since closure moved the vec

    // println!("There're {} elements in vec", haystack.len());
    // ^ Uncommenting above line will result in compile-time error
    // because borrow checker doesn't allow re-using variable after it
    // has been moved.

    // Removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.


    //closure as input parameter
    //Rust chooses how to capture variables on the fly mostly without type annotation,
    // this ambiguity is not allowed when writing functions.
    // When taking a closure as an input parameter,
    // the closure's complete type must be annotated using one of a few traits
    fn apply<F>(f: F )where F: FnOnce()->(){
        f();// The closure takes no input and returns nothing.
        // ^ TODO: Try changing this to `Fn` or `FnMut`.
    }

    fn apply_to_3<F>(f:F) -> i32 where
        // The closure takes an `i32` and returns an `i32`.
        F:Fn(i32)->i32{// A function which takes a closure and returns an `i32`.
        f(3)
    }

    let greeting = "Hello";
    let mut farewell = "goodbye".to_owned();//from borrowed(&str) to owned(String).

    let dairy = ||{
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        std::mem::drop(farewell);//moved value
    };
    // Call the function which applies the closure.
    apply(dairy);

    let double = |x| 2*x;
    let triple = |x| x*3;
    println!("doubled 3: {}",apply_to_3(double));
    println!("doubled 3: {}",apply_to_3(triple));


    //type anonymity -closure
    //using a closure as a function parameter requires generics, which is necessary because of how they are defined
    fn apply_anonymity<F>(f:F) where F:Fn() -> (){//type anonymous closure argument
        f()
    }

    let x  =7;
    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}",x);
    apply(print);

    //input function
    //a function that takes a closure as parameter,
    // then any function that satisfies the trait bound of that closure
    // can be passed as a parameter
    // Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // Define a wrapper function satisfying the `Fn` bound
    fn function() {
        println!("I'm a function!");
    }

    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    //output parameters - closure
    fn create_fn() -> impl Fn(){
        let text = "Fn".to_owned();
        move || println!("This is a {}",text)//text dies here, hence move needed - forces to takes the ownership of text.
    }

    fn create_fnmut() -> impl FnMut(){
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    //examples in std -closure
    //iterator::any
    // pub trait Iterator{
    //     type Item;
    //
    //     // `any` takes `&mut self` meaning the caller may be borrowed
    //     // and modified, but not consumed.
    //     fn any<F>(&mut self, f: F) -> bool where
    //     // `FnMut` meaning any captured variable may at most be
    //     // modified, not consumed. `Self::Item` states it takes
    //     // arguments to the closure by value.
    //         F: FnMut(Self::Item) -> bool {}
    // }

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec1.into_iter().any(|x| x == 2));

    //searching through iterators - std library usage closure
    //Iterator::find()
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));//Destructuring uses &, ref, and ref mut
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));

    //just to understanding destructuring using &
    let xx = &10;
    let print_x = |&x| {//destructuring here
        if x==10{
        println!("{}",true);
        }else {
            println!("{}",false)
        }
    };
    print_x(xx);

    //to find index in iterator, Iterator::position
    let vec = vec![1, 9, 3, 3, 13, 2];
    let index_of_first_even_number = vec.iter().position(|&x| x%2==0 );
    assert_eq!(index_of_first_even_number, Some(5));

    /*HIGH ORDER FUNCTION*/
    //functions that take one or more functions and/or produce a more useful function
    //Rust Functional Programming - HOFs and Lazy Iterators.

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 10000");
    let upper = 10000;

    use std::time::{Instant};
    //imperative approach
    let start_time = Instant::now();
    let mut accumulated_sum = 0;
    for n in 0.. {//o..to infinity
        let n_squared = n*n;
        if n_squared >= upper{
            break;
        }else if(is_odd(n_squared)) {
            accumulated_sum += n_squared;
        }
    }
    println!("imperative style: {} takes {:?} ", accumulated_sum, start_time.elapsed());

    //functional approach
    let start_time = Instant::now();
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
            .take_while(|&n_squared| n_squared < upper) // Below upper limit
            .filter(|&n_squared| is_odd(n_squared))     // That are odd
            .fold(0, |accumulated_sum, n_squared| accumulated_sum + n_squared); // Sum them

    println!("functional style: {} takes {:?}", sum_of_squared_odd_numbers, start_time.elapsed());

    /*DIVERGING FUNCTIONS(experimental)*/
    //functions never return. They are marked using !, which is an empty type.
    // like below;
    // fn foo() -> ! {
    //     panic!("This call never returns.");
    // }
    // let x: ! = panic!("This call never returns.");
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,//here in this case, we may use ! type to match u32 version if required as per the use case.
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
    // "!" also the return type of functions that loop forever (e.g. loop {}) like network servers
    // or functions that terminates the process (e.g. exit()).
}
