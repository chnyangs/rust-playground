pub fn resident(salary:f32) -> f32 {
    let after_tax: f32 = salary;
    // 18,201 – $45,000 19 cents for each $1 over $18,200
    // 45,001 – $120,000 $5,092 plus 32.5 cents for each $1 over $45,000
    // $120,001 – $180,000 $29,467 plus 37 cents for each $1 over $120,000
    // $180,001 and over $51,667 plus 45 cents for each $1 over $180,000
    return if salary <= 18200.0 {
        after_tax
    }  else {
        100.0
    }
}

pub struct Resident{
    pub salary: f32,
}

pub struct NonResident{
    pub salary: f32
}

impl Resident{
    pub fn value(&self) -> &f32{
        let rate = 0.9;
        println!("For Resident with rate:{}", rate);
        &self.salary
    }
}

impl NonResident{
    pub fn value(&self) -> f32 {
        let rate = 0.9;
        println!("For Non Resident with rate:{}", rate);
        &self.salary * rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =resident(18000.0);
        assert_eq!(result, 18000.0);
    }
}
