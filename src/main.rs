use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use clap::Parser;
use crate::fen::Fen;
use crate::sprite_provider::SpriteProvider;

mod fen;
mod sprite_provider;
enum Error {
    IsNotFENConfig(fen::Error),
    IO,
    CouldNotReadImages(sprite_provider::Error)
}

/// Application to parse a Chess FEN Configuration to an image
/// For example. This is the starting position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Valid FEN configuration. Row separated by "/"
    #[arg(short, long)]
    configuration: String,
    /// Alternative output file path: Default is ./Output.png
    #[arg(short, long)]
    output: Option<String>
}

fn main() {
    // https://www.chess.com/terms/fen-chess
    match run() {
        Ok(_) => {}
        Err(err) => eprintln!("{err:?}")
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse();

    let sequence = Fen::from_str(&args.configuration)?;
    let sprite_provider = SpriteProvider::folder("./assets/")?;

    let image = sequence.to_image(sprite_provider)?;

    let output_path = if let Some(path) = args.output {
        path
    } else {
        String::from("./Output.png")
    };

    image.save(output_path).map_err(|_| Error::IO)?;
    Ok(())
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::IsNotFENConfig(v) => format!("{v:?}"),
            Error::CouldNotReadImages(e) => format!("Could not read the image, because: {e:?}"),
            Error::IO => "Error writing to file".to_string()
        })
    }
}

impl From<sprite_provider::Error> for Error {
    fn from(value: sprite_provider::Error) -> Self {
        Error::CouldNotReadImages(value)
    }
}

impl From<fen::Error> for Error {
    fn from(value: fen::Error) -> Self {
        Error::IsNotFENConfig(value)
    }
}
