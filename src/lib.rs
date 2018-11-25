//! # Queues
//!
//! `queues` provides a number of efficient FIFO Queue data structures for
//! usage in your libraries. These are all implemented on top of rust's `Vector`
//! type.
//!
//! A queue is a linear data structure that commonly defines three methods:
//!
//! 1. `add`: Also called `queue` or `push`, this adds elements to the queue.
//! 2. `remove`: Also called `deque` or `pop`, this removes the _oldest_
//!     element from the queue.
//! 3. `peek`: Show the next element in the queue scheduled for removal.
//!
//! There are a number of variants of queues. In this crate, the available
//! variants are:
//!
//! - `Queue<T>`: A simple FIFO queue with a growable size and no limit on its
//!     capacity.
//! - `Buffer<T>`: A FIFO queue with with a limited capacity. The buffer can
//!     have a growable size (up to the defined capacity), or it can be
//!     initialized at capacity, with empty slots being occupied by default
//!     values.
//! - `CircularBuffer<T>`: Similar to the buffer above, but allowing for
//!     overflow. Any additions to the circular buffer that would exceed its
//!     capacity causes its oldest element to be pushed out.
//!
//! # Quick start
//!
//! Quick guide to getting started with the project:
//!
//! ## Installation
//!
//! ### Library usage
//!
//! To use the library in your project, simply ensure that the `queues` crate
//! has been added to your dependencies in your `Cargo.toml` file.
//!
//! ```yaml
//! [dependencies]
//! queues = "1.0.2"
//! ```
//!
//! In your files, import the crate and use it's members:
//!
//! ```rust
//! # #[macro_use]
//! extern crate queues;
//! use queues::*;
//! # fn main() { }
//! ```
//!
//! ### Source code
//!
//! To get the project up and running:
//!
//! ```bash
//! > cd ${WORKING_DIR}
//! > git clone <this_repo>
//! > cargo build
//! ```
//!
//! ## Testing
//!
//! Run the test suite using `cargo`:
//!
//! ```bash
//! > cd ${PROJECT_FOLDER}
//! > cargo test
//! ```
//!
//! ## Examples
//!
//! The project has a number of examples you can run to see how the library members work.
//!
//! The example names are:
//!
//! - `queue` Queue example
//! - `buf` Buffer example
//! - `cbuf` Circular buffer example
//! - `cbuf_def` Circular buffer with default values example
//!
//! ```bash
//! > cd ${PROJECT_FOLDER}
//! > cargo run --example ${EXAMPLE_NAME}
//! ```
//!
//! # Usage
//!
//! Simple usage is described below:
//!
//! ```rust
//! #[macro_use]
//! extern crate queues;
//!
//! use queues::*;
//!
//! fn main() {
//!     // Create a simple Queue
//!     let mut q: Queue<isize> = queue![];
//!
//!     // Add some elements to it
//!     q.add(1);
//!     q.add(-2);
//!     q.add(3);
//!
//!     // Check the Queue's size
//!     q.size();  // 3
//!
//!     // Remove an element
//!     q.remove();  // Ok(1)
//!
//!     // Check the Queue's size
//!     q.size();  // 2
//!
//!     // Peek at the next element scheduled for removal
//!     q.peek();  // Ok(-2)
//!
//!     // Confirm that the Queue size hasn't changed
//!     q.size();  // 2
//!
//!     // Remove the remaining elements
//!     q.remove();  // Ok(-2)
//!     q.remove();  // Ok(3)
//!
//!     // Peek into an empty Queue
//!     q.peek();  // Raises an error
//!
//!     // Attempt to remove an element from an empty Queue
//!     q.remove();  // Raises an error
//! }
//! ```
//!
//! The examples contain more information on `Buffer` and `CircularBuffer`
//! usage

#![warn(missing_docs)]

/// Defines methods that would be expected on a queue data structure
pub trait IsQueue<T: Clone> {
    /// Adds a new value to a queue
    ///
    /// # Parameters
    /// - `val`: Value to add to the queue
    ///
    /// # Returns
    /// - `Ok(_)`: If the element add was successful.
    ///     - `Some(T)`: If adding an element resulted in the removal of an
    ///         existing one (in the case of a circular buffer, for instance)
    ///     - `None`: Adding an element did not return any value
    /// - `Error`: If the element add was unsuccessful
    ///
    /// # Errors
    /// Attempting to add an element to a full queue that does not allow for
    /// overflow will return an error.
    fn add(&mut self, val: T) -> Result<Option<T>, &str>;

    /// Removes an element from the queue and returns it
    ///
    /// For queues with default values, removing an element will add a new
    /// default value into the queue.
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest element in the queue
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to remove an element from
    /// an empty queue
    fn remove(&mut self) -> Result<T, &str>;

    /// Peek at the head of the queue
    ///
    /// # Returns
    /// - `Ok(T)`: The next element scheduled for removal from the queue
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to peek into an empty queue
    fn peek(&self) -> Result<T, &str>;

    /// Gets the size of the queue
    ///
    /// # Returns
    /// The number of elements in the queue. Note, this _includes_ default
    /// values when specified, which means that the `size` of a queue with
    /// default values should always be equal to its `capacity`
    fn size(&self) -> usize;
}

/// A simple FIFO queue with a growable size and no limit on its capacity.
///
/// # Type parameters
/// - `T`: Any type that implements the `Clone` trait.
///
/// # Examples
///
/// This example uses the `queue!` macro to add elements to the queue. Note
/// that the first element in the list of elements passed to the macro is
/// considered the 'oldest'.
///
/// ```
/// # #[macro_use] extern crate queues;
/// # use queues::*;
/// # fn main() {
/// let mut q = queue![3isize, 4, 5];
///
/// // Add an element
/// assert_eq!(q.add(6), Ok(None));
///
/// // Remove some elements
/// assert_eq!(q.remove(), Ok(3));
/// assert_eq!(q.remove(), Ok(4));
///
/// // Peek at the next element scheduled for removal
/// assert_eq!(q.peek(), Ok(5));
///
/// // Check the queue size
/// assert_eq!(q.size(), 2);
/// # }
/// ```
#[derive(Debug)]
pub struct Queue<T: Clone> {
    queue: Vec<T>,
}

impl<T: Clone> Queue<T> {
    /// Create a new queue
    ///
    /// # Returns
    /// A new, empty `Queue<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let q: Queue<isize> = Queue::new();
    /// assert_eq!(q.size(), 0);
    /// ```
    pub fn new() -> Queue<T> {
        Queue { queue: vec![] }
    }
}

impl<T: Clone> Default for Queue<T> {
    /// Default queue initializer
    ///
    /// # Returns
    /// A new, empty `Queue<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let q: Queue<isize> = Queue::default();
    /// assert_eq!(q.size(), 0);
    /// ```
    fn default() -> Queue<T> {
        Queue { queue: vec![] }
    }
}

impl<T: Clone> IsQueue<T> for Queue<T> {
    /// Adds an element to a queue
    ///
    /// # Parameters
    /// - `val`: Value to add to the queue
    ///
    /// # Returns
    /// `Ok(None)` as the element addition should always be successful
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut q: Queue<isize> = Queue::new();
    /// assert_eq!(q.add(42), Ok(None));
    /// assert_eq!(q.size(), 1);
    /// ```
    fn add(&mut self, val: T) -> Result<Option<T>, &str> {
        self.queue.push(val);
        Ok(None)
    }

    /// Removes an element from the queue and returns it
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest element in the queue
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to remove an element from
    /// an empty queue
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut q: Queue<isize> = Queue::new();
    /// q.add(42);
    /// assert_eq!(q.remove(), Ok(42));
    /// assert_eq!(q.size(), 0);
    /// ```
    fn remove(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            Ok(self.queue.remove(0usize))
        } else {
            Err("The queue is empty")
        }
    }

    /// Peek at the head of the queue
    ///
    /// # Returns
    /// - `Ok(T)`: The next element scheduled for removal from the queue
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to peek into an empty queue
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut q: Queue<isize> = Queue::new();
    /// q.add(42);
    /// assert_eq!(q.peek(), Ok(42));
    /// ```
    fn peek(&self) -> Result<T, &str> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err("The Queue is empty"),
        }
    }

    /// Gets the size of the queue
    ///
    /// # Returns
    /// The number of elements in the queue
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut q: Queue<isize> = Queue::new();
    /// assert_eq!(q.size(), 0);
    /// let _ = q.add(42);
    /// assert_eq!(q.size(), 1);
    /// ```
    fn size(&self) -> usize {
        self.queue.len()
    }
}

/// Creates a new `Queue<T>`
///
/// Delegates to the default queue initializer. Note that the elements are
/// added to the queue from left to right, therefore the first element in the
/// list of parameters passed to the macro is considered the 'oldest' element
/// in the queue.
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate queues;
/// # use queues::*;
///
/// # fn main() {
/// let q = queue![3isize, 4, 5];
/// assert_eq!(q.peek(), Ok(3));
///
/// let q_empty: Queue<isize> = queue![];
/// assert_eq!(q_empty.size(), 0);
/// # }
/// ```
#[macro_export]
macro_rules! queue {
    () => { Queue::new() };
    ($($x:expr),+) => {
        {
            let mut temp_q = Queue::default();
            $(
                let _ = temp_q.add($x);
            )*
            temp_q
        }
    };
}

/// A FIFO buffer with a growable size and a capacity limit
///
/// # Type parameters
/// - `T`: Any type that implements the `Clone` trait.
///
/// # Examples
///
/// ```
/// # use queues::*;
/// let mut buf = Buffer::new(3);
///
/// // Add some elements
/// assert_eq!(buf.add(6), Ok(None));
/// assert_eq!(buf.add(7), Ok(None));
///
/// // Remove an element
/// assert_eq!(buf.remove(), Ok(6));
///
/// // Peek at the next element scheduled for removal
/// assert_eq!(buf.peek(), Ok(7));
///
/// // Check the queue size
/// assert_eq!(buf.size(), 1);
/// ```
#[derive(Debug)]
pub struct Buffer<T: Clone> {
    queue: Vec<T>,
    capacity: usize,
}

impl<T: Clone> Buffer<T> {
    /// Create a new buffer
    ///
    /// # Returns
    /// A new, empty `Buffer<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let buf: Buffer<isize> = Buffer::new(3);
    /// assert_eq!(buf.size(), 0);
    /// ```
    pub fn new(capacity: usize) -> Buffer<T> {
        Buffer {
            queue: vec![],
            capacity,
        }
    }

    /// Gets the capacity of the buffer
    ///
    /// # Returns
    /// The number of allowed elements in the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut buf: Buffer<isize> = Buffer::new(5);
    /// assert_eq!(buf.capacity(), 5);
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Clone> IsQueue<T> for Buffer<T> {
    /// Adds an element to a buffer
    ///
    /// # Parameters
    /// - `val`: Value to add to the buffer
    ///
    /// # Returns
    /// - `Ok(None)`: Element addition was successful
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to add an element to a full
    /// buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use queues::*;
    ///
    /// let mut buf: Buffer<isize> = Buffer::new(3);
    /// assert_eq!(buf.add(42), Ok(None));
    /// ```
    fn add(&mut self, val: T) -> Result<Option<T>, &str> {
        if self.queue.len() < self.capacity {
            self.queue.push(val);
            Ok(None)
        } else {
            Err("The buffer is full")
        }
    }

    /// Removes an element from the buffer and returns it.
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest element in the buffer
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to remove an element from
    /// an empty buffer
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut buf: Buffer<isize> = Buffer::new(3);
    /// buf.add(42);
    /// assert_eq!(buf.remove(), Ok(42));
    /// assert_eq!(buf.size(), 0);
    /// ```
    fn remove(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            Ok(self.queue.remove(0usize))
        } else {
            Err("The buffer is empty")
        }
    }

    /// Peek at the head of the buffer
    ///
    /// # Returns
    /// - `Ok(T)`: The next element scheduled for removal from the buffer
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to peek into an empty buffer
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut buf: Buffer<isize> = Buffer::new(3);
    /// buf.add(42);
    /// assert_eq!(buf.peek(), Ok(42));
    /// ```
    fn peek(&self) -> Result<T, &str> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err("The buffer is empty"),
        }
    }

    /// Gets the size of the buffer
    ///
    /// # Returns
    /// The number of elements in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut buf: Buffer<isize> = Buffer::new(3);
    /// assert_eq!(buf.size(), 0);
    /// buf.add(42);
    /// assert_eq!(buf.size(), 1);
    /// ```
    fn size(&self) -> usize {
        self.queue.len()
    }
}

/// Represents a FIFO `CircularBuffer<T>` data structure.
///
/// This structure is a limited capacity queue, with optional provisions
/// for default values. Under normal circumstances, the `size` of the
/// queue grows until it reaches its `capacity`, at which point any
/// further additions push out its oldest member.
///
/// If default values are specified, then the `size` of the queue
/// is always equal to its `capacity`, with empty slots occupied by the
/// specified default value.
///
/// # Type parameters
/// - `T`: Any type that implements the `Clone` trait.
///
/// # Examples
///
/// ```
/// # use queues::*;
/// # fn main() {
/// let mut cbuf = CircularBuffer::<isize>::new(3);
/// let mut cbuf_def = CircularBuffer::with_default(3, 0isize);
///
/// // Check sizes
/// assert_eq!(cbuf.size(), 0);
/// assert_eq!(cbuf_def.size(), 3);
///
/// // Add elements
/// cbuf.add(6);
/// cbuf_def.add(7);
///
/// // Peek at the next element scheduled for removal
/// assert_eq!(cbuf.peek().unwrap(), 6);
/// assert_eq!(cbuf_def.peek().unwrap(), 0);
/// # }
/// ```
#[derive(Debug)]
pub struct CircularBuffer<T: Clone> {
    queue: Vec<T>,
    capacity: usize,
    default_value: Option<T>,
}

impl<T: Clone> CircularBuffer<T> {
    /// Default `CircularBuffer<T>` initializer
    ///
    /// # Returns
    /// A new, empty `CircularBuffer<T>`
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let cbuf: CircularBuffer<isize> = CircularBuffer::new(3);
    /// assert_eq!(cbuf.size(), 0);
    /// assert_eq!(cbuf.capacity(), 3);
    /// ```
    pub fn new(capacity: usize) -> CircularBuffer<T> {
        CircularBuffer {
            queue: vec![],
            capacity,
            default_value: None,
        }
    }

    /// Create a `CircularBuffer<T>` with default values
    ///
    /// # Returns
    /// A new `CircularBuffer<T>` filled with default values
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let cbuf_def = CircularBuffer::with_default(3, -1isize);
    /// assert_eq!(cbuf_def.size(), 3);
    /// assert_eq!(cbuf_def.capacity(), 3);
    /// assert_eq!(cbuf_def.peek(), Ok(-1));
    /// ```
    pub fn with_default(capacity: usize, default_value: T) -> CircularBuffer<T> {
        let queue = vec![default_value.clone(); capacity];

        CircularBuffer {
            queue,
            capacity,
            default_value: Some(default_value),
        }
    }

    /// Gets the capacity of the `CircularBuffer<T>`
    ///
    /// # Returns
    /// The number of allowed elements in the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::CircularBuffer;
    /// let mut cbuf: CircularBuffer<isize> = CircularBuffer::new(3);
    /// assert_eq!(cbuf.capacity(), 3);
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T: Clone> IsQueue<T> for CircularBuffer<T> {
    /// Adds an element to a circular buffer
    ///
    /// # Parameters
    /// - `val`: Value to add to the buffer
    ///
    /// # Returns
    /// - `Ok(Some(T))`: The oldest value in the buffer, in case the addition
    ///     causes an overflow.
    /// - `Ok(None)`: Nothing, if the buffer has room for the added element
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut cbuf: CircularBuffer<isize> = CircularBuffer::new(3);
    /// let mut cbuf_def = CircularBuffer::with_default(3, 5isize);
    /// assert_eq!(cbuf.add(42), Ok(None));
    /// assert_eq!(cbuf_def.add(42), Ok(Some(5)));
    /// ```
    fn add(&mut self, val: T) -> Result<Option<T>, &str> {
        if self.queue.len() < self.capacity {
            self.queue.push(val);
            Ok(None)
        } else {
            self.queue.push(val);
            Ok(Some(self.queue.remove(0usize)))
        }
    }

    /// Removes an element from the circular buffer and returns it.
    ///
    /// For circular buffers with default values, removing an element will add
    /// a new default value into the buffer.
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest element in the buffer
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to remove an element from
    /// an empty buffer
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut cbuf: CircularBuffer<isize> = CircularBuffer::new(3);
    /// cbuf.add(42);
    /// assert_eq!(cbuf.remove(), Ok(42));
    /// assert_eq!(cbuf.size(), 0);
    ///
    /// let mut cbuf_def = CircularBuffer::with_default(3, 4isize);
    /// cbuf_def.add(42);
    /// assert_eq!(cbuf_def.remove(), Ok(4));
    /// ```
    fn remove(&mut self) -> Result<T, &str> {
        if !self.queue.is_empty() {
            if let Some(val) = self.default_value.clone() {
                self.queue.push(val);
            };
            Ok(self.queue.remove(0usize))
        } else {
            Err("The Buffer is empty")
        }
    }

    /// Peek at the head of the circular buffer
    ///
    /// # Returns
    /// - `Ok(T)`: The next element scheduled for removal from the buffer
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if an attempt is made to peek into an empty buffer
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut cbuf: CircularBuffer<isize> = CircularBuffer::new(3);
    /// cbuf.add(42);
    /// assert_eq!(cbuf.peek(), Ok(42));
    /// ```
    fn peek(&self) -> Result<T, &str> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => Err("The Queue is empty"),
        }
    }

    /// Gets the size of the circular buffer
    ///
    /// # Returns
    /// The number of elements in the buffer. Note, this _includes_ default
    /// values, which means that the `size` of a buffer with default values
    /// should always be equal to its `capacity`
    ///
    /// # Examples
    ///
    /// ```
    /// # use queues::*;
    /// let mut cbuf: CircularBuffer<isize> = CircularBuffer::new(3);
    /// assert_eq!(cbuf.size(), 0);
    /// cbuf.add(42);
    /// assert_eq!(cbuf.size(), 1);
    /// ```
    fn size(&self) -> usize {
        self.queue.len()
    }
}
