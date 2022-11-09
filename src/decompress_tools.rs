//use bitpack::bitpack::{news, newu};
use array2::Array2;
use csc411_image::{Rgb};
use csc411_arith::{index_of_chroma};

pub fn code_word_format(be_bytes : &Vec<[u8; 4]> , image_code_words : &mut Vec<u64>) {
    for bytes in be_bytes.iter() {
        image_code_words.push( u32::from_be_bytes(*bytes) as u64 );
    }
}
