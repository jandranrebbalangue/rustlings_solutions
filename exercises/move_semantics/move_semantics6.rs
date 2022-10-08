// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();
    let data2 = data.clone();
    get_char(data);

    string_uppercase(&data2);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    let data2 = data.clone();
    let data3 = &data2.to_uppercase();

    println!("{}", data3);
}
