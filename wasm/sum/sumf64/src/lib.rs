#![allow(unused)]
fn main() {
    #[no_mangle]
    pub extern "C" fn sum(x: f64, y: f64) -> f64 {
        x + y
    }
}
