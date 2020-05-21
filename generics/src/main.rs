struct A;

struct Single(A);//single type

struct SingleGen<T>(T);//generic type


fn main() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(4i32);
    let _char = SingleGen('a');
}
