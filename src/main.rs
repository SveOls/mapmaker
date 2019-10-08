use bmp::{Image, Pixel};

fn main() {
    let img: Image; 
    match bmp::open("input/provinces.bmp") {
        Ok(i) => img = i,
        Err(e) => { println!("Error: {}", e); return; },
    };
    let mut sea = Image::new(img.get_width(), img.get_height());
    let mut prov = Image::new(img.get_width(), img.get_height());
    let mut prevcolor: Pixel;
    let ocean = Pixel::new(255, 255, 255);
    let mut wasocean: bool = false;
    for y in 0..img.get_height() {
        if img.get_pixel(0, y) == ocean { wasocean = true;  sea.set_pixel(0, y, Pixel::new(255, 255, 255)); }
        else { sea.set_pixel(0, y, Pixel::new(100, 100, 100)); }
        prevcolor = img.get_pixel(0, y);
        for x in 1..img.get_width() {
            if img.get_pixel(x, y) == ocean {
                if !wasocean {
                    sea.set_pixel(x, y, Pixel::new(0, 0, 0));
                }
                else {
                    sea.set_pixel(x, y, Pixel::new(255, 255, 255));
                }
                if y != 0 {
                    if img.get_pixel(x, y - 1) != ocean {
                        sea.set_pixel(x, y, Pixel::new(0, 0, 0));   
                    }
                }
                wasocean = true;
            }
            else {
                if wasocean {
                    sea.set_pixel(x - 1, y, Pixel::new(0, 0, 0));
                }
                sea.set_pixel(x, y, Pixel::new(155, 155, 155));
                wasocean = false;
                if y != 0 {
                    if img.get_pixel(x, y - 1) == ocean {
                        sea.set_pixel(x, y - 1, Pixel::new(0, 0, 0));
                    }
                }
            }
            if img.get_pixel(x, y) != prevcolor && prevcolor != ocean && img.get_pixel(x, y) != ocean {
                prov.set_pixel(x, y, Pixel::new(0, 0, 0));
            }
            else if y != 0 && prevcolor != ocean && img.get_pixel(x, y) != ocean {
                if img.get_pixel(x, y) != img.get_pixel(x, y - 1) && img.get_pixel(x, y - 1) != ocean {
                    prov.set_pixel(x, y, Pixel::new(0, 0, 0));
                }
                else {
                    prov.set_pixel(x, y, Pixel::new(255, 255, 255));
                }
            }
            else {
                prov.set_pixel(x, y, Pixel::new(255, 255, 255));
            }
            prevcolor = img.get_pixel(x, y);
        }
        wasocean = false;
    }
    let _ = sea.save("sea.bmp");
    let _ = prov.save("prov.bmp");
}