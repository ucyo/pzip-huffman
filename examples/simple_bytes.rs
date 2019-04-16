use pzip_huffman;

fn main() {

    println!("+++ Using Bytes +++");
    let words : Vec<u8> = vec![2,2,2,2,2,2,2,2,52,52,123,123,123,242];
    let codes = pzip_huffman::hufbites::get_huffman_codes(&words);
    println!("Codetable: {:#?}", codes);

    let word : Vec<u8> = vec![52,123,2,2];
    let coded = pzip_huffman::hufbites::encode(&word, &codes);
    println!("huff('mass'): {:?}", coded);

    let coded_bytes = coded.to_bytes();
    println!("huff('mass'): {:?} [bytes]", coded_bytes);

    print!("huff('mass'): ");
    for val in coded_bytes {
        print!("{:08b} ", val);
    }
    println!("");

}
