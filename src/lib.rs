/// Original code from https://github.com/jalexvig/comprustion

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

use bit_vec::BitVec;

pub mod hufbites;

pub fn encode(data: &str, huffman_codes: &HashMap<char, BitVec>) -> BitVec {

    let mut nbits = 0;
    for c in data.chars() {
        nbits += huffman_codes.get(&c).unwrap().len();
    }

    let mut res = BitVec::with_capacity(nbits);

    for c in data.chars() {
        let bv = huffman_codes.get(&c).unwrap();
        for bit in bv.iter() {
            res.push(bit);
        }
    }

    res
}

pub fn decode(bits: BitVec, huffman_codes: &HashMap<char, BitVec>) -> String {

    let hci = invert_huffman_codes(huffman_codes);

    let mut res = String::new();
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

fn get_char_counts(data: &str) -> HashMap<char, i32> {

    let mut char_counts = HashMap::new();

    for c in data.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }

    char_counts
}

fn heapify(map: HashMap<char, i32>) -> BinaryHeap<Box<Tree>> {

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

pub fn get_huffman_codes(data: &str) -> HashMap<char, BitVec> {

    let char_counts = get_char_counts(data);

    let heap = heapify(char_counts);

    let ht = create_huffman_tree(heap);

    return huffman_codes_from_tree(&Some(ht), BitVec::new(), HashMap::new());
}

fn invert_huffman_codes(codes: &HashMap<char, BitVec>) -> HashMap<BitVec, char> {

    let mut res = HashMap::new();

    for (k, v) in codes.iter() {
        res.insert(v.clone(), k.clone());
    }

    res
}

fn huffman_codes_from_tree(opt: &Option<Box<Tree>>, prefix: BitVec, mut map: HashMap<char, BitVec>) -> HashMap<char, BitVec> {

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
    fn char_counts_simple_string() {
        let a = "abcdeabaccaaaaa";

        let res_fn = get_char_counts(a);
        let res: HashMap<_, _> = a.chars().zip(vec![8, 2, 3, 1, 1]).collect();

        assert_eq!(res_fn, res);
    }

    #[test]
    fn decoding_is_inverse() {
        let s = "abcdeabaccaaaaa";

        let hc = get_huffman_codes(s);

        let s2 = "babbc";

        let encoded = encode(s2, &hc);
        let decoded = decode(encoded, &hc);

        assert_eq!(s2, decoded);
    }
}

#[derive(Eq, Debug, Clone)]
struct Tree {
    count: i32,
    value: Option<char>,
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

    fn new(value: char, count: i32) -> Box<Tree> {
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
