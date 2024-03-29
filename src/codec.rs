use csc411_image::{RgbImage,Read,Rgb,Write};
use csc411_rpegio::{output_rpeg_data, read_in_rpeg_data};
use array2::Array2;
use crate::compress_tools::{RgbFloating, Vid, shave_image, to_float, 
                            to_component_video, pack_bits,isaac_chen_format};
use crate::decompress_tools::{code_word_format, unpack_bits, to_rgb_float, to_int};

///Compresses a .ppm file to an ".rpeg" if no file name is provided
///the file will be read from standard in.  The .rpeg file will be written
///to standard out.
/// 
/// # Arguments:
/// * `filename`: An option to hold the name of the file to be compressed
pub fn compress(filename: Option<&str>) {
    let image = RgbImage::read(filename).unwrap();
    let denom : f32 = image.denominator as f32;
    let mut image_a2 : Array2<Rgb> = Array2::from_row_major(image.pixels,
                                                       image.width as usize,
                                                       image.height as usize);
     
    shave_image(&mut image_a2);

    let mut image_a2_float : Array2<RgbFloating> = Array2::from_row_major( 
                                                    vec![RgbFloating {
                                                    red : 0.0,
                                                    green : 0.0, 
                                                    blue : 0.0 }; 
                                                    image_a2.width*image_a2._height], 
                                                    image_a2.width, 
                                                    image_a2._height );
    
    to_float(&image_a2, &mut image_a2_float, denom);
    
    let mut image_a2_vid : Array2<Vid> = Array2::from_row_major( 
                                                    vec![Vid {
                                                    a : 0.0,
                                                    b : 0.0, 
                                                    c : 0.0, 
                                                    d : 0.0,
                                                    pb_avg : 0.0,
                                                    pr_avg : 0.0 }; 

                                                    (image_a2.width / 2 )*(image_a2._height / 2)], 
                                                    image_a2.width / 2, 
                                                    image_a2._height / 2 );


    to_component_video(&image_a2_float, &mut image_a2_vid.values);
    
    let mut compressed_image_a2 : Array2<u64> = Array2::from_row_major( vec![0 as u64; 
                                                                        image_a2_vid.width *
                                                                        image_a2_vid._height ],
                                                                        image_a2_vid.width,
                                                                        image_a2_vid._height);
    
    pack_bits(&image_a2_vid, &mut compressed_image_a2);

    let mut final_bytes : Vec< [u8; 4] > = Vec::new();

    isaac_chen_format(&compressed_image_a2.values, &mut final_bytes);

    output_rpeg_data(&final_bytes, compressed_image_a2.width as u32, 
                                    compressed_image_a2._height as u32);

}

///Decompresses an ".rpeg" file to a .ppm file. If no file name is provided
///the file will be read from standard in.  The .ppm file will be written
///to standard out.
/// 
/// # Arguments:
/// * `filename`: An option to hold the name of the file to be decompressed
pub fn decompress(filename: Option<&str>) {
    
    let image = read_in_rpeg_data(filename).unwrap();

    let mut image_code_words : Vec<u64> = Vec::new();

    code_word_format(&image.0, &mut image_code_words);

    let mut image_a2_vid : Array2<Vid> = Array2::from_row_major( 
                                                    vec![Vid {
                                                    a : 0.0,
                                                    b : 0.0, 
                                                    c : 0.0, 
                                                    d : 0.0,
                                                    pb_avg : 0.0,
                                                    pr_avg : 0.0 }; 

                                                    (image.1 *image.2) as usize], 
                                                    image.1 as usize, 
                                                    image.2 as usize );

    unpack_bits(&image_code_words, &mut image_a2_vid);

    let mut image_a2_float : Array2<RgbFloating> = Array2::from_row_major( 
                                                    vec![RgbFloating {
                                                    red : 0.0,
                                                    green : 0.0, 
                                                    blue : 0.0 }; 
                                                    (image.1*image.2*4) as usize], 
                                                    (image.1*2) as usize, 
                                                    (image.2*2) as usize );

    to_rgb_float(&image_a2_vid, &mut image_a2_float.values);
    
    let mut image_a2 : Array2<Rgb> = Array2::from_row_major( 
                                                    vec![Rgb {
                                                    red : 0,
                                                    green : 0, 
                                                    blue : 0 }; 
                                                    (image.1*image.2*4) as usize], 
                                                    (image.1*2) as usize, 
                                                    (image.2*2) as usize );

    
    to_int(&image_a2_float, &mut image_a2);

    let final_image = RgbImage { pixels : image_a2.values,
                                            width : image_a2.width as u32,
                                            height : image_a2._height as u32, 
                                            denominator : 255};

    final_image.write(None).unwrap();
}

