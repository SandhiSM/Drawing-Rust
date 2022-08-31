use image::{ImageBuffer, Rgba, RgbaImage};
use std::path;
mod drawing;
#[derive(Clone)]
struct Image {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

fn main() {
    let title = input("Enter image title > ");
    let file_format = input("Enter file format (available -> ['PNG', 'JPEG']) > ");
    let x = input("Enter size of X > ");
    let y = input("Enter size of Y > ");
    if null_checker(&title) && content_checker(&file_format) {
        let (x, resx) = type_checker2(&x);
        let (y, resy) = type_checker2(&y);
        if resx && resy {
            let size = (x, y);
            let instance = drawing::Draw::new(title, file_format, size);
            let mut image = Image::new(x, y);
            Image::mode_branch(&mut image, instance);
        } else {
            println!("Something is not written or valid.");
        }
    } else {
        println!("Something is not written or valid.");
    }
}
fn input(console: &str) -> String {
    println!("{}", console);
    let mut standard_input: String = String::new();
    std::io::stdin()
        .read_line(&mut standard_input)
        .expect("Failed to read standard input.");
    standard_input = standard_input.replace("\r\n", "");
    standard_input
}
fn type_checker(check: &String) -> (u32, bool) {
    let (result, sf) = match check.parse::<u32>() {
        Ok(v) => (v, true),
        Err(_) => (0, false),
    };
    (result, sf)
}
fn type_checker2(check: &String) -> (u32, bool) {
    let (res, sf) = type_checker(check);
    if sf {
        if res > 0 {
            (res, true)
        } else {
            (0, false)
        }
    } else {
        (0, false)
    }
}
fn null_checker(check: &String) -> bool {
    if check == "" {
        false
    } else {
        true
    }
}
fn content_checker(check: &String) -> bool {
    let check = check.to_lowercase();
    if check == "jpeg" || check == "png" {
        true
    } else {
        false
    }
}
fn range_checker(color: u32) -> bool {
    if color <= 255 {
        true
    } else {
        false
    }
}
fn exist_checker(file_name: &String) -> bool {
    let mut res = false;
    let folder_path = path::PathBuf::from(r".\output\save");
    for file in folder_path
        .read_dir()
        .expect("Failed to access save folder.")
    {
        let path = file.unwrap().path();
        let filename = path
            .file_name()
            .unwrap()
            .to_str()
            .expect("Failed to get file name.");
        if filename == file_name {
            res = true;
        }
    }
    res
}
impl Image {
    fn new(horizontal: u32, vertical: u32) -> Self {
        let image = RgbaImage::new(horizontal, vertical);
        Image { image: image }
    }
    fn mode_branch(&mut self, mut instance: drawing::Draw) {
        loop {
            let mode = input("Enter mode (available -> ['load', 'pixelcolor', 'blockcolor', 'scaling', 'show', 'showcolorsample', 'showlist, 'temporarysave', 'save(Type 'save -e' to set to 'ediable')', 'exit']) > ");
            match &*mode {
                "load" => {
                    let name = input("Enter image title to load > ");
                    let name = format!("{}.json", name);
                    if exist_checker(&name) {
                        instance = drawing::Draw::load(&instance, &name);
                    } else {
                        println!("No such file exists.");
                        break;
                    }
                }
                "pixelcolor" => {
                    let mut scope2: Vec<(u32, u32)> = Vec::new();
                    let mut scope4: Vec<(u32, u32, u32, u32)> = Vec::new();
                    loop {
                        let x1 = input("Position of X from (If there are no more or you want to exit, leave this blank.) > ");
                        if x1 == "" {
                            if scope2.len() == 0 && scope4.len() == 0 {
                                break;
                            } else {
                                let r = input("Enter r of rgba > ");
                                let g = input("Enter g of rgba > ");
                                let b = input("Enter b of rgba > ");
                                let a = input("Enter a of rgba > ");
                                let (r, resr) = type_checker(&r);
                                let (g, resg) = type_checker(&g);
                                let (b, resb) = type_checker(&b);
                                let (a, resa) = type_checker(&a);
                                if resr
                                    && resg
                                    && resb
                                    && resa
                                    && range_checker(r)
                                    && range_checker(g)
                                    && range_checker(b)
                                    && range_checker(a)
                                {
                                    let rgba = [r as u8, g as u8, b as u8, a as u8];
                                    let definitive_scope2 = scope2.clone();
                                    let definitive_scope4 = scope4.clone();
                                    if scope2.len() != 0 && scope4.len() != 0 {
                                        let verification_image = instance.clone();
                                        if drawing::Draw::scope_checker2(
                                            &verification_image,
                                            &definitive_scope2,
                                        ) && drawing::Draw::scope_checker4(
                                            &verification_image,
                                            &definitive_scope4,
                                        ) {
                                            drawing::Draw::pixel_color2(
                                                &mut instance,
                                                &definitive_scope2,
                                                rgba,
                                                &mut self.image,
                                            );
                                            drawing::Draw::pixel_color4(
                                                &mut instance,
                                                &definitive_scope4,
                                                rgba,
                                                &mut self.image,
                                            );
                                        } else {
                                            println!("The range of x or y is not valid.");
                                            break;
                                        }
                                    } else if scope2.len() != 0 {
                                        let verification_image = instance.clone();
                                        if drawing::Draw::scope_checker2(
                                            &verification_image,
                                            &definitive_scope2,
                                        ) {
                                            drawing::Draw::pixel_color2(
                                                &mut instance,
                                                &definitive_scope2,
                                                rgba,
                                                &mut self.image,
                                            );
                                        } else {
                                            println!("The range of x or y is not valid.");
                                            break;
                                        }
                                    } else {
                                        let verification_image = instance.clone();
                                        if drawing::Draw::scope_checker4(
                                            &verification_image,
                                            &definitive_scope4,
                                        ) {
                                            drawing::Draw::pixel_color4(
                                                &mut instance,
                                                &definitive_scope4,
                                                rgba,
                                                &mut self.image,
                                            );
                                        } else {
                                            println!("The range of x or y is not valid.");
                                            break;
                                        }
                                    }
                                    scope2.clear();
                                    scope4.clear();
                                }
                            }
                        } else {
                            let y1 = input("Position of Y from > ");
                            let x2 = input(
                                "Position of X to (If you don't specify a range, leave this blank.) > ",
                            );
                            if x2 == "" {
                                let (x, resx) = type_checker2(&x1);
                                let (y, resy) = type_checker2(&y1);
                                if resx && resy {
                                    scope2.push((x, y));
                                } else {
                                    println!("Something is not written or valid.");
                                    break;
                                }
                            } else {
                                let y2 = input("Position of Y to > ");
                                let (x1, resx1) = type_checker2(&x1);
                                let (y1, resy1) = type_checker2(&y1);
                                let (x2, resx2) = type_checker2(&x2);
                                let (y2, resy2) = type_checker2(&y2);
                                if resx1 && resy1 && resx2 && resy2 {
                                    if y1 <= y2 {
                                        scope4.push((x1, y1, x2, y2));
                                    } else {
                                        println!("The first y value entered must be less than or equal to the y value entered later.");
                                        break;
                                    }
                                } else {
                                    println!("Something is not written or valid.");
                                    break;
                                }
                            }
                        }
                    }
                }
                "blockcolor" => {
                    let mut scope2: Vec<(u32, u32)> = Vec::new();
                    let mut scope4: Vec<(u32, u32, u32, u32)> = Vec::new();
                    let x = input("Enter the width of one block > ");
                    let y = input("Enter the height of one block > ");
                    let (x, resx) = type_checker2(&x);
                    let (y, resy) = type_checker2(&y);
                    if resx && resy {
                        let block = (x, y);
                        let verification_image = instance.clone();
                        if drawing::Draw::division_checker(&verification_image, block) {
                            loop {
                                let x1 = input("Block position of X from (If there are no more or you want to exit, leave blank.) > ");
                                if x1 == "" {
                                    if scope2.len() == 0 && scope4.len() == 0 {
                                        break;
                                    } else {
                                        let r = input("Enter r of rgba > ");
                                        let g = input("Enter g of rgba > ");
                                        let b = input("Enter b of rgba > ");
                                        let a = input("Enter a of rgba > ");
                                        let (r, resr) = type_checker(&r);
                                        let (g, resg) = type_checker(&g);
                                        let (b, resb) = type_checker(&b);
                                        let (a, resa) = type_checker(&a);
                                        if resr
                                            && resg
                                            && resb
                                            && resa
                                            && range_checker(r)
                                            && range_checker(g)
                                            && range_checker(b)
                                            && range_checker(a)
                                        {
                                            let rgba = [r as u8, g as u8, b as u8, a as u8];
                                            let definitive_scope2 = scope2.clone();
                                            let definitive_scope4 = scope4.clone();
                                            let verification_image = instance.clone();
                                            if scope2.len() != 0 && scope4.len() != 0 {
                                                if drawing::Draw::scope_checker2b(
                                                    &verification_image,
                                                    block,
                                                    &definitive_scope2,
                                                ) && drawing::Draw::scope_checker4b(
                                                    &verification_image,
                                                    block,
                                                    &definitive_scope4,
                                                ) {
                                                    drawing::Draw::block_color2(
                                                        &mut instance,
                                                        block,
                                                        &definitive_scope2,
                                                        rgba,
                                                        &mut self.image,
                                                    );
                                                    drawing::Draw::block_color4(
                                                        &mut instance,
                                                        block,
                                                        &definitive_scope4,
                                                        rgba,
                                                        &mut self.image,
                                                    );
                                                } else {
                                                    println!("The range of x or y is not valid.");
                                                    break;
                                                }
                                            } else if scope2.len() != 0 {
                                                if drawing::Draw::scope_checker2b(
                                                    &verification_image,
                                                    block,
                                                    &definitive_scope2,
                                                ) {
                                                    drawing::Draw::block_color2(
                                                        &mut instance,
                                                        block,
                                                        &definitive_scope2,
                                                        rgba,
                                                        &mut self.image,
                                                    );
                                                } else {
                                                    println!("The range of x or y is not valid.");
                                                    break;
                                                }
                                            } else {
                                                if drawing::Draw::scope_checker4b(
                                                    &verification_image,
                                                    block,
                                                    &definitive_scope4,
                                                ) {
                                                    drawing::Draw::block_color4(
                                                        &mut instance,
                                                        block,
                                                        &definitive_scope4,
                                                        rgba,
                                                        &mut self.image,
                                                    );
                                                } else {
                                                    println!("The range of x or y is not valid.");
                                                    break;
                                                }
                                            }
                                            scope2.clear();
                                            scope4.clear();
                                        }
                                    }
                                } else {
                                    let y1 = input("Block position of Y from > ");
                                    let x2 = input("Block position of X to (If you don't specify a range, leave this blank.) > ");
                                    if x2 == "" {
                                        let (x, resx) = type_checker2(&x1);
                                        let (y, resy) = type_checker2(&y1);
                                        if resx && resy {
                                            scope2.push((x, y));
                                        } else {
                                            println!("Something is not written or valid.");
                                            break;
                                        }
                                    } else {
                                        let y2 = input("Block position of Y to > ");
                                        let (x1, resx1) = type_checker2(&x1);
                                        let (y1, resy1) = type_checker2(&y1);
                                        let (x2, resx2) = type_checker2(&x2);
                                        let (y2, resy2) = type_checker2(&y2);
                                        if resx1 && resy1 && resx2 && resy2 {
                                            if y1 <= y2 {
                                                scope4.push((x1, y1, x2, y2));
                                            } else {
                                                println!("The first y value entered must be less than or equal to the y value entered later.");
                                                break;
                                            }
                                        } else {
                                            println!("Something is not written or valid.");
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("The width and height of the block must be an integral number of the overall size.");
                        }
                    } else {
                        println!("Something is not written or valid.");
                    }
                }
                "scaling" => {
                    let x = input("Enter the x value to resize > ");
                    let y = input("Enter the y value to resize > ");
                    let (x, resx) = type_checker2(&x);
                    let (y, resy) = type_checker2(&y);
                    if resx && resy {
                        let size = (x, y);
                        self.image = drawing::Draw::scaling(&mut instance, size, &self.image);
                        instance = drawing::Draw::new(instance.title, instance.file_format, size);
                    } else {
                        println!("Something is not written or valid.");
                    }
                }
                "show" => drawing::Draw::confirm(&mut instance, &self.image),
                "showcolorsample" => {
                    let r = input("Enter r of rgba > ");
                    let g = input("Enter g of rgba > ");
                    let b = input("Enter b of rgba > ");
                    let a = input("Enter a of rgba > ");
                    let (r, resr) = type_checker(&r);
                    let (g, resg) = type_checker(&g);
                    let (b, resb) = type_checker(&b);
                    let (a, resa) = type_checker(&a);
                    if resr
                        && resg
                        && resb
                        && resa
                        && range_checker(r)
                        && range_checker(g)
                        && range_checker(b)
                        && range_checker(a)
                    {
                        let rgba = [r as u8, g as u8, b as u8, a as u8];
                        drawing::Draw::confirm_color_sample(&instance, rgba)
                    }
                }
                "showlist" => drawing::Draw::show_list(&instance),
                "temporarysave" => drawing::Draw::temporary_save(&instance),
                "save" => drawing::Draw::save(&mut instance, &self.image, false),
                "save -e" => drawing::Draw::save(&mut instance, &self.image, true),
                "exit" => break,
                _ => println!("Invalid syntax."),
            }
        }
    }
}
