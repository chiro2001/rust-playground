use std::{thread, time::Duration};

fn main() {
  thread::spawn(|| {
    for i in 1..10 {
      println!("{i} in child thread");
      thread::sleep(Duration::from_millis(1));
    }
  }).join().unwrap();
  for i in 1..5 {
    println!("{i} in main thread");
    thread::sleep(Duration::from_millis(1));
  }
}