use std::fmt;

#[derive(PartialEq, Debug)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", &self.0)
    }
}

impl std::error::Error for ParseError {}

/// Spefic result type for M8 song parsing
pub type M8Result<T> = std::result::Result<T, ParseError>;

pub struct Writer {
    buffer: Vec<u8>
}

impl Writer {
    pub fn write(&mut self, v: u8) { self.buffer.push(v); }

    pub fn write_string(&mut self, _str: &str, _fill: usize) {
    }

    pub fn pos(&self) -> usize { self.buffer.len() }

    pub fn fill_till(&mut self, v: u8, until : usize) {
        let to_fill = until - self.buffer.len();
        for _i in 0 .. to_fill {
            self.buffer.push(v);
        }
    }
}

pub struct Reader {
    buffer: Vec<u8>,
    position: usize,
}

#[allow(dead_code)]
impl Reader {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    pub fn read(&mut self) -> u8 {
        let p: usize = self.position;
        let b = self.buffer[p];
        self.position += 1;
        b
    }

    pub fn read_bytes(&mut self, n: usize) -> &[u8] {
        let p: usize = self.position;
        let bs = &self.buffer[p..p + n];
        self.position += n;
        bs
    }

    pub fn read_bool(&mut self) -> bool {
        self.read() == 1
    }

    pub fn read_string(&mut self, n: usize) -> String {
        let b = self.read_bytes(n);
        let end = b.iter().position(|&x| x == 0 || x == 255).unwrap_or(0);
        std::str::from_utf8(&b[0..end])
            .expect("invalid utf-8 sequence in string")
            .to_string()
    }

    pub fn pos(&self) -> usize { self.position }

    pub fn set_pos(&mut self, n: usize) {
        self.position = n;
    }
}
