use clap::Parser;
use cli_options::RunOptions;

mod basic_drawings;
mod cli_options;
mod falling_sand;
mod perlin_noise;
mod rainbow_sinewave;

fn main() {
    let args = cli_options::Args::parse();

    match args.name {
        RunOptions::PerlinNoise => perlin_noise::run(),
        RunOptions::AmplitudeNoise => rainbow_sinewave::run(),
        RunOptions::FallingSand => falling_sand::run(),
        _ => basic_drawings::run(),
    };
}
