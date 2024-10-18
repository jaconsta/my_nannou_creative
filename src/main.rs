use clap::Parser;
use cli_options::RunOptions;

mod basic_drawings;
mod cli_options;
mod falling_sand;
mod game_of_life;
mod perlin_noise;
mod rainbow_sinewave;
mod rgb_linear;

fn main() {
    let args = cli_options::Args::parse();

    match args.name {
        RunOptions::PerlinNoise => perlin_noise::run(),
        RunOptions::AmplitudeNoise => rainbow_sinewave::run(),
        RunOptions::FallingSand => falling_sand::run(),
        RunOptions::RgbLinearTransition => rgb_linear::run(),
        RunOptions::GameOfLife => game_of_life::run(),
        _ => basic_drawings::run(),
    };
}
