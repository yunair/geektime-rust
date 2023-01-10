use std::mem::size_of_val;

fn main() {
    let name = String::from("Tyr");
    let vec = vec!["Rust", "Elixir", "Javascript"];
    let v = &vec[..];
    let data = (1, 2, 3, 4);
    let c = move || {
        println!("data: {:?}", data);
        println!("v: {:?}, name: {:?}", v, name.clone());
    };
    c();

    println!(
        "data: {}, v: {}, main: {}",
        size_of_val(&data),
        size_of_val(v),
        size_of_val(&c)
    )
}
