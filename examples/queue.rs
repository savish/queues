#[macro_use]
extern crate queues;

use queues::{IsQueue, Queue};

fn main() {
    println!("\nQueue - typical usage");
    println!("--");

    println!("\nCreate a new empty queue:");
    println!("let mut q: Queue<isize> = queue![];");
    let mut q: Queue<isize> = queue![];

    println!("\nAdd elements to it:");
    println!("q.add(1);");
    println!("> {:?}", q.add(1));
    println!("q.add(-2);");
    println!("> {:?}", q.add(-2));
    println!("q.add(3);");
    println!("> {:?}", q.add(3));

    println!("\nCheck the queue's size:");
    println!("q.size();  // Should be 3");
    println!("> {}", q.size());

    println!("\nRemove elements from it:");
    println!("q.remove();  // Should be Ok(1)");
    println!("> {:?}", q.remove());

    println!("\nCheck the queue's size:");
    println!("q.size();  // Should be 2");
    println!("> {}", q.size());

    println!("\nPeek at the next element to be removed:");
    println!("q.peek();  // Should be Ok(-2)");
    println!("> {:?}", q.peek());

    println!("\nCheck the queue's size:");
    println!("q.size();  // Should be 2");
    println!("> {}", q.size());

    println!("\nRemove more elements from it:");
    println!("q.remove();  // Should be Ok(-2)");
    println!("> {:?}", q.remove());
    println!("q.remove();  // Should be Ok(3)");
    println!("> {:?}", q.remove());

    println!("\nPeek at the next element to be removed:");
    println!("q.peek();  // Should raise an error");
    println!("> {:?}", q.peek());

    println!("\nAttempt to remove elements from it:");
    println!("q.remove();  // Should raise an error");
    println!("> {:?}", q.remove());

    println!("\n--\n")
}
