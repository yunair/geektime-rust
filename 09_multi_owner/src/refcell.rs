use std::cell::RefCell;
fn main() {
    let data = RefCell::new(1);
    // 为什么要把获取和操作可变借用的两句代码，用花括号分装到一个作用域下？
    // 因为根据所有权规则，在同一个作用域下，我们不能同时有活跃的可变借用和不可变借用。通过这对花括号，我们明确地缩小了可变借用的生命周期，不至于和后续的不可变借用冲突。
    {
        // 获得 RefCell 内部数据的可变借用
        let mut v = data.borrow_mut();
        *v += 1;
    }
    println!("data: {:?}", data.borrow());
}
