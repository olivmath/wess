#![allow(unused)]
fn main() {
    #[no_mangle]
    pub extern "C" fn sum(x: f32, y: f32) -> f32 {
        x + y
    }
}
