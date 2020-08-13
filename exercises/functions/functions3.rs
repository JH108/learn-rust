// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
		const CALL_TIMES: i32 = 10;
    call_me(CALL_TIMES);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
