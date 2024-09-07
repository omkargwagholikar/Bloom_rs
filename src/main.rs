// use bloom_rs::bloom_v1::BloomFilterV1;
use bloom_rs::bloom_v2::BloomFilter;

fn main() {
    let filename = "src/bloom_data.txt";
    let input = "omkar";
    let mut bf = BloomFilter::new(filename.to_owned(), 4096, 2);
    println!("{}", bf.lookup(input));
    bf.insert(input);
    println!("{}", bf.lookup(input));
    // let bf = BloomFilterV1::new(filename);
    // bf.clear_data();
    // println!( "{}", bf.exists(input));
    // bf.add(input);
    // println!( "{}", bf.exists(input)); 


}
