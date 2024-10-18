use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the creative to run
    #[arg(short, long, value_enum, default_value_t=RunOptions::InitialShapes)]
    pub name: RunOptions,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RunOptions {
    // "PERLIN_NOISE"
    PerlinNoise,
    // "AMPLITUDE_NOISE"
    AmplitudeNoise,
    // "FALLING_SAND"
    FallingSand,
    // "RGB_LINEAR_TRANSITION"
    RgbLinearTransition,
    // "KONWAYS_GAME_OF_LIFE"
    GameOfLife,
    // _
    InitialShapes,
}
