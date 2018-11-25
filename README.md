# Queues

`queues` provides a number of efficient FIFO Queue data structures for
usage in your libraries. These are all implemented on top of rust's `Vector`
type.

A queue is a linear data structure that commonly defines three methods:

1. `add`: Also called `queue` or `push`, this adds elements to the queue.
2. `remove`: Also called `deque` or `pop`, this removes the _oldest_
   element from the queue.
3. `peek`: Show the next element in the queue scheduled for removal.

There are a number of variants of queues. In this crate, the available
variants are:

- `Queue<T>`: A simple FIFO queue with a growable size and no limit on its
  capacity.
- `Buffer<T>`: A FIFO queue with with a limited capacity. The buffer can
  have a growable size (up to the defined capacity), or it can be
  initialized at capacity, with empty slots being occupied by default
  values.
- `CircularBuffer<T>`: Similar to the buffer above, but allowing for
  overflow. Any additions to the circular buffer that would exceed its
  capacity causes its oldest element to be pushed out.

# Quick start

Quick guide to getting started with the project:

## Installation

### Library usage

To use the library in your project, simply ensure that the `queues` crate
has been added to your dependencies in your `Cargo.toml` file.

```yaml
[dependencies]
queues = "1.0.2"
```

In your files, import the crate and use it's members:

```rust
# #[macro_use]
extern crate queues;
use queues::*;
# fn main() { }
```

### Source code

To get the project up and running:

```bash
> cd ${WORKING_DIR}
> git clone <this_repo>
> cargo build
```

## Testing

Run the test suite using `cargo`:

```bash
> cd ${PROJECT_FOLDER}
> cargo test
```

## Examples

The project has a number of examples you can run to see how the library members work.

The example names are:

- `queue` Queue example
- `buf` Buffer example
- `cbuf` Circular buffer example
- `cbuf_def` Circular buffer with default values example

```bash
> cd ${PROJECT_FOLDER}
> cargo run --example ${EXAMPLE_NAME}
```

# Usage

Simple usage is described below:

```rust
#[macro_use]
extern crate queues;

use queues::*;

fn main() {
    // Create a simple Queue
    let mut q: Queue<isize> = queue![];

    // Add some elements to it
    q.add(1);
    q.add(-2);
    q.add(3);

    // Check the Queue's size
    q.size();  // 3

    // Remove an element
    q.remove();  // Ok(1)

    // Check the Queue's size
    q.size();  // 2

    // Peek at the next element scheduled for removal
    q.peek();  // Ok(-2)

    // Confirm that the Queue size hasn't changed
    q.size();  // 2

    // Remove the remaining elements
    q.remove();  // Ok(-2)
    q.remove();  // Ok(3)

    // Peek into an empty Queue
    q.peek();  // Raises an error

    // Attempt to remove an element from an empty Queue
    q.remove();  // Raises an error
}
```

The examples contain more information on `Buffer` and `CircularBuffer`
usage
