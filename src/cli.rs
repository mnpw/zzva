use clap::Parser;

/// zwei zero vier acht / 2048, the game
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Size of the board
    #[clap(short, long, value_parser, default_value_t = 4)]
    pub board: u8,

    /// Size of the winning tile
    #[clap(short, long, value_parser, default_value_t = 2048)]
    pub max: u16,
}
