extern crate queues;

use queues::{CircularBuffer, IsQueue};

fn main() {
    println!("\nCircular buffer - typical usage");
    println!("--");

    println!("\nCreate an empty circular buffer with capacity 5:");
    println!("let mut cbuf: CircularBuffer<isize> = CircularBuffer::new(5);");
    let mut cbuf: CircularBuffer<isize> = CircularBuffer::new(5);

    println!("\nAdd elements to it:");
    println!("cbuf.add(1);");
    println!("> {:?}", cbuf.add(1));
    println!("cbuf.add(-2);");
    println!("> {:?}", cbuf.add(-2));
    println!("cbuf.add(3);");
    println!("> {:?}", cbuf.add(3));

    println!("\nCheck the buffer's size:");
    println!("cbuf.size();  // Should be 3");
    println!("> {}", cbuf.size());

    println!("\nRemove elements from it:");
    println!("cbuf.remove();  // Should be Ok(1)");
    println!("> {:?}", cbuf.remove());

    println!("\nCheck the buffer's size:");
    println!("cbuf.size();  // Should be 2");
    println!("> {}", cbuf.size());

    println!("\nPeek at the next element to be removed:");
    println!("cbuf.peek();  // Should be Ok(-2)");
    println!("> {:?}", cbuf.peek());

    println!("\nFill the buffer:");
    println!("cbuf.add(-7);");
    println!("> {:?}", cbuf.add(-7));
    println!("cbuf.add(8);");
    println!("> {:?}", cbuf.add(8));
    println!("cbuf.add(-9);");
    println!("> {:?}", cbuf.add(-9));

    println!("\nCheck the buffer's size:");
    println!("cbuf.size();  // Should be 5");
    println!("> {}", cbuf.size());

    println!("\nAdd a new element to the buffer:");
    println!("cbuf.add(10);  // Should be OK(Some(-2))");
    println!("{:?}", cbuf.add(10));

    println!("\nCheck the buffer's size:");
    println!("cbuf.size();  // Should still be 5");
    println!("> {}", cbuf.size());

    println!("\n--\n");
}
