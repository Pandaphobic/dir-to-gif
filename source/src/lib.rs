use napi_derive::napi;

// image imports
extern crate image;
use image::codecs::gif::GifEncoder;

// fs imports
use std::fs::File;
use std::fs::read_dir;
use image::open;

#[napi]
pub fn build_gif_from_dir(in_dir: String, out_dir: String, out_name: String ){
  // get paths to each image
  let mut paths: Vec<_> = read_dir(in_dir).unwrap()
  .map(|r| r.unwrap())
  .collect();
  // sort paths alphabetically
  paths.sort_by_key(|dir| dir.path());


  // establish full out path
  let out_path: String = format!("{}/{}.gif", out_dir, out_name);
  let file_out: File = File::create(&out_path).unwrap();


  // setup encoder
  let mut encoder: GifEncoder<File> = GifEncoder::new(file_out);
  encoder.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();
  
  
  for path in paths {
    
    let rgba = open(path.path()).unwrap().into_rgba8();
    encoder.encode_frame(image::Frame::from_parts(
      rgba,
        0,
        0,
        image::Delay::from_numer_denom_ms(160, 1),
      )).unwrap();     
  }
}
