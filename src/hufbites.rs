/// TODO: bit-vec is deprecated
/// TODO: Huffman codes not consistent, if the number of occurences is the same
/// TODO: Rename merged_huffman to numerical_huffman

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use log::{debug, warn, info};
use std::collections::BTreeMap;
use bit_vec::BitVec;

pub fn adaptive_encode_to_bytes(data: &Vec<u8>) -> Vec<u8> {
    let mut bv = BitVec::new();
    for i in 0..data.len() {
        let huffman_codes = get_huffman_codes(&data[..i+1].to_vec());
        let b = huffman_codes.get(&data[i]).unwrap();
        for bit in b {
            bv.push(bit)
        }
    }
    bv.to_bytes()
}

pub fn encode_itself_by_merged_huffman_to_bytes(data: &Vec<u8>) -> (Vec<u8>, HashMap<u8,u8>) {
    let (huffman_codes, mappings) = get_merged_huffman_codes(data);
    let bitvec = encode(data, &huffman_codes);
    (bitvec.to_bytes(), mappings)
}


pub fn encode_itself_to_bytes(data: &Vec<u8>) -> Vec<u8> {
    let huffman_codes = get_huffman_codes(data);
    let bitvec = encode(data, &huffman_codes);
    bitvec.to_bytes()
}


pub fn decode_from_bytes(data: Vec<u8>, huffman_codes: &HashMap<u8, BitVec>, size: usize) -> Vec<u8> {
    let bitvec = BitVec::from_bytes(&data[..]);
    let result = decode(bitvec, huffman_codes);

    result[..size].to_vec()
}


pub fn encode(data: &Vec<u8>, huffman_codes: &HashMap<u8, BitVec>) -> BitVec {

    let mut nbits = 0;
    for c in data.iter() {
        nbits += huffman_codes.get(c).unwrap().len();
    }

    let mut res = BitVec::with_capacity(nbits);

    for c in data.iter() {
        let bv = huffman_codes.get(c).unwrap();
        for bit in bv.iter() {
            res.push(bit);
        }
    }

    res
}

pub fn decode(bits: BitVec, huffman_codes: &HashMap<u8, BitVec>) -> Vec<u8> {

    let hci = invert_huffman_codes(huffman_codes);

    let mut res : Vec<u8> = Vec::new();
    let mut bv = BitVec::new();

    for bit in bits.iter() {
        bv.push(bit);
        if hci.contains_key(&bv) {
            res.push(hci.get(&bv).unwrap().clone());
            bv = BitVec::new();
        }
    }

    res
}

fn get_bytes_counts(data: &Vec<u8>) -> HashMap<u8, i32> {

    let mut char_counts = HashMap::new();

    for c in data.iter() {
        let count = char_counts.entry(*c).or_insert(0);
        *count += 1;
    }

    char_counts
}

fn heapify(map: HashMap<u8, i32>) -> BinaryHeap<Box<Tree>> {

    let mut heap = BinaryHeap::new();

    for (letter, count) in map.iter() {
        let t = Tree::new(*letter, *count);
        heap.push(t);
    }

    heap
}

fn create_huffman_tree(mut heap: BinaryHeap<Box<Tree>>) -> Box<Tree> {
    while heap.len() > 1 {
        let t1 = heap.pop().unwrap();
        let t2 = heap.pop().unwrap();

        let t_new = Tree::combine(t1, t2);
        heap.push(t_new);
    }

    heap.pop().unwrap()
}

pub fn get_huffman_codes(data: &Vec<u8>) -> HashMap<u8, BitVec> {
    let mut bv = BitVec::new();

    let char_counts = get_bytes_counts(data);
    debug!("Huffman Counts: {:?}", char_counts);
    if char_counts.len() <= 1 {
        bv.push(false);
    }

    let heap = heapify(char_counts.clone());
    let ht = create_huffman_tree(heap);

    let result = huffman_codes_from_tree(&Some(ht), bv, HashMap::new());
    debug!("Huffman Codes: {:?}", result);
    result
}


pub fn get_merged_huffman_codes(data: &Vec<u8>)  -> (HashMap<u8, BitVec>, HashMap<u8,u8>) {

    // calculate once
    let mut bv = BitVec::new();

    let mut char_counts = get_bytes_counts(data);
    if char_counts.len() <= 1 {
        bv.push(false);
    }

    let heap = heapify(char_counts.clone());
    let ht = create_huffman_tree(heap);
    let result = huffman_codes_from_tree(&Some(ht), bv, HashMap::new());
    debug!("Codebook 1: {:?}", result);
    let mappings = check_saving(&result, &char_counts);

    // remap values and update char_counts
    for (old,new) in mappings.iter() {
        let old_count = char_counts.get(old).unwrap();
        char_counts.insert(*new, char_counts.get(new).unwrap() + old_count);
        char_counts.remove(old).unwrap();
    }

    // redo
    let mut bv = BitVec::new();
    if char_counts.len() <= 1 {
        bv.push(false);
    }
    let heap = heapify(char_counts.clone());
    let ht = create_huffman_tree(heap);
    let mut result = huffman_codes_from_tree(&Some(ht), bv, HashMap::new());

    // add mapped LZCs to codebook
    for m in mappings.iter() {
        let code = result.get(m.1).unwrap();
        let mut new_bv = BitVec::new();
        for b in code.iter() {
            new_bv.push(b == true);
        }
        result.insert(*m.0, new_bv);
    }
    debug!("Codebook 2: {:?}", result);

    (result, mappings)
}


fn check_saving(huffman_codes: &HashMap<u8, BitVec>, counts: &HashMap<u8, i32>) -> HashMap<u8, u8>{
    let threshold = 0i32;
    debug!("Codebook: {:?}", huffman_codes);
    let cl : BTreeMap<_,_> = huffman_codes.iter().map(|(&k,v)| (k,v.len())).collect();
    debug!("Element & Codelength: {:?}", cl);
    let co : BTreeMap<_,_> = counts.iter().collect();
    debug!("Element & Counts: {:?}", co);

    let mut savings : HashMap<(u8,u8), i32> = HashMap::new();
    // calculate potential saving by comparing every element with every other
    let elements: Vec<u8> = cl.keys().cloned().collect();
    debug!("All elements: {:?}", elements);
    for i in 0..elements.len() {
        let x = elements[elements.len()-i-1];
        for j in i..elements.len() {
            let y = elements[elements.len()-1-j];
            if x <= y {
                continue
            }
            let saving : i32 = (*cl.get(&y).unwrap() as i32 + x as i32 - y as i32 - *cl.get(&x).unwrap() as i32) * **co.get(&x).unwrap();
            if saving < 0 {
                savings.insert((x as u8, y as u8), saving);
            }
        }
    }
    debug!("Potential savings: {:?}", savings);
    // Sorting from smallest saving gain to biggest
    let mut sorted : Vec<(_,_)> = savings.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    debug!("Sorted: {:?}", sorted);

    let mut result : HashMap<u8, u8> = HashMap::new();
    for save in sorted.into_iter() {
        let mapping = save.0;
        if save.1.abs() < threshold {
            continue
        }
        result.insert(mapping.0, mapping.1);
    }
    debug!("Savings: {:?}", result);
    result
}


fn invert_huffman_codes(codes: &HashMap<u8, BitVec>) -> HashMap<BitVec, u8> {

    let mut res = HashMap::new();

    for (k, v) in codes.iter() {
        res.insert(v.clone(), k.clone());
    }

    res
}

fn huffman_codes_from_tree(opt: &Option<Box<Tree>>, prefix: BitVec, mut map: HashMap<u8, BitVec>) -> HashMap<u8, BitVec> {

    if let Some(ref tree) = *opt {
        match tree.value {
            Some(c) => {
                map.insert(c, prefix);
            },
            None => {
                let mut prefix_left = prefix.clone();
                prefix_left.push(true);
                let map = huffman_codes_from_tree(&tree.left, prefix_left, map);
                let mut prefix_right = prefix.clone();
                prefix_right.push(false);
                return huffman_codes_from_tree(&tree.right, prefix_right, map);
            }
        }
    }

    return map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_no_output_by_single_values() {
        let a : Vec<u8> = vec![4,4,4,4,4,4,4,4,4,4,4,4,4];
        let codes = get_huffman_codes(&a);
        let encode = encode(&a, &codes);
        assert_ne!(encode.to_bytes(), Vec::<u8>::new());
    }

    #[test]
    fn byte_counts_simple_string() {
        let a : Vec<u8> = vec![1,2,3,4,5,1,2,1,3,3,1,1,1,1,1];

        let res_fn = get_bytes_counts(&a);
        let res: HashMap<_, _> = a.into_iter().zip(vec![8, 2, 3, 1, 1]).collect();

        assert_eq!(res_fn, res);
    }

    #[test]
    fn decoding_is_inverse() {
        let s : Vec<u8> = vec![1,2,3,4,5,1,2,1,3,3,1,1,1,1,1];

        let hc = get_huffman_codes(&s);

        // let s2 = "babbc";
        let s2 : Vec<u8> = vec![2,1,2,2,3];

        let encoded = encode(&s2, &hc);
        let decoded = decode(encoded, &hc);

        assert_eq!(s2, decoded);
    }
}

#[derive(Eq, Debug, Clone)]
struct Tree {
    count: i32,
    value: Option<u8>,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Ord for Tree {
    fn cmp(&self, other: &Tree) -> Ordering {
        (-self.count).cmp(&(-other.count))
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Tree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
        self.count == other.count
    }
}

impl Tree {

    fn new(value: u8, count: i32) -> Box<Tree> {
        let t = Tree {
            count,
            value: Some(value),
            left: None,
            right: None,
        };

        Box::new(t)
    }

    fn combine(tree_smaller: Box<Tree>, tree_larger: Box<Tree>) -> Box<Tree> {
        let t = Tree {
            count: tree_smaller.count + tree_larger.count,
            value: None,
            left: Some(tree_smaller),
            right: Some(tree_larger),
        };

        Box::new(t)
    }
}
