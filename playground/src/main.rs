#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String),
}
fn main() {
    let x = Either::Right(String::from("Hello world"));
    let value = match x {
        Either::Left(n) => n,
        Either::Right(s) => s.len(),
    };
    println!("{value}");
    println!("{:?}", x);
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}
fn main2() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}

use std::{sync::Arc, thread};
fn main3() {
    let s = String::from("Hello world");
    let a = Arc::new(&s);
    // let a = Arc::new(s);
    let a2 = Arc::clone(&a);
    let t = thread::spawn(move || a2.len());
    let len = t.join().unwrap();
    println!("{} {}", a, len);
}

fn reverse(v: &mut Vec<String>) {
    let n = v.len();
    for i in 0..n / 2 {
        let p1 = &mut v[i] as *mut String;
        let p2 = &mut v[n - i - 1] as *mut String;
        unsafe {
            std::ptr::swap_nonoverlapping(p1, p2, 1);
        }
    }
}
