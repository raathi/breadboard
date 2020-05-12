#![allow(overflowing_literals)]//To allow the overflow instead of compile time error.
fn main() {
    let decimal = 65.23434_f32;
    let integer:u8 = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is {}",1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as u8 is {}",1000 as u8);
    println!("-1 as u8 is {}",-1i8 as u8);// -1 + 256 = 255
    println!("1000 mod 256 is {}",1000 % 256);// For positive numbers, this is the same as the modulus


    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    /*LITERALS*/
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));//borrowing x, pass by reference
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    /*TYPE INFERENCE*/
    let elem = 5u8;//annotated type
    let mut vec = Vec::new();//unknown type
    vec.push(elem);//now the type infered as u8
    println!("{:?}",vec);

    /*ALIASING*/
    //for better domain representation
    type NanoSecond = u64; //type should be camel case instead of snake case literals
    type Inch = u64;

    #[allow(non_camel_case_types)]
    type u64_t = u64;

    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds : NanoSecond = 5 as u64_t;

    let inches: Inch = 2 as u64_t;
    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);

}
