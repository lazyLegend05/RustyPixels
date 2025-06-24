pub fn image_to_ascii(path: &str) {
    use image::{GenericImageView, imageops::FilterType};
    use colored::Colorize;

    // Load image
    let img = image::open(path).expect("Failed to open image");

    // Resize and KEEP RGB (no .grayscale()!)
    let width = 150;
    let scale = 0.55;
    let height = (width as f32 * scale) as u32;
    let img = img.resize_exact(width, height, FilterType::Nearest);

    // Char palette
    let chars: Vec<char> = vec![
        '@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.', ' '
    ];

    // Loop pixels
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            // Convert to brightness (grayscale formula)
            let brightness = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
            let brightness = brightness / 255.0;
            let index = (brightness * (chars.len() - 1) as f32).round() as usize;

            // Print colored character
            print!("{}", chars[index].to_string().truecolor(r, g, b));
        }
        println!();
    }
}
