use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use sha2::Digest;
use std::io::Error;
use std::sync::RwLock;

pub struct BloomFilter {
    filename: String, // For writing the bit_vector to disk for persistence
    bit_vector: RwLock<Vec<u8>>,
    size: usize,
    hash_count:i32,
}

impl BloomFilter {
    pub fn new(filename: String, size: usize, hash_count: i32) -> Self {
        if size & (size-1) != 0 {
            println!("The size must be a power of 2");
        }

        Self {
            filename,
            bit_vector: RwLock::new(vec![0; size as usize]),
            size,
            hash_count
        }
    }

    fn set_byte(&mut self, position: usize) {        
        if position > self.size as usize {
            println!("Position is greater than size");
            return;
        }
        
        // Accquire write lock
        let mut bit_vector = self.bit_vector.write().unwrap();
        bit_vector[position] = 1;
        // Release write lock
    }

    fn get_byte(&self, position: usize) -> u8 {
        if position > self.size as usize {
            println!("Position is greater than size");
            return 0 as u8;
        }
        // Accquire read lock
        let bit_vector = self.bit_vector.read().unwrap();
        bit_vector[position]
        // Release read lock
    }


    fn get_hashes(&self, input: &str) -> Vec<[u8; 32]> {
        let mut res = vec![[0u8;32]; self.hash_count as usize];
        let mut hash: [u8;32] = sha2::Sha256::digest(input).into();
        for i in 0..self.hash_count as usize{
            res[i] = hash;
            hash[0] = hash[0] & 0xf;
            hash = sha2::Sha256::digest(hash).into();
        }
        res
    }

    pub fn lookup(&self, input: &str) -> bool {
        let hashes:Vec<[u8; 32]> = self.get_hashes(input);
        for i in 0..self.hash_count as usize {
            let hash = hashes[i];
            // Convert the first 8 bytes of the hash to a u64 integer to make mapping easier
            let hash_bytes: [u8; 8] = hash[..8].try_into().expect("Failed to convert hash slice to array");
            let position = usize::from_be_bytes(hash_bytes);
            if self.get_byte(position & (self.size-1) as usize) == 0 {
                return false
            }
        }
        true
    }

    pub fn insert(&mut self, input: &str) {
        let hashes: Vec<[u8; 32]> = self.get_hashes(input);
        for i in 0..self.hash_count as usize {
            let hash = hashes[i];
            // Convert the first 8 bytes of the hash to a u64 integer to make mapping easier
            let hash_bytes: [u8; 8] = hash[..8].try_into().expect("Failed to convert hash slice to array");
            let position = usize::from_be_bytes(hash_bytes);
            self.set_byte(position & (self.size -1) as usize);
        }
    }

    pub fn write(& self) {
        let mut f = File::create(&self.filename).expect("Unable to create file");
        let bit_vector = self.bit_vector.read().unwrap();
        writeln!(f, "{}", self.size).expect("Trouble writing to file");
        writeln!(f, "{}", self.hash_count).expect("Trouble writing to file");

        for i in bit_vector.iter() {
            f.write_all(&[*i]).expect("Unable to write data");                                                                                                                            
        } 
    }

    pub fn load(&mut self, filepath: &str)  {
        let f = File::open(filepath).expect("Error in opening the file for loading");
        let br = BufReader::new(f);
        // create an empty vector, type of the stored elements will be inferred
        let mut v = Vec::new();
        // br.lines() creates an iterator over lines in the reader
        let mut count = 0;

        for line in br.lines() {
            count += 1;
            if count == 3 {
                break;
            }
            let line = line.unwrap();
            let n: usize = line   
                .trim() 
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
                .expect("Error in parsing the file");
            v.push(n);
        }

        self.filename = filepath.to_owned();
        self.size = v[0];
        self.hash_count = v[1] as i32;
        // let mut bit_vector = self.bit_vector.write().expect("Could not get lock to load file");
        // f.read_to_end(&mut bit_vector).expect("Could not get lock to load file");
    }

}