use std::thread;

fn main() {
    let mut handles = Vec::new();
    for x in 0..10 {
        //move を使ってxの所有権をクロージャーに渡してあげる
        handles.push(thread::spawn(move || {
            println!("Hello, world from thread No.{}", x);
        }));
    }

    for h in handles {
        let _ = h.join();
    }
}