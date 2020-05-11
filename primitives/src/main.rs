use std::path::Display;

fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    let default_integer = 4;
    let mut infered_type = default_integer;
    infered_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    //mutable = true;
    //shdowing
    let mutable = true;

    let readable_integer = 100_000_000;

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    /*TUPLES*/
    fn reverse(pair: (i32,bool)) -> (bool, i32){
        (pair.1,pair.0)
    }
    println!("{:?}", reverse((200,true)));

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    //tuple of tuples
    let tuple_of_tuples = ((1,2,3,4),true,(-3,4,3.3));
    println!("{:?}", tuple_of_tuples);

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    use std::fmt::{self,Display,Formatter};
    impl fmt::Display for Matrix{
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            writeln!(f,"");
            writeln!(f, "({} {})", self.0, self.1)?;
            write!(f, "({} {})", self.2, self.3)
        }
    }
    println!("{}",matrix);

    fn transpose2X2(matrix: Matrix) -> Matrix{
        Matrix(matrix.0,matrix.2, matrix.1, matrix.3)
    }
    println!("transpose = {}",transpose2X2(matrix));

    /*ARRAYS & SLICES*/
    let x = 10;
    let mut y = x;
    println!("{} {}",x, y);
    y = 12;
    println!("{} {}",x, y);

    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [1; 500];//Fill array with default value.

    use std::mem;
    println!("{:?}", xs);
    println!("array size: {}", xs.len());
    println!("array xs occupies bytes: {}", mem::size_of_val(&xs));
    println!("array ys occupies bytes: {}", mem::size_of_val(&ys));

    fn analyze_slice(slice: &[i32]){//func borrows slice, cannot be a array since size unknown
        println!("first element of the slice: {} and total len {}", slice[0], slice.len());
    }

    println!("whole array as slice");
    analyze_slice(&xs);
    analyze_slice(&xs[0..=4]);//inclusive 4
    analyze_slice(&ys[1..8]);//exclusive 8

}
