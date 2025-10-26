use std::{collections::HashMap, env::{self, args}, fs::{File, OpenOptions}, io::{self, BufReader, BufWriter, Cursor, Read, Write}, str::MatchIndices, usize};
use bitstream_io::{BigEndian, BitRead, BitReader};
use postcard::{fixint::le, from_bytes, to_stdvec};
use serde::{Deserialize, Serialize};
use bitvec::{order::Msb0, vec::BitVec};

#[derive(Debug,PartialEq,Serialize,Deserialize,Clone)]
struct HufNode{
    left_child:Option<Box<HufNode>>,
    right_child:Option<Box<HufNode>>,
    freq:i32,
    val:Option<u8>,
}

impl HufNode {

    fn traverse(&self,arr:Vec<i8>,code_map:&mut HashMap<u8,Vec<i8>>){
        if let Some(left) = &self.left_child {
            let mut n_arr = arr.clone();
            n_arr.push(0);
            left.traverse(n_arr,code_map);
        }

        if self.left_child.is_none() && self.right_child.is_none() {
            code_map.insert(self.val.unwrap(), arr.clone());
}

        if let Some(right) = &self.right_child {
            let mut n_arr = arr.clone();
            n_arr.push(1);
            right.traverse(n_arr,code_map);
        };
    }

    fn merge(left:HufNode,right:HufNode)->HufNode{
        let freq = &left.freq + &right.freq;
        let parent = HufNode{
            left_child:Some(Box::new(left)),
            right_child:Some(Box::new(right)),
            freq:freq,
            val:None,
        };
        parent

    }
    fn from_hash(hash_val:(&u8,&i32)) -> HufNode {
        return HufNode{
            left_child:None,
            right_child:None,
            freq:*hash_val.1,
            val:Some(*hash_val.0)
        }
    }
}

fn u8_to_bits(byte: u8) -> Vec<bool> {
    (0..8)
        .map(|i| (byte & (1 << (7 - i))) != 0) // Check if the i-th bit is set
        .collect()
}

fn encode(data:Vec<u8>){
    let mut min_heap = gen_min_heap(&data);
    gen_tree(&mut min_heap);
    let head_data = gen_head(&min_heap); 
    let min_heap = &min_heap[0]; 
    let mut mp = HashMap::new();
    let mut b_mp:HashMap<u8,BitVec<u8,Msb0>> = HashMap::new();
    min_heap.traverse(vec![],&mut mp);
    for i in mp{
        let mut bvec:BitVec<u8, Msb0> = BitVec::new();
        let bl:Vec<bool> = i.1.iter().map(|&x| x != 0).collect();
        for j in bl{
            bvec.push(j);
        }
        b_mp.insert(i.0, bvec);
    }
   


    let mut b_stream:BitVec<u8,Msb0> = BitVec::new();
    let file = OpenOptions::new().create(true).append(true).open("hope.zx").unwrap();
    let mut writer = BufWriter::new(file);
    for i in data{
        let mut l = b_mp.get(&i).unwrap().clone();
        b_stream.append(&mut l);
    }
    writer.write(&head_data).unwrap();
    writer.write(&b_stream.clone().into_vec()).unwrap();    
    writer.flush().unwrap();
}




fn gen_head(map:&Vec<HufNode>) -> Vec<u8>{
    //generate head for tree creation;
    let mut head_data = to_stdvec(&map).unwrap();
    head_data.push(b'\\');
    head_data
}

fn gen_min_heap(data:&Vec<u8>) -> Vec<HufNode>{
    let mut map:HashMap<u8,i32> = HashMap::new();
    for val in data{
        let freq = match map.get(&val){
        Some(freq) => freq,
        None => &0,
        };
        map.insert(*val, freq + 1);
    }
    let mut map = map.iter().collect::<Vec<_>>();
    map.sort_by(|a, b| a.1.cmp(&b.1));
    let mut min_heap:Vec<HufNode> = vec![];
    for val in map{
       min_heap.push(HufNode::from_hash(val)); 
    }
    min_heap
}


fn gen_tree(min_heap:&mut Vec<HufNode>){
    while min_heap.len() > 1{
        let merged = HufNode::merge(min_heap.remove(0), min_heap.remove(0));
        let mut index = 0;
        while index <= min_heap.len(){
            if min_heap.len() > 0{
                if merged.freq > min_heap[index].freq && index < min_heap.len()-1{
                    index += 1;
                    continue;
                }
            }
            if index == min_heap.len(){
                min_heap.push(merged);
                break;
            }
            min_heap.insert(index, merged);
            break;
        } 

    }
}

fn decode(data:&mut BufReader<File>){
    let mut bitstream:BitVec<u8,Msb0> = BitVec::new();
    io::copy(data,&mut bitstream).unwrap(); 
    let b = bitstream.into_vec();
    let mut index = 0;
    for (inx,byte) in b.iter().enumerate(){
        if *byte == 92{
           index = inx; 
           break;
        }
    }
    let heap:Vec<HufNode> = from_bytes(&b[0..index]).unwrap();
    let mut bits:Vec<bool> = vec![];
    for byte in &b[index+1..]{
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1; // Shift the bit to the LSB position and then mask
            if bit == 1{
                bits.push(true);
            }else{
                bits.push(false);
            }
        }
    }
    let mut leaf = heap[0].clone();
    let mut data:Vec<u8> = vec![];
    for i in bits{
        if leaf.left_child.is_none() && leaf.right_child.is_none(){
            data.push(leaf.val.unwrap());
            leaf = heap[0].clone();
        }
        if i{
            leaf = *leaf.right_child.unwrap();
        }else{
            leaf = *leaf.left_child.unwrap();
        }
    }
    let file = OpenOptions::new().create(true).append(true).open("./hope.txt").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&data).unwrap();
    writer.flush().unwrap();

}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args[1] == String::from("d"){
        //let file = File::open("./ho.txt").unwrap();
        let file = File::open("./hope.zx").unwrap();
        let mut buffer = BufReader::new(file);
        decode(&mut buffer);
        //encode(data);
    }else{
        let file = File::open("./test.txt").unwrap();
        let mut buffer = BufReader::new(file);
        let mut data:Vec<u8>  = vec![];
        buffer.read_to_end(&mut data).unwrap();
        encode(data);
    }
}

