use clap::{App, Arg};
use rand::Rng;
use std::error::Error;

mod canvas;
mod emoji;

fn main() -> Result<(), Box<dyn Error>> {
    let m = parse_args().get_matches();

    let num_iterations = match m.value_of("num_iterations") {
        Some(num_str) => num_str.parse::<u32>()?,
        None => 500000,
    };
    let output = m.value_of("output").unwrap_or("canvas.png");
    let image_size = match m.value_of("image_size") {
        Some(image_size_str) => image_size_str.parse::<u32>()?,
        None => 20,
    };
    let scale = match m.value_of("scale") {
        Some(scale_str) => scale_str.parse::<u32>()?,
        None => 1,
    };
    let emoji_dir = m.value_of("emoji_dir").unwrap_or("emojis");

    let target_path = match m.value_of("target_path") {
        Some(target_path) => target_path,
        None => return Err("Please provide a path to the target image".into()),
    };
    let target = image::open(target_path).unwrap();
    let target_rgba = target.to_rgba();
    let (width, height) = target_rgba.dimensions();
    println!("Opened image {} {}x{}", target_path, width, height);

    println!("Generating new EmojiManager");
    let mut em = emoji::EmojiManager::new(emoji_dir)?;

    let canvas_width = width * scale;
    let canvas_height = height * scale;
    println!(
        "Generating new CanvasManager with dimensions {}x{}",
        canvas_width, canvas_height
    );
    let mut cm = canvas::CanvasManager::new(output, canvas_width, canvas_height)?;

    println!(
        "Placing emojis - size: {}, iterations: {}",
        image_size, num_iterations
    );

    let mut rng = rand::thread_rng();
    for _ in 0..num_iterations {
        let rand_x = rng.gen_range(0, width);
        let rand_y = rng.gen_range(0, height);
        let target_p = target_rgba.get_pixel(rand_x, rand_y);

        match em.get_nearest_emoji(*target_p, image_size) {
            Some(emoji) => cm.place_emoji(emoji, rand_x * scale, rand_y * scale),
            None => continue,
        }
    }

    println!("Saving output to file: {}", output);
    cm.save_canvas();

    Ok(())
}

fn parse_args<'a, 'b>() -> App<'a, 'b> {
    App::new("emvision")
        .author("trstruth")
        .version("0.1.0")
        .about("Generates an emoji vision image")
        .arg(Arg::with_name("target_path").required(true).index(1))
        .arg(
            Arg::with_name("num_iterations")
                .short("n")
                .long("num-iterations")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("emoji_dir")
                .short("e")
                .long("emoji-dir")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("image_size")
                .short("z")
                .long("image-size")
                .takes_value(true),
        )
}
