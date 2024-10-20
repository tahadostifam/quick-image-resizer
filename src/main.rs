use image2::{Error, Image, Rgb, Size};
use seahorse::{App, Command};
use std::env;

fn read_image(path: String) -> Result<Image<f64, Rgb>, Error> {
    Ok(Image::<f64, Rgb>::open(path)?)
}

fn resize_image(image: Image<f64, Rgb>, size: Size, output: String) -> Result<(), Error> {
    let resized_image = image.resize(size);
    resized_image.save(output).unwrap();

    Ok(())
}

fn resize_command() -> Command {
    Command::new("resize")
        .description("resize images to size specified")
        .alias("r")
        .usage("<image_path> <output_path> <size>")
        .action(|c| {
            if c.args.len() != 3 {
                eprintln!("Invalid number of arguments! See --help for more information.");
                std::process::exit(1);
            }

            let path = &c.args[0];
            let output: &String = &c.args[1];
            let size: Vec<&str> = c.args[2].split("x").collect();
            
            if size.len() != 2 {
                eprintln!("Invalid value specified for the size! Use it like 35x35 for example.");
                std::process::exit(1);
            }

            let width: usize = size[0].parse().unwrap();
            let height: usize = size[1].parse().unwrap();

            let image = read_image(path.clone()).unwrap();
            let size = Size::new(width, height);

            resize_image(image, size, output.clone()).unwrap();
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    App::new("Quick Image Resizer")
        .description("Resize your images quickly with this Rust program")
        .author("Taha Dostifam")
        .command(resize_command())
        .run(args);
}
