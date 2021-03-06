//
// This file is part of Takkerus.
//
// Takkerus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Takkerus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Takkerus. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright 2016 Chris Foster
//

use std::fmt::Display;

use tak::{Color, Direction, Piece, Player, Ply, State};

pub trait Ai : Player {
    fn analyze(&mut self, state: &State) -> Vec<Ply>;
    fn get_stats(&self) -> Box<Display>;
}

pub use self::minimax::MinimaxBot;

pub mod minimax;

lazy_static! {
    static ref SLIDE_TABLE: Vec<Vec<Vec<u8>>> = generate_slide_table(8);
}

pub trait Extrapolatable {
    fn get_possible_plies(&self) -> Vec<Ply>;
}

impl Extrapolatable for State {
    fn get_possible_plies(&self) -> Vec<Ply> {
        let mut plies = Vec::new();

        let next_color = if self.ply_count % 2 == 0 {
            Color::White
        } else {
            Color::Black
        };

        if self.ply_count >= 2 {
           for (x, column) in self.board.iter().enumerate() {
                for (y, stack) in column.iter().enumerate() {
                    if stack.is_empty() {
                        plies.push(Ply::Place {
                            x: x,
                            y: y,
                            piece: Piece::Flatstone(next_color),
                        });
                        plies.push(Ply::Place {
                            x: x,
                            y: y,
                            piece: Piece::StandingStone(next_color),
                        });

                        match next_color {
                            Color::White => if self.p1.capstone_count > 0 {
                                plies.push(Ply::Place {
                                    x: x,
                                    y: y,
                                    piece: Piece::Capstone(next_color),
                                });
                            },
                            Color::Black => if self.p2.capstone_count > 0 {
                                plies.push(Ply::Place {
                                    x: x,
                                    y: y,
                                    piece: Piece::Capstone(next_color),
                                });
                            },
                        }
                    } else if stack.last().unwrap().get_color() == next_color {
                        let board_size = self.board.len();
                        for &(direction, distance) in &[
                            (Direction::North, board_size - 1 - y),
                            (Direction::East,  board_size - 1 - x),
                            (Direction::South, y),
                            (Direction::West,  x),
                        ] {
                            let max_grab = if stack.len() <= board_size {
                                stack.len()
                            } else {
                                board_size
                            };

                            for drops in SLIDE_TABLE[max_grab].iter() {
                                if drops.len() <= distance {
                                    plies.push(Ply::Slide {
                                        x: x,
                                        y: y,
                                        direction: direction,
                                        drops: drops.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        } else {
            for (x, column) in self.board.iter().enumerate() {
                for (y, stack) in column.iter().enumerate() {
                    if stack.is_empty() {
                        plies.push(Ply::Place {
                            x: x,
                            y: y,
                            piece: Piece::Flatstone(next_color.flip()),
                        });
                    }
                }
            }
        }

        plies
    }
}

fn generate_slide_table(size: u8) -> Vec<Vec<Vec<u8>>> {
    let mut result: Vec<Vec<Vec<u8>>> = Vec::with_capacity(size as usize);
    result.push(Vec::new());

    for stack in 1..(size + 1) {
        let mut out = Vec::with_capacity((2 as usize).pow(stack as u32) - 1);

        for i in 1..(stack + 1) {
            out.push(vec![i]);

            for sub in &result[(stack - i) as usize] {
                let mut t = vec![0; sub.len() + 1];
                t[0] = i;
                t[1..].clone_from_slice(sub);

                out.push(t);
            }
        }

        result.push(out);
    }

    result
}
