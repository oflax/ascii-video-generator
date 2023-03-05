use image::{GenericImageView, Pixel, imageops::FilterType::Gaussian};
use std::{fs, fs::File, io, io::{prelude::*, BufReader}, thread, time};
use term_size;

fn main() -> std::io::Result<()> {
    let entries = fs::read_dir("frames")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    let size = term_size::dimensions().unwrap();
    let _nwidth = size.0 as u32/2;
    let nheight = size.1 as u32;

    for index in 0..entries.len()-1 {
        let mut file = File::create("frame_data/".to_owned()+&index.to_string())?;
        let mut img = image::io::Reader::open("frames/frame".to_owned()+&index.to_string()+".jpg")?
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        // img = img.resize_exact(nwidth*2, img.height()*nwidth/img.width(), Gaussian);
        img = img.resize_exact(img.width()*nheight/img.height()*2, nheight, Gaussian);
        let gscale = r"  ..,:;coahkbzx0X#"; // asciigrayscale levels
        let div: f32 = 255.0/((gscale.len()-1) as f32);
        for i in 0..img.height() {
            for j in 0..img.width() {
                let color = img.get_pixel(j, i);
                let colorgb = img.get_pixel(j, i).0;
                let px = color.to_luma().0[0] as f32;
                let idx = (px/div) as usize;
                // file.write(gscale.chars().nth(idx).unwrap().to_string().as_bytes())?; // no colors
                file.write((format!("\x1b[38;2;{};{};{}m", colorgb[0], colorgb[1], colorgb[2]).to_owned()+&gscale.chars().nth(idx).unwrap().to_string()).as_bytes())?; // colored
            }
            file.write(b"\n")?;
        }
        clearscreen::clear().expect("fail");
        println!("\x1b[36m[INFO] - Generating ASCII frame ({}/{})\x1b[0m", index+1, entries.len());
    }

    for frame in 0..entries.len() {
        let file = File::open("frame_data/".to_owned()+&frame.to_string())?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        clearscreen::clear().expect("fail");
        for i in contents.chars() {
            print!("{}", i)
        }
        thread::sleep(time::Duration::from_millis(1000/30)); // 30 fps
    }

    Ok(())
}
