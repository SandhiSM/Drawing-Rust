use image::{self, imageops, ImageBuffer, Rgba, RgbaImage};

#[derive(Clone)]
pub struct Draw {
    title: String,
    file_format: String,
    size: (u32, u32),
    horizontal: u32,
    vertical: u32,
    picture: Vec<Vec<[u8; 4]>>,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}
impl Draw {
    pub fn new(title: String, file_format: String, size: (u32, u32)) -> Self {
        let (horizontal, vertical) = size;
        let picture = generate(horizontal, vertical);
        let image = RgbaImage::new(horizontal, vertical);
        fn generate(horizontal: u32, vertical: u32) -> Vec<Vec<[u8; 4]>> {
            let mut picture: Vec<Vec<[u8; 4]>> = Vec::new();
            for _ in 0..vertical {
                picture.push(vec![[255; 4]; horizontal as usize]);
            }
            picture
        }
        Draw {
            title: title,
            file_format: file_format,
            size: size,
            horizontal: horizontal,
            vertical: vertical,
            picture: picture,
            image: image,
        }
    }
    pub fn scope_checker2(&self, scopes: &Vec<(u32, u32)>) -> bool {
        let mut res = false;
        for scope in scopes {
            if scope.0 <= self.horizontal && scope.1 <= self.vertical {
                res = true;
            } else {
                res = false;
            }
        }
        res
    }
    pub fn scope_checker4(&self, scopes: &Vec<(u32, u32, u32, u32)>) -> bool {
        let mut res = false;
        for scope in scopes {
            if scope.0 <= self.horizontal
                && scope.1 <= self.vertical
                && scope.2 <= self.horizontal
                && scope.3 <= self.vertical
            {
                res = true;
            } else {
                res = false;
            }
        }
        res
    }
    pub fn scope_checker2b(&self, block: (u32, u32), scopes: &Vec<(u32, u32)>) -> bool {
        let mut res = false;
        let (width, height) = block;
        for scope in scopes {
            if scope.0 * width <= self.horizontal && scope.1 * height <= self.vertical {
                res = true;
            } else {
                res = false;
            }
        }
        res
    }
    pub fn scope_checker4b(&self, block: (u32, u32), scopes: &Vec<(u32, u32, u32, u32)>) -> bool {
        let mut res = false;
        let (width, height) = block;
        for scope in scopes {
            if scope.0 * width <= self.horizontal
                && scope.1 * height <= self.vertical
                && scope.2 * width <= self.horizontal
                && scope.3 * height <= self.vertical
            {
                res = true;
            } else {
                res = false;
            }
        }
        res
    }
    pub fn division_checker(&self, block: (u32, u32)) -> bool {
        let (x, y) = block;
        if self.horizontal % x == 0 && self.vertical % y == 0 {
            true
        } else {
            false
        }
    }
    pub fn pixel_color2(&mut self, scope: &Vec<(u32, u32)>, rgba: [u8; 4]) {
        for place in scope {
            let x = place.0 as usize;
            let y = place.1 as usize;
            self.picture[y - 1][x - 1] = rgba;
        }
        self.putpixel();
    }
    pub fn pixel_color4(&mut self, scope: &Vec<(u32, u32, u32, u32)>, rgba: [u8; 4]) {
        for place in scope {
            let x1 = place.0 as usize;
            let y1 = place.1 as usize;
            let x2 = place.2 as usize;
            let y2 = place.3 as usize;
            let mut x1 = x1 - 1;
            let mut y1 = y1 - 1;
            let x2 = x2 - 1;
            let y2 = y2 - 1;
            loop {
                if y1 == y2 {
                    if x1 != x2 {
                        self.picture[y1][x1] = rgba;
                    } else {
                        self.picture[y2][x2] = rgba;
                        break;
                    }
                } else {
                    if x1 <= self.horizontal as usize - 1 {
                        self.picture[y1][x1] = rgba;
                    } else {
                        y1 += 1;
                        x1 = 0;
                        continue;
                    }
                }
                x1 += 1;
            }
        }
        self.putpixel();
    }
    pub fn block_color2(&mut self, block: (u32, u32), scope: &Vec<(u32, u32)>, rgba: [u8; 4]) {
        let mut scopes: Vec<(u32, u32, u32, u32)> = Vec::new();
        let (width, height) = block;
        for place in scope {
            let (x, y) = place;
            let x1 = width * x - width + 1;
            let y1 = height * y - height + 1;
            let x2 = width * x;
            let y2_ = height * y + 1;
            for y2 in y1..y2_ {
                scopes.push((x1, y2, x2, y2));
            }
        }
        self.pixel_color4(&scopes, rgba);
    }
    pub fn block_color4(
        &mut self,
        block: (u32, u32),
        scope: &Vec<(u32, u32, u32, u32)>,
        rgba: [u8; 4],
    ) {
        let mut scopes: Vec<(u32, u32, u32, u32)> = Vec::new();
        let (width, height) = block;
        for place in scope {
            let (x1, y1, x2, y2) = place;
            let x1 = width * x1 - width + 1;
            let y1 = height * y1 - height + 1;
            let x2 = width * x2;
            let y2_ = height * y2 + 1;
            for y2 in y1..y2_ {
                scopes.push((x1, y2, x2, y2));
            }
        }
        self.pixel_color4(&scopes, rgba);
    }
    pub fn putpixel(&mut self) {
        for (line, y) in self.picture.iter().zip(0..self.vertical) {
            for (rgba, x) in line.iter().zip(0..self.horizontal) {
                let rgba = Rgba::<u8>(*rgba);
                self.image.put_pixel(x, y, rgba);
            }
        }
    }
    pub fn scaling(&mut self, size: (u32, u32)) {
        let (x, y) = size;
        let file_path = format!(r".\output\temp\{}.{}", self.title, self.file_format);
        self.image.save(&file_path).expect("Failed to save.");
        let mut unresized_image = image::open(&file_path).expect("Failed to save temp file.");
        std::fs::remove_file(file_path).expect("Failed to remove file.");
        let resized_image = imageops::resize(&mut unresized_image, x, y, imageops::Lanczos3);
        self.image = resized_image;
    }
    pub fn confirm(&mut self) {
        self.scaling(self.size);
        let file_name = format!(r".\output\confirm\{}.{}", self.title, self.file_format);
        self.image.save(file_name).expect("Failed to save.");
        println!(
            "To confirm, go to Explorer and check 'DRAWING-RUST/output/confirm/{}.{}' in Explorer.",
            self.title, self.file_format
        );
    }
    pub fn show_list(&self) {
        println!("{:?}", self.picture);
    }
    pub fn save(&mut self) {
        self.scaling(self.size);
        let file_name = format!(r".\output\completed\{}.{}", self.title, self.file_format);
        self.image.save(file_name).expect("Failed to save.");
        println!("Saved.");
    }
    pub fn confirm_color_sample(&self, rgba: [u8; 4]) {
        let mut sample = RgbaImage::new(512, 512);
        for (_, _, pixel) in sample.enumerate_pixels_mut() {
            *pixel = Rgba(rgba);
        }
        let file_name = format!(r".\output\confirm\{:?}.png", rgba);
        sample.save(file_name).expect("Failed to save.");
        println!("To confirm, go to Explorer and check 'DRAWING-RUST/output/confirm/{:?}.png' in Explorer.", rgba);
    }
}
