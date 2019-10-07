use bmp;


fn main() 
{
    let img = bmp::open("provinces.bmp").expect("TH");
    let my_pix = bmp::Pixel::new(255, 255, 255);
    let mut i: usize = 0;
    for (x,y) in img.coordinates()
    {
        if img.get_pixel(x,y) == my_pix
        {
            i += 1;
        }
    }
    println!("{}", i);
}
