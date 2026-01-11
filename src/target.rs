use std::{fs::File, io::BufWriter, path::Path};

use image::{Rgb, RgbImage};

use crate::{
    colour::Colour,
    geometry::{Point2, Rect, space::TargetSpace},
    grid::Grid,
};

pub struct Target {
    pub width: u32,
    pub height: u32,
    film: Grid<Colour>,
}

impl Target {
    pub fn new(width: u32, height: u32) -> Self {
        let film = Grid::create_uniform(width as usize, height as usize, Colour::zero());

        Self {
            width,
            height,
            film,
        }
    }

    pub fn rect(&self) -> Rect<TargetSpace, u32> {
        Rect::new(Point2::new(0, 0), Point2::new(self.width, self.height))
    }

    pub fn set_pixel(&mut self, pixel: Point2<TargetSpace, u32>, colour: Colour) {
        self.film.set(pixel.x as usize, pixel.y as usize, colour);
    }

    pub fn write_png(&self, path: impl AsRef<Path>) {
        let mut image = RgbImage::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let pixel_colour = self.film.get(x as usize, y as usize);

                let colour = Rgb([
                    (255.0 * pixel_colour.r) as u8,
                    (255.0 * pixel_colour.g) as u8,
                    (255.0 * pixel_colour.b) as u8,
                ]);

                image.put_pixel(x, y, colour);
            }
        }

        let mut writer = BufWriter::new(File::create(path).expect("Failed to create file"));

        image
            .write_to(&mut writer, image::ImageFormat::Png)
            .expect("Failed to write image to file");
    }
}
