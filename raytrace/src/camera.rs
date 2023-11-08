use image::{ImageBuffer, Rgba};

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub struct Camera{
    image_width: u32, 
    image_height: u32
}

impl Camera{
    pub fn new(image_width: u32, image_height: u32)-> Self{
        Self{
            image_width,
            image_height
        }
    }

    pub fn render(&self) -> Image{
        let mut image = ImageBuffer::new(self.image_width, self.image_height);
        image
    }

    pub fn render_to_disk(filename: &str, image: Image){
        image.save(filename).unwrap(); 
    }
}