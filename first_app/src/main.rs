static MESSAGE: &str = "Hello Rust!";
static mut COUNTER: u32 = 0;

fn click() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    println!("{}", MESSAGE);
    click();
    click();
    unsafe {
        let current_state_of_counter = COUNTER;
        println!("COUNTER: {}", current_state_of_counter);
    }
}