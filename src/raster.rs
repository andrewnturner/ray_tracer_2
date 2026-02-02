use std::{fs::File, io::BufWriter, path::Path};

use image::{Rgb, RgbImage};

use crate::{
    colour::Colour,
    geometry::{Point2, Rect, space::RasterSpace},
    grid::Grid,
};

pub struct Raster {
    pub width: u32,
    pub height: u32,
    film: Grid<Colour>,
}

impl Raster {
    pub fn new(width: u32, height: u32) -> Self {
        let film = Grid::create_uniform(width as usize, height as usize, Colour::zero());

        Self {
            width,
            height,
            film,
        }
    }

    pub fn rect(&self) -> Rect<RasterSpace, u32> {
        Rect::new(Point2::new(0, 0), Point2::new(self.width, self.height))
    }

    pub fn set_pixel(&mut self, pixel: Point2<RasterSpace, u32>, colour: Colour) {
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

                // Raster space starts from bottom left and y goes up, but the image buffer starts
                // from top left and y goes down.
                image.put_pixel(x, self.height - y - 1, colour);
            }
        }

        let mut writer = BufWriter::new(File::create(path).expect("Failed to create file"));

        image
            .write_to(&mut writer, image::ImageFormat::Png)
            .expect("Failed to write image to file");
    }
}
