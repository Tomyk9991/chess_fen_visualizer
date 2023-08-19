use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::path::Path;

use image::{DynamicImage, ImageError};

pub enum Error {
    ImageError(ImageError),
    ImageNotFound(String),
    DirectoryNotFound(String),
}

pub struct SpriteProvider {
    board: DynamicImage,
    figures: HashMap<String, DynamicImage>,
}

pub enum Color {
    Black,
    White,
}

impl SpriteProvider {
    pub fn folder<P: AsRef<Path>>(path_ref: P) -> Result<Self, Error> {
        let path = path_ref.as_ref().display().to_string();

        if Path::new(&path).is_dir() {
            static PATHS: [&str; 13] = ["board", "bb", "bk", "bn", "bp", "bq", "br", "wb", "wk", "wn", "wp", "wq", "wr"];
            let mut figures: HashMap<String, DynamicImage> = HashMap::with_capacity(12);

            let board_path = format!("{p}/board.png", p = path.clone());

            let board = image::open(&board_path)
                .map_err(|_| Error::ImageNotFound(board_path))?;

            for file_name in PATHS {
                let file_path = format!("{p}/{}.png", file_name, p = path.clone());

                figures.insert(file_name.to_string(), image::open(&file_path)
                    .map_err(|_| Error::ImageNotFound(file_path))?);
            }

            return Ok(Self {
                board,
                figures,
            });
        }

        Err(Error::DirectoryNotFound(path))
    }

    pub fn get_figure_ref(&self, figure: &char, color: Color) -> &DynamicImage {
        let search_term = format!("{}{}", match color {
            Color::Black => "b",
            Color::White => "w"
        }, figure);

        // allowing it, since figures must be valid, otherwise ths sprite_provider structure is not constructed
        #[allow(clippy::unwrap_used)]
        self.figures.get(&search_term).unwrap()
    }

    pub fn board(&self) -> DynamicImage {
        return self.board.clone();
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::ImageError(e) => format!("{e}"),
            Error::DirectoryNotFound(path) => format!("Could not read the directory: {path}"),
            Error::ImageNotFound(path) => format!("Image not found: {path}")
        })
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Error::ImageError(value)
    }
}