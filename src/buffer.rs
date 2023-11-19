
extern crate alloc;
use alloc::vec::Vec;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::prelude::FileExt;

pub struct AudioBuffer {
    data: [u8; 4096],
    index: u64,
    path: String,
    length: u64,
    empty: bool,
}

impl AudioBuffer {
    pub fn new(file_path: String) -> Result<Self, io::Error> {
        let mut file = File::open(&file_path)?;
        let metadata = file.metadata()?;
        let default_data = [0u8; 4096];
        let default_index = 0;
        let default_length = metadata.len();
        let default_empty = metadata.len() == 0;

        Ok(Self {
            data: default_data,
            path: file_path,
            index: default_index,
            length: default_length,
            empty: default_empty,
        })
    }

    pub fn next(&mut self) {
        // Open the file
        let mut file = match File::open(&self.path) {
            Ok(file) => file,
            Err(_) => return,
        };

        if file.metadata().unwrap().len() - self.index >= 4096 {
            match file.read_exact_at(&mut self.data, self.index) {
                Ok(_) => {
                    println!("Byte read: {}", self.data[0]);
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        println!("Reached EOF!");
                    }
                    _ => {
                        println!("Error occurred updating the buffer after byte # {}", self.index);
                    }
                },
            }
        } else {
            self.data = [0; 4096];
            match file.read_at(&mut self.data, self.index) {
                Ok(_) => {
                    println!("Byte read: {}", self.data[0]);
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        println!("Reached EOF!");
                    }
                    _ => {
                        println!("Error occurred updating the buffer after byte # {}", self.index);
                    }
                },
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.empty
    }
    pub fn get_data(&self) ->  &[u8; 4096]{
        &self.data
    }
}

