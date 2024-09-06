mod bloom;
use bloom::BloomFilter;

fn main() {
    let filename = "/home/omkar/Desktop/bloom_rs/src/bloom.txt";
    let bf = BloomFilter::new(filename);
    let input = "omkar1";
    println!( "{}", bf.exists(input));
    bf.add(input);
    println!( "{}", bf.exists(input)); 
}
