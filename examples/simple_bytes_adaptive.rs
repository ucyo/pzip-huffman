use pzip_huffman;

fn main() {

    println!("+++ Using Bytes (adaptive) +++");
    let words : Vec<u8> = vec![2,2,2,2,2,2,2,2,52,52,123,123,123,242,2,2,2,2,2,2,2,2,52];
    let encoded = pzip_huffman::hufbites::adaptive_encode_to_bytes(&words);
    println!("{:?}", encoded);

    // let decoded = pzip_huffman::hufbites::adaptive_decode_to_bytes(encoded, words.len());
    // println!("{:?}", decoded);

}
