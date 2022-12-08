use ::{
    image2::RgbaImage,
    std::{env, error::Error},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let width = args.next().unwrap();
    let height = args.next().unwrap();

    lightball(width.parse()?, height.parse()?, [255, 255, 255])
        .unwrap()
        .save("image.png")?;

    Ok(())
}

// now instead of a ball it's sometimes a diamond,sometimes.It was a funny experiment but don't expect this messy app in crates.io
fn lightball(width: u32, height: u32, color: [u8; 3]) -> Option<RgbaImage> {
    let mut lightball = Vec::with_capacity(width.checked_mul(height)?.checked_mul(4)? as usize);
    let transparent = [255, 255, 255, 0];
    let mut wa = Vec::with_capacity(width as usize);

    if width % 2 == 0 {
        let mid = width / 2;

        for _ in 0..mid {
            wa.push(false);
        }

        for _ in 0..2 {
            wa.push(true)
        }

        for _ in (mid + 2)..width {
            wa.push(false)
        }
    } else {
        let mid = (width / 2) + 1;

        for _ in 0..mid {
            wa.push(false);
        }

        wa.push(true);

        for _ in (mid + 1)..width {
            wa.push(false)
        }
    }

    let rest = height % width;
    let m = height / width;

    let mut ttc = 0; 
    let mut reverse = false;

    for _ in 0..height {
        for wa1 in wa.iter().copied() {
            if wa1 {
                lightball.extend(color);
                lightball.push(255)
            } else {
                lightball.extend(transparent);
            }
        }

        ttc += 1;

        if ttc == m {
            ttc = 0;
            if reverse {
                let a = wa.iter().copied().position(|v| v).unwrap();
                let b = wa.iter().copied().rposition(|v| v).unwrap();

                if let Some(e) = wa.get_mut(a) {
                    *e = false;
                    if let Some(e) =  wa.get_mut(b) {
                        *e = false;
                    }
                }
            } else {
                let a = wa.iter().copied().position(|v| v).unwrap();
                let b = wa.iter().copied().rposition(|v| v).unwrap();

                if let Some(e) = wa.get_mut(a-1) {
                    *e = true;
                    if let Some(e) =  wa.get_mut(b + 1) {
                        *e = true;
                    }
                } else {
                    reverse = true;
                }    
            } 
        }        
    }

    RgbaImage::from_vec(width, height, lightball)
}
