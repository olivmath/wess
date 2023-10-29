#![allow(unused)]
fn main() {
    #[no_mangle]
    pub extern "C" fn sum(x: i64, y: i64) -> i64 {
        x + y
    }
}
