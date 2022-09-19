
pub struct Width{
    pub val: f64,
}
pub struct Height{
    pub val: f64,
}
impl Width{
    fn value(&self) -> &f64 {
        &self.val
    }
}
impl Height{
    fn value(&self) -> &f64 {
        &self.val
    }
}
pub struct Size{
    pub width: Width, pub height:Height
}

impl Size{
    pub fn cal_area(&self) -> f64{
        self.width.value() * self.height.value()
    }
}