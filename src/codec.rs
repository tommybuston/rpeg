use csc411_image::{RgbImage,Read, Rgb};
use csc411_arith;
use array2::Array2;

pub fn compress(filename: Option<&str>) {
    let image = RgbImage::read(filename).unwrap();
    let mut image_A2 : Array2<Rgb> = Array2::from_row_major(image.pixels,
                                                       image.width as usize,
                                                       image.height as usize);

    if image.width % 2 != 0 && image.height % 2 != 0 {
        
        let mut shaved_pixels : Vec<Rgb> = Vec::new();
        let mut i = 0;

        for pix in image_A2.iter_row_major() {
            if i % (image.width - 1) == 0 {continue;}
            shaved_pixels.push(*pix.2);
            i += 1;
        }
        
        shaved_pixels.truncate( ((image.height - 1)*(image.width - 1)) as usize );

        image_A2.values = shaved_pixels;
        image_A2.width = image_A2.width - 1;
        image_A2._height = image_A2._height - 1;


    }
    else if image.width % 2 != 0 {

        let mut shaved_pixels : Vec<Rgb> = Vec::new();
        let mut i = 0;

        for pix in image_A2.iter_row_major() {
            if i % (image.width - 1) == 0 {continue;}
            shaved_pixels.push(*pix.2);
            i += 1;
        }

        image_A2.values = shaved_pixels;
        image_A2.width = image_A2.width - 1;

    }
    else if image.height % 2 != 0 {

        image_A2.values.truncate( ((image.height - 1)*(image.width)) as usize );
        image_A2._height = (image.height - 1) as usize;
        
    }

}

pub fn decompress(_filename: Option<&str>) { 
    todo!();
}
