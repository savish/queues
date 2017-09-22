extern crate queues;

use queues::{CircularBuffer, IsQueue};

fn main() {
    println!("\nCircular buffer with default values - typical usage");
    println!("--");

    println!("\nCreate a circular buffer with capacity 3 and default -1:");
    println!("let mut cbuf_def: CircularBuffer<isize> = CircularBuffer::with_default(3, -1);");
    let mut cbuf_def: CircularBuffer<isize> = CircularBuffer::with_default(3, -1);

    println!("\nCheck the buffer's size - should always be equal to the capacity:");
    println!("cbuf_def.size();  // Should be 3");
    println!("> {}", cbuf_def.size());

    println!("\nPeek at the next element to be removed:");
    println!("cbuf_def.peek();  // Should be Ok(-1)");
    println!("> {:?}", cbuf_def.peek());

    println!("\nAdd a new element to the buffer:");
    println!("cbuf_def.add(45);  // Should be Ok(Some(-1))");
    println!("{:?}", cbuf_def.add(45));

    println!("\nPeek at the next element to be removed:");
    println!("cbuf_def.peek();  // Should be Ok(-1)");
    println!("> {:?}", cbuf_def.peek());

    println!("\nFill the buffer:");
    println!("cbuf_def.add(56);");
    println!("> {:?}", cbuf_def.add(56));
    println!("cbuf_def.add(67);");
    println!("> {:?}", cbuf_def.add(67));

    println!("\nPeek at the next element to be removed:");
    println!("cbuf_def.peek();  // Should be Ok(45)");
    println!("> {:?}", cbuf_def.peek());

    println!("\nEmpty the buffer:");
    println!("cbuf_def.remove();");
    println!("{:?}", cbuf_def.remove());
    println!("cbuf_def.remove();");
    println!("{:?}", cbuf_def.remove());
    println!("cbuf_def.remove();");
    println!("{:?}", cbuf_def.remove());

    println!("\nConfirm the buffer's size:");
    println!("cbuf_def.size();  // Should be 3");
    println!("> {}", cbuf_def.size());

    println!("\nPeek at the next element to be removed:");
    println!("cbuf_def.peek();  // Should be Ok(-1)");
    println!("> {:?}", cbuf_def.peek());

    println!("\n--\n");
}
