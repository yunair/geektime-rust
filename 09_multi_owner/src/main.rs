use std::sync::Arc;

fn main() {
    let arr = vec![1];
    let handler = std::thread::spawn(move || {
        println!("{:?}", arr);
    });
    handler.join().unwrap();

    let str = Arc::new("str");
    {
        let str = Arc::clone(&str);
        let handler = std::thread::spawn(move || {
            println!("{:?}", str);
        });
        handler.join().unwrap();
    }

    println!("{:?}", str);
}
