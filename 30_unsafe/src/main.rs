use std::{mem::transmute, slice::from_raw_parts_mut, str::from_utf8_unchecked_mut};
fn main() {
    use_libc();

    let mut s = "hello world1".to_string();
    println!("{:?}", split_mut(&mut s, ' ').unwrap())
}

fn split(s: &str, sep: char) -> Option<(&str, &str)> {
    let pos = s.find(sep);
    pos.map(|pos| {
        let len = s.len();
        let sep_len = sep.len_utf8();

        // SAFETY: pos 是 find 得到的，它位于字符的边界处，同样 pos + sep_len 也是如此
        // 所以以下代码是安全的
        unsafe { (s.get_unchecked(0..pos), s.get_unchecked(pos + sep_len..len)) }
    })
}

fn split_mut(s: &mut str, sep: char) -> Option<(&mut str, &mut str)> {
    let pos = s.find(sep);

    pos.map(|pos| {
        let len = s.len();

        let sep_len = sep.len_utf8();
        let ptr = s.as_mut_ptr();
        unsafe {
            let pre = from_raw_parts_mut(ptr, pos);
            let ptr = ptr.add(pos + sep_len);
            let post = from_raw_parts_mut(ptr, len - pos - sep_len);

            (from_utf8_unchecked_mut(pre), from_utf8_unchecked_mut(post))
        }
    })
}

fn use_libc() {
    let data = unsafe {
        let p = libc::malloc(8);
        let arr: &mut [u8; 8] = transmute(p);
        arr
    };
    data.copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    println!("data: {:?}", data);
    unsafe { libc::free(transmute(data)) };
}
