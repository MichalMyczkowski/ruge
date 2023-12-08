
use gl::types::*;
use image::io::Reader as ImageReader;
use image::{Rgba, EncodableLayout};

pub struct Texture {
    texture_id: GLuint,
}


impl Texture {
    pub fn load_image(&self, img_src: &str) {
        let img = ImageReader::open(img_src).expect(&format!("Couldn't load '{}' texture file!", img_src));
        let img = img.decode().expect(&format!("Couldn't decode '{}' texture file!", img_src));
        //let img = img.into_rgba8();
        let img = img.into_rgba8();
        let (w, h) = (img.width() as i32, img.height() as i32);
        let img: Vec<u8> = Vec::from(img.as_bytes());

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, w, h, 0, gl::RGBA, gl::UNSIGNED_BYTE, img.as_ptr() as *const GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        
    }

    pub fn bind_texture(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

}


impl Default for Texture {
    fn default() -> Self {
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
        Texture {
            texture_id,
        }
    }
}

impl From<&str> for Texture {
    fn from(value: &str) -> Self {
        let t = Texture::default();   
        t.load_image(value);
        t
    }
}
