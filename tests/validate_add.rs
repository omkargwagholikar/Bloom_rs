use bloom_rs::bloom_v1::BloomFilterV1;
use bloom_rs::bloom_v2::BloomFilter;
use std::fs;

#[test]
fn add_new() {
    println!("Testing adding new value");
    let filename = "tests/bloom_test.txt";
    let input = "test";
    let bf = BloomFilterV1::new(filename);
    bf.clear_data();
    let before = bf.exists(input);
    bf.add(input);
    let after = bf.exists(input);
    assert_eq!(before, false, "Input should not exist in the filter before adding");
    assert_eq!(after, true, "Input should exist in the filter after adding");
    fs::remove_file(filename).expect("test file deleted");
}


#[test]
fn check_bloom() {
    let filename = "tests/bloom_data.txt";
    let input = "omkar";
    let mut bf = BloomFilter::new(filename.to_owned(), 4096, 12);
    let before = bf.lookup(input);
    bf.insert(input);
    let after =  bf.lookup(input);
    
    assert_eq!(before, false, "Input should not exist in the filter before adding");
    assert_eq!(after, true, "Input should exist in the filter after adding");
    
    // bf.write();
}