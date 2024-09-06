mod bloom;
use bloom::BloomFilter;

fn main() {
    let filename = "src/bloom_data.txt";
    let bf = BloomFilter::new(filename);
    bf.clear_data();
    let input = "omkar";
    println!( "{}", bf.exists(input));
    bf.add(input);
    println!( "{}", bf.exists(input)); 
}
