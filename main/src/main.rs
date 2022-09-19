use utils;
fn main() {
    println!("================ Lib Start ================");
    println!("Hello, world!:{}", utils::add(1,2));
    let guess: f32 = "3".parse().expect("Not a float!");
    println!("{}", guess)
}
