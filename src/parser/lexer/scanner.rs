pub struct Scanner<'a> {
    /// Buffer of bytes to iterate over.
    buffer: &'a [u8],

    /// Current index into the buffer,
    /// can be incremented with consume.
    index: usize,
}

impl Scanner<'_> {
    /// Instantiate a new Scanner instance.
    pub fn new(input: &[u8]) -> Scanner {
        return Scanner {
            buffer: input,
            index: 0,
        };
    }

    /// Read one item from the list of
    /// characters and increment the index.
    pub fn consume(&mut self) -> u8 {
        self.index += 1;
        return self.buffer[self.index - 1];
    }

    /// Read one item from the list
    /// of characters; while not incrementing
    /// the index, and specifying how far ahead to peek.
    pub fn look_ahead(&mut self, index: usize) -> u8 {
        return self.buffer[self.index + index];
    }

    /// Read one item from the list of
    /// characters and do not increment the index.
    pub fn peek(&mut self) -> u8 {
        return self.look_ahead(0);
    }

    /// Check if current character is an EOF.
    pub fn is_eof(&mut self) -> bool {
        return self.peek() == 0 || self.index + 1 > self.buffer.len();
    }
}
