use image::{self, DynamicImage, ImageBuffer, ImageFormat, Luma};

#[derive(Debug, PartialEq)]
pub struct Logoboy {
    rom: Vec<u8>,
}

impl Logoboy {
    pub fn new(rom: Vec<u8>) -> Result<Logoboy, &'static str> {
        if rom.len() < 0x014F {
            return Err("ROM too small");
        }

        Ok(Logoboy { rom })
    }

    pub fn get_rom(&mut self) -> Result<Vec<u8>, &'static str> {
        Ok((&self.rom[..]).to_vec())
    }

    pub fn get_logo_raw(&mut self) -> Result<Vec<u8>, &'static str> {
        Ok((&self.rom[0x0104..0x0134]).to_vec())
    }

    pub fn get_logo(&mut self) -> Result<Vec<u8>, &'static str> {
        let mut pixels: Vec<u8> = vec![0x00; 48 * 8];
        let raw = match self.get_logo_raw() {
            Ok(raw) => raw,
            Err(e) => return Err(e),
        };
        let nibbles: Vec<u8> = raw
            .iter()
            .map(|byte| vec![(byte >> 4) & 0x0F, byte & 0x0F])
            .flatten()
            .collect();
        let offsets: Vec<(usize, usize)> = (0..2)
            .map(|i| {
                (0..48)
                    .step_by(4)
                    .map(|x| ((i * 4)..((i * 4) + 4)).map(move |y| (x, y)))
                    .flatten()
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect();

        nibbles
            .iter()
            .zip(offsets.iter())
            .for_each(|(nibble, (x, y))| {
                (0..4).for_each(|xs| {
                    let i = (x + (3 - xs)) + (y * 48);
                    let pixel = (nibble >> xs) & 0x01;

                    pixels[i] = pixel * 255;
                });
            });

        let img = ImageBuffer::from_fn(48, 8, |x, y| {
            let i = x + (y * 48);

            Luma([pixels[i as usize]])
        });
        let img = DynamicImage::ImageLuma8(img);

        let mut buf = Vec::new();

        match img.write_to(&mut buf, ImageFormat::Png) {
            Ok(_) => {}
            Err(_) => return Err("Could not write image"),
        }

        Ok(buf)
    }

    pub fn set_logo_raw(&mut self, logo: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        if logo.len() < 48 {
            return Err("Logo too small");
        }

        let rom_a = &self.rom[..0x0104];
        let rom_b = &self.rom[0x0134..];

        self.rom = [rom_a, &logo[..48], rom_b].concat();

        self.get_logo_raw()
    }

    pub fn set_logo(&mut self, bytes: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        let png = match image::load_from_memory_with_format(&bytes, ImageFormat::Png) {
            Ok(png) => png,
            Err(_) => return Err("Could not read image"),
        };

        let img = match png.as_luma8() {
            Some(img) => img,
            None => return Err("Image must be 48x8, 1-bit grayscale"),
        };

        let pixels: Vec<u8> = img
            .enumerate_pixels()
            .map(|(_, _, pixel)| {
                let pixel = pixel.0[0];

                if pixel != 0 {
                    return 1;
                }

                0
            })
            .collect();

        let nibbles: Vec<u8> = pixels
            .as_slice()
            .chunks(4)
            .map(|chunk| (0..4).fold(0, |acc, x| acc | chunk[x] << 3 - x))
            .collect();

        let offsets: Vec<u8> = [0, 48]
            .iter()
            .map(|y| {
                (0..4)
                    .map(move |x| ((x + y)..(48 + y)).step_by(4))
                    .flatten()
            })
            .flatten()
            .collect();

        let sorted_nibbles: Vec<u8> = (0..96)
            .map(|i| offsets.iter().position(|&r| r == i).unwrap())
            .map(|i| nibbles[i])
            .collect();

        let logo: Vec<u8> = (0..96)
            .step_by(2)
            .map(|i| (sorted_nibbles[i] << 4) | sorted_nibbles[i + 1])
            .collect();

        self.set_logo_raw(logo)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn new_valid() {
        let rom_a: Vec<u8> = vec![0x00; 0x0104];
        let logo: Vec<u8> = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        let rom_b: Vec<u8> = vec![0x00; 0x0150 - 0x0134];
        let rom = [&rom_a[..], &logo[..], &rom_b[..]].concat();

        Logoboy::new(rom).unwrap();
    }

    #[test]
    fn new_invalid() {
        let rom: Vec<u8> = vec![0x00; 0x01];

        match Logoboy::new(rom) {
            Err(e) => assert_eq!(e, "ROM too small"),
            _ => unreachable!(),
        };
    }

    #[test]
    fn get_logo_raw() {
        let rom_a: Vec<u8> = vec![0x00; 0x0104];
        let logo: Vec<u8> = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        let rom_b: Vec<u8> = vec![0x00; 0x0150 - 0x0134];
        let rom = [&rom_a[..], &logo[..], &rom_b[..]].concat();

        let mut logoboy = Logoboy::new(rom).unwrap();

        let rom_logo = logoboy.get_logo_raw().unwrap();

        assert_eq!(logo, rom_logo);
    }

    #[test]
    fn get_logo() {
        let rom_a: Vec<u8> = vec![0x00; 0x0104];
        let logo: Vec<u8> = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        let rom_b: Vec<u8> = vec![0x00; 0x0150 - 0x0134];
        let rom = [&rom_a[..], &logo[..], &rom_b[..]].concat();

        let mut logoboy = Logoboy::new(rom).unwrap();

        let rom_logo = logoboy.get_logo().unwrap();
        let png: Vec<u8> = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x08, 0x08, 0x00, 0x00, 0x00,
            0x00, 0x9A, 0x40, 0x23, 0x9B, 0x00, 0x00, 0x00, 0x66, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9C, 0x7D, 0xCB, 0x89, 0x0D, 0xC0, 0x30, 0x0C, 0x02, 0xC0, 0x64, 0xFF, 0xA1, 0xD3,
            0x23, 0xBF, 0x5A, 0xA9, 0x08, 0x1B, 0xF0, 0x53, 0x5B, 0xA9, 0xA5, 0x68, 0x58, 0x7E,
            0xB0, 0xD7, 0x0E, 0x59, 0x11, 0xC5, 0xE0, 0xB8, 0x0B, 0x7B, 0xE8, 0x90, 0xDD, 0x11,
            0x0C, 0xAE, 0xB4, 0xB0, 0x2F, 0x3C, 0xD4, 0xD6, 0xDB, 0x62, 0xDF, 0x8D, 0x2F, 0x0E,
            0x73, 0x0D, 0x89, 0x7A, 0x26, 0xE7, 0x21, 0x35, 0x69, 0x4D, 0x52, 0xE1, 0x48, 0x97,
            0x74, 0x45, 0x36, 0x62, 0x00, 0x64, 0x0D, 0x3E, 0xD2, 0x15, 0xD9, 0x08, 0x7A, 0x48,
            0x48, 0xE1, 0x4B, 0xA6, 0xE2, 0x74, 0x04, 0xA6, 0x4B, 0xC1, 0x90, 0xFA, 0x00, 0xA1,
            0x63, 0x4D, 0x06, 0xB4, 0xD9, 0x32, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E,
            0x44, 0xAE, 0x42, 0x60, 0x82,
        ];

        assert_eq!(rom_logo, png);
    }

    #[test]
    fn set_logo_raw() {
        let rom_a: Vec<u8> = vec![0x00; 0x0104];
        let logo: Vec<u8> = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        let rom_b: Vec<u8> = vec![0x00; 0x0150 - 0x0134];
        let rom = [&rom_a[..], &logo[..], &rom_b[..]].concat();

        let mut logoboy = Logoboy::new((&rom[..]).to_vec()).unwrap();

        let logo_raw: Vec<u8> = vec![
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
            0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
        ];

        let rom_logo = logoboy.set_logo_raw((&logo_raw[..]).to_vec()).unwrap();

        assert_eq!(rom_logo, logo_raw);
        assert_eq!(rom.len(), logoboy.rom.len());
    }

    #[test]
    fn set_logo_raw_err() {
        let rom_a: Vec<u8> = vec![0x00; 0x0104];
        let logo: Vec<u8> = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        let rom_b: Vec<u8> = vec![0x00; 0x0150 - 0x0134];
        let rom = [&rom_a[..], &logo[..], &rom_b[..]].concat();

        let mut logoboy = Logoboy::new(rom).unwrap();

        let logo_raw: Vec<u8> = vec![0x42];

        match logoboy.set_logo_raw(logo_raw) {
            Err(e) => assert_eq!(e, "Logo too small"),
            _ => unreachable!(),
        };
    }

    #[test]
    fn set_logo() {
        let rom_a: Vec<u8> = vec![0x00; 0x0104];
        let logo: Vec<u8> = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        let rom_b: Vec<u8> = vec![0x00; 0x0150 - 0x0134];
        let rom = [&rom_a[..], &logo[..], &rom_b[..]].concat();

        let mut logoboy = Logoboy::new(rom).unwrap();

        let logo_input: Vec<u8> = vec![
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x08, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x97, 0x50, 0x41, 0xea, 0x00, 0x00, 0x00, 0x3a, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x01, 0x63, 0xf8, 0x61, 0x79, 0xce, 0xf0, 0xc7, 0x7f, 0x86, 0xcf, 0xfb, 0xcf, 0x1b,
            0xff, 0x01, 0x52, 0x0f, 0x9b, 0x8d, 0x54, 0xe6, 0x33, 0x7c, 0xfe, 0x79, 0x9e, 0x59,
            0xc5, 0x9f, 0xe1, 0x93, 0xe4, 0x79, 0x63, 0x95, 0xf3, 0x0c, 0x9f, 0x2d, 0xa1, 0xd4,
            0x59, 0x63, 0xa0, 0xe0, 0x8f, 0x84, 0x24, 0xc3, 0x86, 0xf9, 0x00, 0x41, 0xd1, 0x1b,
            0x1e, 0xd9, 0xd5, 0xca, 0x4a, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae,
            0x42, 0x60, 0x82,
        ];

        logoboy.set_logo(logo_input).unwrap();

        let logo_output = logoboy.get_logo_raw().unwrap();
        let logo_raw: Vec<u8> = vec![
            0xFF, 0xFF, 0x83, 0x33, 0x3B, 0xEF, 0x9F, 0x19, 0xCC, 0x8C, 0xEF, 0x3F, 0x33, 0x30,
            0x13, 0x23, 0xFF, 0x22, 0x8C, 0x44, 0xFF, 0x94, 0xFF, 0xFF, 0xFF, 0xFF, 0x23, 0x38,
            0x13, 0x36, 0x99, 0x90, 0xCC, 0xC6, 0xFF, 0xD2, 0x33, 0x33, 0x33, 0x31, 0x22, 0x28,
            0x44, 0x40, 0xCC, 0x49, 0xFF, 0xFF,
        ];

        assert_eq!(logo_output, logo_raw);
    }
}
