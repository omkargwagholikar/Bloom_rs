use bloom_rs::bloom::BloomFilter;

#[test]
fn add_new() {
    println!("Testing adding new value");
    let filename = "/home/omkar/Desktop/bloom_rs/src/bloom_test.txt";
    let input = "test";
    let bf = BloomFilter::new(filename);
    bf.clear_data();
    let before = bf.exists(input);
    bf.add(input);
    let after  =bf.exists(input);
    assert_eq!(before, false, "Input should not exist in the filter before adding");
    assert_eq!(after, true, "Input should exist in the filter after adding");
}

