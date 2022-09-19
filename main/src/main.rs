use utils;
use tax;
fn main() {
    println!("================ Lib Start ================");
    println!("Hello, world!:{}", utils::add(1,2));
    let guess: f32 = "3".parse().expect("Not a float!");
    println!("{}", guess);
    let after_tax: f32 = tax::resident(10000.0);
    println!("{}", after_tax);
    utils::counter();

    let r = tax::Resident{salary: 10000.0};
    let nr = tax::NonResident{salary: 10000.0};
    println!("For resident:{} and Non resident:{}.", r.value(), nr.value());
    let area = utils::area(10.0,20.0);
    println!("With area:{}", area)
}
