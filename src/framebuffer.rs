use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    Pixel,
};

pub struct FrameBuffer {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }
}

impl OriginDimensions for FrameBuffer {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl DrawTarget for FrameBuffer {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            if coord.x >= 0
                && coord.y >= 0
                && (coord.x as usize) < self.width
                && (coord.y as usize) < self.height
            {
                let index = coord.y as usize * self.width + coord.x as usize;
                self.buffer[index] =
                    ((color.r() as u32) << 16) | ((color.g() as u32) << 8) | (color.b() as u32);
            }
        }
        Ok(())
    }
}
