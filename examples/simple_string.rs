use pzip_huffman;

fn main() {

    println!("+++ Using Strings +++");
    let words = "mmmaaaassssssssssssii";
    let codes = pzip_huffman::get_huffman_codes(&words);
    println!("Codetable: {:#?}", codes);

    let word = "mass";
    let coded = pzip_huffman::encode(&word, &codes);
    println!("huff('mass'): {:?}", coded);

    let coded_bytes = coded.to_bytes();
    println!("huff('mass'): {:?} [bytes]", coded_bytes);

    print!("huff('mass'): ");
    for val in coded_bytes {
        print!("{:08b} ", val);
    }
    println!("");

}
