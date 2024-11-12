#[derive(Debug)]
struct RingBuffer<T> {
    storage: Vec<Option<T>>,
    head: usize,
    tail: usize,
    full: bool
}

impl<T: Clone> RingBuffer<T> {
    fn new(cap: usize) -> Self {
        RingBuffer {
            storage: vec![None; cap],
            head: 0,
            tail: 0,
            full: false
        }
    }

    fn write(&mut self, item:T) {
        if self.full {
            // wrap around! and override!!
            self.head = (self.head + 1) % self.storage.len();
            println!("Overiding...");
        }

        self.storage[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.storage.len();

        self.full = self.tail == self.head;
    }

    fn read(&mut self) -> Option<T> {
        println!("READING!!");

        println!("head {}",self.head);
        println!("tail {}",self.tail);
        if !self.full && self.head == self.tail {
            // read all, nothing more to read btw
            return None;
        }

        if let Some(item) = &self.storage[self.head].take() {
            self.head = (self.head + 1) % self.storage.len();
            self.full = false;

            Some(item.clone())
        } else {
            None
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buffer = RingBuffer::<i32>::new(5);
        assert_eq!(buffer.storage.len(), 5);
        assert_eq!(buffer.head, 0);
        assert_eq!(buffer.tail, 0);
    }

    #[test]
    fn test_write_and_read() {
        let mut buffer = RingBuffer::<i32>::new(5);

        buffer.write(1);
        buffer.write(2);
        buffer.write(3);

        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
        assert_eq!(buffer.read(), None);
    }

    #[test]
    fn test_wrap_around_and_override() {
        let mut buffer = RingBuffer::<i32>::new(3);

        buffer.write(1);
        buffer.write(2);
        buffer.write(3);
        assert_eq!(buffer.read(), Some(1));

        buffer.write(4);
        assert_eq!(buffer.read(), Some(2));
        buffer.write(5);
        assert_eq!(buffer.read(), Some(3));
        assert_eq!(buffer.read(), Some(4));
        assert_eq!(buffer.read(), Some(5));
        assert_eq!(buffer.read(), None);
    }
}

fn main() {}
