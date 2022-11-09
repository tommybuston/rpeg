use bitpack::bitpack::{news, newu};
use array2::Array2;
use csc411_image::{ Rgb};
use csc411_arith::{index_of_chroma};

#[derive(Clone)]
pub struct RgbFloating { 
    pub red : f32,
    pub blue : f32,
    pub green : f32,
}

#[derive(Clone)]
pub struct Vid { 
    pub a : f32,
    pub b : f32,
    pub c : f32,
    pub d : f32,
    pub pb_avg : f32,
    pub pr_avg : f32,
}


pub fn shave_image( image_a2 : &mut Array2<Rgb> ) {
    let w = image_a2.width;
    let h = image_a2._height;

    if w % 2 != 0 && h % 2 != 0 {
        
        let mut shaved_pixels : Vec<Rgb> = vec![Rgb {red : 0,
                                                    green : 0, 
                                                    blue : 0 }
                                                    ; (w - 1)*h];
        let mut i : usize = 0;

        for pix in image_a2.iter_row_major() {
            if i % (w - 1) == 0 {continue;}

            shaved_pixels[i].red = pix.2.red;
            shaved_pixels[i].green = pix.2.green;
            shaved_pixels[i].blue = pix.2.blue;

            i += 1;
        }
        
        shaved_pixels.truncate( (h - 1)*(w - 1) );

        image_a2.values = shaved_pixels;
        image_a2.width = w - 1;
        image_a2._height = h - 1;


    }
    else if w % 2 != 0 {

        let mut shaved_pixels : Vec<Rgb> = vec![Rgb {red : 0,
                                                    green : 0, 
                                                    blue : 0 }
                                                    ; w*(h - 1)];
        let mut i : usize = 0;

        for pix in image_a2.iter_row_major() {
            if i % (w - 1) == 0 {continue;}

            shaved_pixels[i].red = pix.2.red;
            shaved_pixels[i].green = pix.2.green;
            shaved_pixels[i].blue = pix.2.blue;


            i += 1;
        }

        image_a2.values = shaved_pixels;
        image_a2.width = w - 1;

    }
    else if h % 2 != 0 {

        image_a2.values.truncate( (h - 1)*w );
        image_a2._height = h - 1;
        
    }

}

pub fn to_float ( image_a2 : &Array2<Rgb>,
              image_a2_float : &mut Array2 <RgbFloating>,
              denom : f32 ) {
    
    for pix in image_a2.iter_row_major(){
        let x = pix.0;
        let y = pix.1;

        let current = image_a2_float.get_mut_value(x, y); 
        current.red = (pix.2.red as f32) / denom;
        current.green = (pix.2.green as f32) / denom;
        current.blue = (pix.2.blue as f32) / denom;
    }

}

pub fn to_component_video(image_a2_float : &Array2<RgbFloating>,
                      image_a2_vid : &mut Vec <Vid> ) {
    //let mut i_debug = 0;
    let og_w = image_a2_float.width;
    let og_h = image_a2_float._height;

    let new_w = og_w / 2;
    let new_h = og_h / 2;

    //println!("(W,H) = ({:?}, {:?})", new_w, new_h);

    for square_row in 0..(new_h) {
        for square_col in 0..(new_w) {
            
            let mut y_vals : Vec<f32> = Vec::new();
            let mut pb_vals : f32 = 0.0;
            let mut pr_vals : f32 = 0.0;
            
            for j in 0..2 {

                for pix in image_a2_float.values.iter()
                    .skip(square_row*2*og_w+j*og_w+square_col*2).take(2) {
                    //println!("(x,y) = ({:?}, {:?})", pix.0, pix.1);
                    
                    let r = pix.red;
                    let g = pix.green;
                    let b = pix.blue;

                    let y = 0.299 * r + 0.587 * g + 0.114 * b;
                    let pb = - 0.168736 * r - 0.331264 * g + 0.5 * b;
                    let pr = 0.5 * r - 0.418688 * g - 0.081312 * b;

                    y_vals.push(y);
                    pb_vals += pb;
                    pr_vals += pr;
                    
                    //println!("{:?}", i_debug);
                    //i_debug = i_debug + 1;
                    
                }
                
            }
            
            let mut current_square = &mut image_a2_vid[square_row * new_w + square_col];

            let a_current = (y_vals[3] + y_vals[2] + y_vals[1] + y_vals[0])/4.0;
            let b_current = (y_vals[3] + y_vals[2] - y_vals[1] - y_vals[0])/4.0; 
            let c_current = (y_vals[3] - y_vals[2] + y_vals[1] - y_vals[0])/4.0;
            let d_current = (y_vals[3] - y_vals[2] - y_vals[1] + y_vals[0])/4.0;
            let pb_avg_current : f32 = pb_vals / 4.0;
            let pr_avg_current : f32 = pr_vals / 4.0;

            current_square.a = a_current;
            current_square.b = b_current;
            current_square.c = c_current;
            current_square.d = d_current;
            current_square.pb_avg = pb_avg_current;
            current_square.pr_avg = pr_avg_current;
            
            //if square_col % 639 == 0 {println!("(x,y) = ({:?},{:?})",square_col,square_row);}
        }
    }

    //println!("Ran!");

}

pub fn pack_bits(image_a2_vid : &Array2 <Vid>, compressed_image_a2 : &mut Array2 <u64>) {

    for square in image_a2_vid.iter_row_major() {
        let mut bits : u64 = 0;
        let x = square.0;
        let y = square.1;

        let pb_quant : u64 = index_of_chroma(square.2.pb_avg) as u64;
        let pr_quant : u64 = index_of_chroma(square.2.pr_avg) as u64;
        let a_quant : u64 = (square.2.a * 511.0).round() as u64;
        let b_quant : i64 = b_c_d_quantize(square.2.b);
        let c_quant : i64 = b_c_d_quantize(square.2.c);
        let d_quant : i64 = b_c_d_quantize(square.2.d);

        bits = newu(bits, 4, 0, pr_quant).unwrap();
        bits = newu(bits, 4, 4, pb_quant).unwrap();
        bits = news(bits, 5, 8, d_quant).unwrap();
        bits = news(bits, 5, 13, c_quant).unwrap();
        bits = news(bits, 5, 18, b_quant).unwrap();
        bits = newu(bits, 9, 23, a_quant).unwrap();

        let current = compressed_image_a2.get_mut_value(x,y);
        
        *current = bits;

    }

}

pub fn b_c_d_quantize(num : f32) -> i64 {
    if num >= 0.3 {
        return 15;
    }
    else if num <= -0.3 {
        return -15;
    }
    
    (num*50.0).round() as i64
}

pub fn isaac_chen_format( code_words : &Vec<u64>, final_bytes : &mut Vec<[u8 ; 4]> ) {
    
    for word in code_words.iter() {
        final_bytes.push( (*word as u32).to_be_bytes() );
    }

}


