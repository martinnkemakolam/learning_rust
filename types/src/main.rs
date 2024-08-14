use std::io;

struct Person{
    name: String,
    age: u8
}

fn main() {

    // SCALAR
    // INTEGER
    // i[number] is usually the u[number]/2 cause half the length goes to the nagative range
    // u8 from number 0 - 255
    let _val: u8 = 255; 
    let _val: i8 = -128; 

    // u16 from number 0 - 65535
    let _val2: u16 = 65535;
    let _val2: i16 = -32768;

    // u32 from number 0 - 4294967295
    let _val3: u32 = 4294967295;
    let _val3: i32 = -2147483648;

    // u64 from number 0 - 18446744073709551615
    let _val4: u64 = 18446744073709551615;
    // let _val5: arch = 18446744073709551615;

    // u128 from number 0 - 340282366920938463463374607431768211455
    let _val5: u128 = 340282366920938463463374607431768211455;
    // and so on...

    //FLOAT
    // For single precision
    let _decimal1: f32 = 0.44;
    // For double precision
    let _decimal2: f64 = 0.34;
    let _decimal3;
    _decimal3 = -4;

    //BOLEAN
    let _bolean: bool = false;
    let _bolean1 = false;
    let _bolean3;
    _bolean3 = true;
    //STRING
    // takes any string, even emoji's
    let _string1:&str= "ewr";


    //COUMPOUND TYPES
    let tuple: (&str, f64, bool) = ("string", 2.03, false);
    let (a, b, c) = tuple;
    print!("{}", tuple.0);
    print!("this is a: {} \n", a);

    print!("{} {} \n", b, tuple.1);
    print!("{} {} \n", tuple.2, c);


    let array: [&str; 5] = ["", "", "Boy", "", ""];
    let _array1= [3; 3];
    print!("this is the value of _array[3] {} \n", array[2]);

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Must be a number");

    let index: usize = index.trim().parse().expect("Must be a number in 1 - 5");
    let element = array[index];
    println!("{element}");

    // STRUCT
    let _person = Person {
        name: String::from("John"),
        age: 32,
    };

    
    
}
