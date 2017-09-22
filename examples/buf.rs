extern crate queues;

use queues::{Buffer, IsQueue};

fn main() {
    println!("\nBuffer - typical usage");
    println!("--");

    println!("\nCreate a new empty buffer:");
    println!("let mut buf: Buffer<isize> = Buffer::new(3);");
    let mut buf: Buffer<isize> = Buffer::new(3);

    println!("\nAdd elements to it:");
    println!("buf.add(1);");
    println!("> {:?}", buf.add(1));
    println!("buf.add(-2);");
    println!("> {:?}", buf.add(-2));
    println!("buf.add(3);");
    println!("> {:?}", buf.add(3));

    println!("\nAttempt to add elements when full:");
    println!("buf.add(-4);  // Should raise an error");
    println!("> {:?}", buf.add(-4));

    println!("\nCheck the buffer's size:");
    println!("buf.size();  // Should be 3");
    println!("> {}", buf.size());

    println!("\nRemove elements from it:");
    println!("buf.remove();  // Should be Ok(1)");
    println!("> {:?}", buf.remove());

    println!("\nCheck the buffer's size:");
    println!("buf.size();  // Should be 2");
    println!("> {}", buf.size());

    println!("\nPeek at the next element to be removed:");
    println!("buf.peek();  // Should be Ok(-2)");
    println!("> {:?}", buf.peek());

    println!("\nCheck the queue's size:");
    println!("buf.size();  // Should be 2");
    println!("> {}", buf.size());

    println!("\nRemove more elements from it:");
    println!("buf.remove();  // Should be Ok(-2)");
    println!("> {:?}", buf.remove());
    println!("buf.remove();  // Should be Ok(3)");
    println!("> {:?}", buf.remove());

    println!("\nPeek at the next element to be removed:");
    println!("buf.peek();  // Should raise an error");
    println!("> {:?}", buf.peek());

    println!("\nAttempt to remove elements from it:");
    println!("buf.remove();  // Should raise an error");
    println!("> {:?}", buf.remove());

    println!("\n--\n")
}
