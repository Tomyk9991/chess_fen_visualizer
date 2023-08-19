use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use crate::sprite_provider::{Color, SpriteProvider};

pub enum Error {
    NotEnoughRows(usize),
    ColumnNotReach(String, usize),
    FigureNotFound(String)
}
#[derive(Debug, Clone)]
pub struct Fen {
    rows: Vec<String>
}

impl Fen {
    pub fn to_image(&self, sprite_provider: SpriteProvider) -> Result<DynamicImage, Error> {
        let mut result = sprite_provider.board();

        let (background_width, background_height) = result.dimensions();
        let (cell_width, cell_height) = (background_width / 8, background_height / 8);
        let mut term: String;

        let mut x_position = 0;
        let mut y_position = 0;

        for row in &self.rows {
            for char in row.chars() {
                if let Some(digit) = char.to_digit(10) {
                    for _ in 0..digit {
                        x_position += cell_width;
                    }

                    continue;
                }

                if let Some(figure) = if char.is_lowercase() { // black piece
                    term = SpriteProvider::term(&char, Color::Black);
                    sprite_provider.get_figure_ref(&char, Color::Black)
                } else {
                    term = SpriteProvider::term(&char.to_ascii_lowercase(), Color::White);
                    sprite_provider.get_figure_ref(&char.to_ascii_lowercase(), Color::White)
                } {
                    let (foreground_width, foreground_height) = figure.dimensions();

                    for x in 0..foreground_width {
                        for y in 0..foreground_height {
                            let fg_pixel = figure.get_pixel(x, y);

                            if fg_pixel[3] != 0 {
                                let mut result_pixel = result.get_pixel(x_position + x, y_position + y);
                                result_pixel.blend(&fg_pixel);
                                result.put_pixel(x_position + x, y_position + y, result_pixel);
                            }
                        }
                    }

                    x_position += cell_width;
                } else {
                    return Err(Error::FigureNotFound(term));
                }

            }

            y_position += cell_height;
            x_position = 0;
        }

        Ok(result)
    }
}

impl FromStr for Fen {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let rows = value.split('/').collect::<Vec<&str>>();
        if rows.len() != 8 {
            return Err(Error::NotEnoughRows(rows.len()))
        }

        for row in &rows {
            let figures = row.chars().filter(Fen::is_chess_figure).count();
            let spaces = row.chars().filter_map(|c| c.to_digit(10)).sum::<u32>() as usize;

            let column_size = figures + spaces;
            if column_size != 8 {
                return Err(Error::ColumnNotReach(row.to_string(), column_size));
            }
        }

        Ok(Self {
            rows: rows.iter().map(|a| a.to_string()).collect()
        })
    }
}

impl Fen {
    fn is_chess_figure(char: &char) -> bool {
        static VALID_CHARS: [char; 12] = ['r', 'n', 'b', 'q', 'k', 'p', 'R', 'N', 'B', 'Q', 'K', 'P'];
        VALID_CHARS.contains(char)
    }
}


impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::NotEnoughRows(actual_length) => format!("You have to put exactly 8 rows in your configuration, separated by a \"/\", but you provided just {actual_length}"),
            Error::ColumnNotReach(row, actual_length) => format!("The current line \"{row}\" does not reach the 8 necessary positionings, you provided just {actual_length}"),
            Error::FigureNotFound(figure) => format!("The figure with the signature {figure} wasn't found")
        })
    }
}