use ::{
    image2::RgbaImage,
    std::{env, error::Error},
};

/*
gray = 240, 240, 240, 255

if pixel_width == 1 && pixel_height == 1 {
    lightball.extend([255, 255, 255, 255]);
} else if pixel_width == 3 && pixel_height == 3 {
    lightball = vec![
                transparent, gray, transparent
                gray, white, gray,
                transparent, gray, transparent];
}
*/

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let width = args.next().unwrap();
    let height = args.next().unwrap();

    lightball(width.parse()?, height.parse()?, [255, 255, 255])
        .unwrap()
        .save("image.png")?;

    Ok(())
}

fn lightball(width: u32, height: u32, color: [u8; 3]) -> Option<RgbaImage> {
    let mut lightball = Vec::with_capacity(width.checked_mul(height)?.checked_mul(4)? as usize);

    let transparent = [255, 255, 255, 0];

    // 12.5 percent in each corner 10% transparent 1% alpha 200 1.5% alpha 100

    // should be the number of pixels allowed in x axis starting in the middle
    let mut wa = 1;
    let mut mid = width / 2;

    for h in 1..=height {
        for w in 1..=width {
            if w == mid {
                lightball.extend(color);
                lightball.push(255);                
            } else {
                lightball.extend(transparent);
            }
        }     
    }

    RgbaImage::from_vec(width, height, lightball)
}
