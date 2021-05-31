    pub fn run(){
    // Primitive data types
    let _x = 5;
    let mut x = 12;
    println!("{}", x);
    x = 10;
    println!("{}", x);
    
    // Primitive times
    // integer - i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    let _i = 12;
    // float - f32, f64
    let _f = 12.6;
    // bool - true/false
    let _b = true;
    // char 
    let _c: char = 'z';  //  single quotes are necessary

    // Tuples
    let t : (i32, f64, char) = (42, 6.12, 'j');
    let (_1, _2, _3) = t; // Destructuring
}