extern crate cairo;
extern crate clap;

use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, BufRead};

use cairo::{Context, Format, ImageSurface};
use clap::{App, Arg};

#[derive(Debug, Copy, Clone)]
struct Options {
    canvas_height: i32,
    canvas_width: i32,
    dot_radius: f64,
    mp_filter_low: f64,
    spacing_angular: f64,
    spacing_radial: f64,
}

fn visualize(output_path: &str, options: &Options) {
    let Options {
        canvas_height,
        canvas_width,
        mp_filter_low,
        dot_radius,
        spacing_angular,
        spacing_radial,
    } = options;
    let surface = ImageSurface::create(Format::ARgb32, *canvas_width, *canvas_height)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);

    // paint canvas white
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint();
    context.set_source_rgb(0.0, 0.0, 0.0);

    // draw line chart
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let data = line.unwrap();
        let split: Vec<&str> = data.split(" ").collect();
        let candidate: f64 = split[0].parse::<f64>().unwrap();
        let mut mp: f64 = split[1].parse::<f64>().unwrap();

        if mp <= *mp_filter_low {
            mp = 0.0;
        }

        let n = candidate * spacing_angular;
        let r = n.sqrt();
        let t = 2.0 * PI * r;
        let x = (*canvas_width as f64 * 0.5) + (r * t.cos()) * spacing_radial;
        let y = (*canvas_height as f64 * 0.5) - (r * t.sin()) * spacing_radial;
        let s = mp * dot_radius;
        context.move_to(x, y);
        context.arc(x, y, s, 0.0, 2.0 * PI);
        context.fill();
    }
    context.stroke();

    let mut file = File::create(output_path).expect("Couldn't create file");
    surface
        .write_to_png(&mut file)
        .expect("Couldn't write to png");
}

pub fn main() {
    let app = App::new("mp-visualize")
        .about("Visualise mp values in prime spiral format")
        .arg(
            Arg::with_name("output_path")
                .help("Output file path")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("width")
                .help("canvas width")
                .takes_value(true)
                .short("w")
                .long("width"),
        )
        .arg(
            Arg::with_name("height")
                .help("canvas height")
                .takes_value(true)
                .short("i")
                .long("height"),
        )
        .arg(
            Arg::with_name("dot_radius")
                .help("multiplier for dot radius")
                .takes_value(true)
                .short("d")
                .long("dot-radius"),
        )
        .arg(
            Arg::with_name("mp_filter_low")
                .help("filter out mp values below this number")
                .takes_value(true)
                .short("f")
                .long("filter"),
        )
        .arg(
            Arg::with_name("spacing_angular")
                .help("alter spacing of lines radiating from the center")
                .takes_value(true)
                .short("a")
                .long("angular-spacing"),
        )
        .arg(
            Arg::with_name("spacing_radial")
                .help("alter spacing between rings of dots")
                .takes_value(true)
                .short("r")
                .long("radial-spacing"),
        );
    let matches = app.get_matches();
    let output_path = matches.value_of("output_path").unwrap();
    let options = Options {
        canvas_height: matches
            .value_of("height")
            .unwrap_or("600")
            .parse()
            .expect("Invalid integer for height"),
        canvas_width: matches
            .value_of("width")
            .unwrap_or("600")
            .parse()
            .expect("Invalid integer for width"),
        dot_radius: matches
            .value_of("dot_radius")
            .unwrap_or("1.0")
            .parse()
            .expect("Invalid float for dot radius"),
        mp_filter_low: matches
            .value_of("mp_filter_low")
            .unwrap_or("0.0")
            .parse()
            .expect("Invalid float for mp filter"),
        spacing_angular: matches
            .value_of("spacing_angular")
            .unwrap_or("1.0")
            .parse()
            .expect("Invalid float for angular spacing"),
        spacing_radial: matches
            .value_of("spacing_radial")
            .unwrap_or("5.0")
            .parse()
            .expect("Invalid float for radial spacing"),
    };
    visualize(output_path, &options);
}
