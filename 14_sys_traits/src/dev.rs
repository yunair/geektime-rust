#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

fn main() {
    let dev = Developer {
        name: "Air".to_string(),
        age: 18,
        lang: Language::Rust,
    };
    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    let mut dev1 = dev.clone();
    println!(
        "dev1: {:?}, addr of dev1 name: {:p}",
        dev1,
        dev1.name.as_ptr()
    );
    dev1 = dev.clone();
    println!(
        "dev1: {:?}, addr of dev1 name: {:p}",
        dev1,
        dev1.name.as_ptr()
    );
    dev1.clone_from(&dev);
    println!(
        "dev1: {:?}, addr of dev1 name: {:p}",
        dev1,
        dev1.name.as_ptr()
    )
}
