use bitpack::bitpack::{getu, gets};
use array2::Array2;
use csc411_image::{Rgb};
use csc411_arith::{chroma_of_index};
use crate::compress_tools::{Vid, RgbFloating};

pub fn code_word_format(be_bytes : &Vec<[u8; 4]> , image_code_words : &mut Vec<u64>) {
    for bytes in be_bytes.iter() {
        image_code_words.push( u32::from_be_bytes(*bytes) as u64 );
    }
}

pub fn unpack_bits(image_code_words : &Vec<u64> , image_a2_vid : &mut Array2<Vid>) {
    let mut i = 0;
    for word in image_code_words {
        let a = getu(*word, 9, 23) as f32;
        let b = gets(*word, 5, 18) as f32 / 50.0;
        let c = gets(*word, 5, 13) as f32 / 50.0;
        let d = gets(*word, 5, 8) as f32 / 50.0;
        let pr_avg = chroma_of_index(getu(*word, 4, 4) as usize );
        let pb_avg = chroma_of_index(getu(*word, 4, 0) as usize );

        image_a2_vid.values[i].a = a;
        image_a2_vid.values[i].b = b;
        image_a2_vid.values[i].c = c;
        image_a2_vid.values[i].d = d;
        image_a2_vid.values[i].pr_avg = pr_avg;
        image_a2_vid.values[i].pb_avg = pb_avg;

        i += 1;
    }
}

pub fn to_rgb_float(image_a2_vid : &Array2<Vid>, image_a2_float : &mut Vec<RgbFloating>) {
    //let og_w = image_a2_float.width;
    //let og_h = image_a2_float._height;

    let comp_w = image_a2_vid.width;
    let comp_h = image_a2_vid._height;

    let og_w = comp_w * 2;

    for square_row in 0..comp_h {
        for square_col in 0..comp_w {
            let vid_data = image_a2_vid.get_value(square_col, square_row);

            let pb = vid_data.pb_avg;
            let pr = vid_data.pr_avg;
            let a = vid_data.a;
            let b = vid_data.b;
            let c = vid_data.c;
            let d = vid_data.d;

            let y = vec![a-b-c+d, a-b+c-d, a+b-c-d, a+b+c+d ];

            let mut i : usize = 0;
            for j in 0..2 {
                for pix in image_a2_float.iter_mut().skip(square_row*2*og_w + square_col*2 + j*og_w).take(2) {
                        pix.red = 1.0 * y[i] + 0.0 * pb + 1.402 * pr;
                        pix.green = 1.0 * y[i] - 0.344136 * pb - 0.714136 * pr;
                        pix.blue = 1.0 * y[i] + 1.772 * pb + 0.0 * pr;
                        i += 1;
                }
            }

        }
    }
}


