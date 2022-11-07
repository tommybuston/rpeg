use csc411_image::{RgbImage,Read, Rgb};
use csc411_arith;
use array2::Array2;

#[derive(Clone)]
struct RgbFloating { 
    red : f64,
    blue : f64,
    green : f64,
}

pub fn compress(filename: Option<&str>) {
    let image = RgbImage::read(filename).unwrap();
    let denom : f64 = image.denominator as f64;
    let mut image_a2 : Array2<Rgb> = Array2::from_row_major(image.pixels,
                                                       image.width as usize,
                                                       image.height as usize);
    
    shave_image(&mut image_a2);
    let mut image_a2_float = to_float(&image_a2, denom);


}


pub fn decompress(_filename: Option<&str>) { 
    todo!();
}


fn shave_image( image_a2 : &mut Array2<Rgb> ) {
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

fn to_float ( image_a2 : &Array2<Rgb>, denom : f64 ) -> Array2 <RgbFloating> {
    let w = image_a2.width;
    let h = image_a2._height;


    let mut image_a2_float : Array2 <RgbFloating> = Array2::from_row_major( vec![RgbFloating {
                                                    red : 0.0,
                                                    green : 0.0, 
                                                    blue : 0.0 }
                                                    ; w*h], w, h );
    
    for pix in image_a2.iter_row_major(){
        let x = pix.0;
        let y = pix.1;

        let current = image_a2_float.get_mut_value(x, y); 
        current.red = (pix.2.red as f64) / denom;
        current.green = (pix.2.green as f64) / denom;
        current.blue = (pix.2.blue as f64) / denom;
    }

    image_a2_float

}
