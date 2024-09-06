mod bloom;
use bloom::BloomFilter;

use sha256::digest;
use std::{fs::{read_to_string, OpenOptions}, io::{Read, Seek, SeekFrom, Write}};


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn clear_data(filename: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)
        .expect("File could not be opened");

    for _ in 0..(64 * 36) {
        file.write_all(&[0x00]).expect("Error in writing to file");
    }
}

fn is_set(filename: &str, position: u64) -> bool {
    let mut file = OpenOptions::new()
        .read(true)
        .open(filename)
        .expect("File could not be opened");
    file.seek(SeekFrom::Start(position)).expect("Error in reading the file");
    let mut buffer = [0u8; 1];
    let _ = file.read_exact(&mut buffer);
    buffer[0] == 0x01
}

fn set_posn(filename: &str, position: u64) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(filename)
        .expect("Error in opening file");
    file.seek(SeekFrom::Start(position)).expect("error in reading file");

    file.write_all(&[0x01]).expect("Error in writing to file");
}

fn get_hash(input: &str) -> String{
    digest(input)
}

fn get_posn(index: i32, ch: char) -> u64 {
    if ch.is_digit(10) {
        (index * 36 + (ch as i32 - '0' as i32)) as u64
    } else {
        (index * 36 + (ch as i32 - 'a' as i32) + 11) as u64
    }
}

fn exists(hash: &str, filename: &str) -> bool{
    let mut flag = true;
    let chars: Vec<char> = hash.chars().collect();
    for i in 0..chars.len() {
        if(!is_set(filename, get_posn(i.try_into().unwrap(), chars[i]))){
            flag = false;
            break;
        }
    }
    flag
}

fn add(hash: &str, filename: &str) {
    let chars: Vec<char> = hash.chars().collect();
    for i in 0..chars.len() {
        set_posn(filename, get_posn(i.try_into().unwrap(), chars[i]));
    }
}
fn main() {
    println!("Hello, world!");
    let filename = "/home/omkar/Desktop/bloom_rs/src/bloom.txt";
    let bf = BloomFilter::new(&filename);
    let input = "somename1";
    let hash = digest(input);
    
    println!("{}", exists(&hash, filename));
    add(&hash, filename);
    println!("{}", exists(&hash, filename));
}
