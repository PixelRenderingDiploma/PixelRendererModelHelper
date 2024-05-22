use std::io::{self, Read, BufReader};

pub trait ReadToStringExact {
    fn read_to_string_exact(&mut self, size: usize) -> io::Result<String>;
    fn read_to_string_lossy_exact(&mut self, size: usize) -> io::Result<String>;
}

impl<T: Read> ReadToStringExact for BufReader<T> {
    fn read_to_string_exact(&mut self, size: usize) -> io::Result<String> {
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;
        String::from_utf8(buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn read_to_string_lossy_exact(&mut self, size: usize) -> io::Result<String> {
        let mut buffer = vec![0; size];
        self.read_exact(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}