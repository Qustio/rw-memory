use std::{thread, time::Duration};

fn main() {
    let mut var = 12804u32;
    println!("Value adress: {:?}", &var as *const _);
    loop {
        thread::sleep(Duration::from_secs(5));
        println!("var: {}", var);
        var += 3;
    }
}
