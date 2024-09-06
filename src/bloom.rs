use sha256::digest;
use std::{fs::{read_to_string, OpenOptions}, io::{Read, Seek, SeekFrom, Write}};

use crate::get_hash;

pub struct BloomFilter {
    filename: String,
}

impl BloomFilter {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }

    pub fn read_lines(&self) -> Vec<String> {
        let mut result = Vec::new();
    
        for line in read_to_string(&self.filename).unwrap().lines() {
            result.push(line.to_string())
        }
    
        result
    }

    pub fn clear_data(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.filename)
            .expect("File could not be opened");
    
        for _ in 0..(64 * 36) {
            file.write_all(&[0x00]).expect("Error in writing to file");
        }
    }

    pub fn is_set(&self, position: u64) -> bool {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.filename)
            .expect("File could not be opened");
        file.seek(SeekFrom::Start(position)).expect("Error in reading the file");
        let mut buffer = [0u8; 1];
        let _ = file.read_exact(&mut buffer);
        buffer[0] == 0x01
    }

    pub fn set_posn(&self, position: u64) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.filename)
            .expect("Error in opening file");
        file.seek(SeekFrom::Start(position)).expect("error in reading file");    
        file.write_all(&[0x01]).expect("Error in writing to file");
    }

    pub fn get_hash(input: &str) -> String {
        digest(input)
    }

    pub fn get_posn(&self, index: i32, ch: char) -> u64 {
        if ch.is_digit(10) {
            (index * 36 + (ch as i32 - '0' as i32)) as u64
        } else {
            (index * 36 + (ch as i32 - 'a' as i32) + 11) as u64
        }
    }

    pub fn exists(&self, input: &str) -> bool {
        let mut flag = true;
        let hash = get_hash(input);
        let chars: Vec<char> = hash.chars().collect();
        for i in 0..chars.len() {
            if !self.is_set(self.get_posn(i.try_into().unwrap(), chars[i])) {
                flag = false;
                break;
            }
        }
        flag
    }

    pub fn add(&self, input: &str) {
        let hash = get_hash(input);
        let chars: Vec<char> = hash.chars().collect();
        for i in 0..chars.len() {
            self.set_posn(self.get_posn(i.try_into().unwrap(), chars[i]));
        }
    }
}
