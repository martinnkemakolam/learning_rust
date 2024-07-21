
fn main() {
    // let mut x:i32 = 5;
    let x:i32 = 3;
    println!("Hello, world!{}", x);
    let x = 4;
    {
        // x = "Hello, world";
        let x = x + 1;
        println!("Hello, world!{}", x);
    }
    let x = "Hello, worlr";
    println!("Hello, world!{}", x);


    //const
    const X:i32 = 100_000;
    print!("I mage my first {}", X);

    let r;
    r = "Hello,";
    print!(" C {}", r);
}
