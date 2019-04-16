use pzip_huffman;

fn main() {

    println!("+++ Using Bytes +++");
    let words : Vec<u8> = vec![2,2,2,2,2,2,2,2,52,52,123,123,123,242];
    let encoded = pzip_huffman::hufbites::encode_itself_to_bytes(&words);
    println!("{:?}", encoded);

    let table = pzip_huffman::hufbites::get_huffman_codes(&words);
    let decoded = pzip_huffman::hufbites::decode_from_bytes(encoded, &table, words.len());

    println!("{:?}", decoded);

}
