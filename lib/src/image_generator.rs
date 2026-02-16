use image::{ImageBuffer, Rgba};
use crate::error::Result;
use rand::Rng;

/// Generates beautiful geometric artwork
pub struct GeometricArtGenerator {
    width: u32,
    height: u32,
    seed: u64,
}

impl GeometricArtGenerator {
    pub fn new(width: u32, height: u32, seed: u64) -> Self {
        Self { width, height, seed }
    }

    /// Generate a geometric pattern image
    pub fn generate(&self) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mut img = ImageBuffer::new(self.width, self.height);
        let mut _rng = rand::thread_rng();

        // Base color
        let base_hue = ((self.seed as f32) % 360.0) as f32;

        // Generate multiple geometric layers
        for y in 0..self.height {
            for x in 0..self.width {
                let fx = x as f32 / self.width as f32;
                let fy = y as f32 / self.height as f32;

                // Create geometric patterns from data
                let pattern = self.compute_pattern(fx, fy);
                let color = self.pattern_to_color(pattern, base_hue);

                img.put_pixel(x, y, color);
            }
        }

        Ok(img)
    }

    /// Generate image from a chunk of binary data
    pub fn generate_from_data(&self, data: &[u8]) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let mut img = ImageBuffer::new(self.width, self.height);

        // Use data to seed the pattern generation
        let data_seed = self.bytes_to_seed(data);

        for y in 0..self.height {
            for x in 0..self.width {
                let fx = x as f32 / self.width as f32;
                let fy = y as f32 / self.height as f32;
                let pixel_idx = ((y * self.width + x) as usize) % data.len();

                // Combine geometric pattern with actual data
                let pattern = self.compute_pattern_with_data(fx, fy, data[pixel_idx]);
                let color = self.pattern_to_color(pattern, data_seed);

                img.put_pixel(x, y, color);
            }
        }

        Ok(img)
    }

    fn compute_pattern(&self, x: f32, y: f32) -> f32 {
        // Create multiple overlapping geometric patterns
        let distance = ((x - 0.5).powi(2) + (y - 0.5).powi(2)).sqrt();
        let angle = y.atan2(x);

        // Concentric circles
        let circles = (distance * 10.0).sin();

        // Grid patterns
        let grid = ((x * 5.0).sin() * (y * 5.0).cos()).abs();

        // Spiral
        let spiral = ((distance * 20.0 + angle).sin()).abs();

        // Combine patterns
        (circles + grid + spiral) / 3.0
    }

    fn compute_pattern_with_data(&self, x: f32, y: f32, data_byte: u8) -> f32 {
        let base_pattern = self.compute_pattern(x, y);
        // Map 0..255 to the center of 256 buckets in [-1, 1]
        let data_influence = ((data_byte as f32 + 0.5) / 256.0) * 2.0 - 1.0; 

        // Increase data influence for better robustness (90% signal)
        base_pattern * 0.1 + data_influence * 0.9
    }

    fn pattern_to_color(&self, pattern: f32, _base_hue: f32) -> Rgba<u8> {
        // Normalize pattern to [0, 1]
        let normalized = ((pattern + 1.0) / 2.0).max(0.0).min(1.0);

        // Map to grayscale for maximum robustness across color space conversions
        let v = (normalized * 255.0).round() as u8;
        Rgba([v, v, v, 255])
    }

    fn hsl_to_rgb(&self, h: f32, s: f32, l: f32) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - ((h_prime % 2.0 - 1.0).abs()));

        let (r1, g1, b1) = match h_prime as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        let m = l - c / 2.0;
        let r = ((r1 + m) * 255.0).round() as u8;
        let g = ((g1 + m) * 255.0).round() as u8;
        let b = ((b1 + m) * 255.0).round() as u8;

        (r, g, b)
    }

    /// Decode data from an image
    pub fn decode_from_image(&self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>, chunk_size: usize) -> Result<Vec<u8>> {
        let mut data = vec![0u8; chunk_size];
        let mut accumulations = vec![0.0f32; chunk_size];
        let mut counts = vec![0u32; chunk_size];

        let base_hue = 0.0; // Consistently ignored in lightness-based extraction

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = img.get_pixel(x, y);
                let fx = x as f32 / self.width as f32;
                let fy = y as f32 / self.height as f32;
                let pixel_idx = ((y * self.width + x) as usize) % chunk_size;

                // Reverse color to pattern
                let pattern = self.color_to_pattern(pixel, base_hue);
                
                let base_pattern = self.compute_pattern(fx, fy);
                // pattern = base_pattern * 0.1 + data_influence * 0.9
                let data_influence = (pattern - base_pattern * 0.1) / 0.9;
                
                // data_influence = ((data_byte + 0.5) / 256.0) * 2.0 - 1.0
                // (data_influence + 1.0) / 2.0 = (data_byte + 0.5) / 256.0
                // data_byte = ((data_influence + 1.0) / 2.0 * 256.0) - 0.5
                let data_byte = (data_influence + 1.0) / 2.0 * 256.0 - 0.5;

                accumulations[pixel_idx] += data_byte;
                counts[pixel_idx] += 1;
            }
        }

        for i in 0..chunk_size {
            if counts[i] > 0 {
                // Rounding the average of 500+ pixels should be extremely robust
                data[i] = (accumulations[i] / counts[i] as f32).round().max(0.0).min(255.0) as u8;
            }
        }

        Ok(data)
    }

    fn color_to_pattern(&self, color: &Rgba<u8>, _base_hue: f32) -> f32 {
        let v = color[0] as f32 / 255.0;
        
        // v = normalized = (pattern + 1.0) / 2.0
        v * 2.0 - 1.0
    }

    fn bytes_to_seed(&self, data: &[u8]) -> f32 {
        let sum: u32 = data.iter().map(|&b| b as u32).sum();
        ((sum as f32) % 360.0) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let gen = GeometricArtGenerator::new(1920, 1080, 42);
        assert_eq!(gen.width, 1920);
        assert_eq!(gen.height, 1080);
    }

    #[test]
    fn test_pattern_computation() {
        let gen = GeometricArtGenerator::new(256, 256, 42);
        let pattern = gen.compute_pattern(0.5, 0.5);
        assert!(pattern.is_finite());
        assert!(pattern >= -2.0 && pattern <= 2.0);
    }
}
