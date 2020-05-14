fn main() {
    //statement
    let x =5;
    //expression ending with ;
    x;
    x+1;
    //block expressions
    let y= {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
    //unit expression
    let z = {
        2*x;//return ()
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
