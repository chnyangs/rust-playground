mod area;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn area(width: f64, height: f64) -> f64 {
    let width = area::Width{val:width};
    let height = area::Height{val:height};
    let size = area::Size{width, height};
    size.cal_area()
}
pub fn counter(){
    let mut ten_count = 0;
    'ten_loop: loop {
        let mut bin_count = 1;
        'bin_loop: loop {
            println!("with binary loop: {}", bin_count);
            if bin_count == 2{
                break 'bin_loop;
            }
            bin_count += 1;
        }
        ten_count += 1;
        println!("with ten loop: {}", ten_count);
        if ten_count == 10 {
            break 'ten_loop;
        }
    }
}