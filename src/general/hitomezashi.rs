use anyhow::{anyhow, Result};
use image::Luma;

pub struct HitomezashiImage {
    image: image::GrayImage,
    line_length: u32,
    message: String,
    key: String,
}

trait Draw {
    fn plot_line(&mut self, from: (u32, u32), to: (u32, u32), color: Luma<u8>);
}

impl Draw for image::GrayImage {
    fn plot_line(&mut self, from: (u32, u32), to: (u32, u32), color: Luma<u8>) {
        let (width, height) = self.dimensions();

        let mut x = from.0 as i32;
        let mut y = from.1 as i32;

        self.put_pixel(x as u32, y as u32, color);

        let delta_x = (to.0 as i32 - from.0 as i32).abs();
        let delta_y = (to.1 as i32 - from.1 as i32).abs();
        let step_x = if from.0 < to.0 { 1 } else { -1 };
        let step_y = if from.1 < to.1 { 1 } else { -1 };

        let mut error = if delta_x > delta_y { delta_x } else { -delta_y } / 2;

        while x != to.0 as i32 || y != to.1 as i32 {
            self.put_pixel(x as u32, y as u32, color);

            let error2 = error;

            if error2 > -delta_x {
                error -= delta_y;
                x += step_x;
            }

            if error2 < delta_y {
                error += delta_x;
                y += step_y;
            }
        }
    }
}

impl HitomezashiImage {
    pub fn new(message: String, key: String, pixel_size: u32) -> Result<HitomezashiImage> {
        if message.len() == 0 || key.len() == 0 {
            return Err(anyhow!("Message and key must not be empty."));
        }

        let image = image::GrayImage::new(
            key.len() as u32 * pixel_size,
            message.len() as u32 * pixel_size,
        );

        Ok(Self {
            image,
            line_length: pixel_size,
            message,
            key,
        })
    }

    pub fn stitch_image(self: &mut Self) -> Result<image::GrayImage> {
        self.stitch_message();
        self.stitch_key();
        self.image.save("img.png")?;
        Ok(self.image.clone())
    }

    fn stitch_message(self: &mut Self) {
        // Draw horizontal lines
        // If consonant no offset
        // If vowel offset by line_length 
        for (i, c) in self.message.chars().enumerate() {
            let y = i as u32 * self.line_length;

            if is_vowel(c) {
                self.image
                    .plot_line((self.line_length, y), (self.image.width(), y), Luma([0u8]));
            } else {
                self.image
                    .plot_line((0, y), (self.image.width(), y), Luma([0u8]));
            }
        }
    }

    fn stitch_key(self: &mut Self) {
        // Draw vertical lines
        // If even no offset
        // If odd offset by line_length
        for (i, c) in self.key.chars().enumerate() {
            let x = i as u32 * self.line_length;
            
            let digit_result = c.to_digit(10);
            let digit = match digit_result {
                Some(d) => d,
                None => continue,
            };

            if digit % 2 == 0 {
                self.image
                    .plot_line((x + self.line_length, 0), (x + self.line_length, self.image.height()), Luma([0u8]));
            } else {
                self.image
                    .plot_line((x + self.line_length, self.line_length), (x + self.line_length, self.image.height()), Luma([0u8]));
            }
        }
    }
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'i' | 'u' | 'e' | 'o' => true,
        _ => false,
    }
}

