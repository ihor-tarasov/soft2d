use crate::core::*;

pub struct Image {
    pixels: Box<[u32]>,
    size: IVec2,
}

impl Image {
    pub fn new(width: u32, height: u32, color: Color) -> Self {
        debug_assert!(width <= i32::MAX as u32 && height <= i32::MAX as u32);
        Self {
            pixels: vec![color.as_u32(); (width * height) as usize].into_boxed_slice(),
            size: ivec2(width as i32, height as i32),
        }
    }

    #[cfg(feature = "png")]
    pub fn open<P>(path: P) -> Self
    where
        P: AsRef<std::path::Path>,
    {
        let image = image::open(path).unwrap().to_rgba8();
        let (width, height) = image.dimensions();
        let mut result = Self::new(width, height, Color::BLACK);
        for (x, y, pixel) in image.enumerate_pixels() {
            let [r, g, b, a] = pixel.0;
            result.set_pixel(ivec2(x as i32, y as i32), Color::from_rgba(r, g, b, a));
        }
        result
    }
}

impl Surface for Image {
    fn get_pixel(&self, pos: IVec2) -> Color {
        Color::from_u32(self.pixels[Self::index(pos, self.size.x) as usize])
    }

    fn set_pixel(&mut self, pos: IVec2, color: Color) {
        self.pixels[Self::index(pos, self.size.x) as usize] = color.as_u32();
    }

    fn size(&self) -> IVec2 {
        self.size
    }

    fn clear(&mut self, color: Color) {
        self.pixels.fill(color.as_u32());
    }
}
