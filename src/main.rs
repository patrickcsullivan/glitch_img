use clap::{arg, Parser};
use image::{io::Reader as ImageReader, GenericImage, Rgba};
use itertools::Itertools;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = Some("Inserts data from an arbitrary file into a rectangular sub-image within a larger image"))]
struct Args {
    /// Required path to the image to modify
    #[arg(short, long)]
    img: PathBuf,

    /// Required path to the file from which data will be copied
    #[arg(short, long)]
    data: PathBuf,

    /// Required path to the output
    #[arg(short, long)]
    out: PathBuf,

    /// Offset from the left (in pixels) of the sub-image in which to copy data
    #[arg(short, long)]
    x: u32,

    /// Offset from the top (in pixels) of the sub-image in which to copy data
    #[arg(short, long)]
    y: u32,

    /// Width (in pixels) of the sub-image box in which to copy data
    #[arg(long)]
    width: u32,

    /// Height (in pixels) of the sub-image in which to copy data
    #[arg(long)]
    height: u32,

    /// Offset into the data file of bytes to use
    #[arg(long, default_value_t = 0)]
    offset: u32,
}

fn main() {
    let args = Args::parse();

    let mut img = ImageReader::open(args.img)
        .expect("cannot open image file")
        .decode()
        .expect("cannot decode image file");
    let data: Vec<u8> = fs::read(args.data).expect("cannot open data file");
    let data = data.iter().dropping(args.offset as usize).tuples();

    if args.x + args.width > img.width() {
        panic!("sub-image x + width > image width")
    }

    if args.y + args.height > img.height() {
        panic!("sub-image y + height > image height")
    }

    let rows = args.y..args.y + args.height;
    let cols = args.x..args.x + args.width;

    for ((row, col), (&b1, &b2, &b3)) in rows.cartesian_product(cols).zip(data) {
        img.put_pixel(col, row, Rgba([b1, b2, b3, 255]))
    }

    img.save(args.out).expect("cannot write output file");
}
