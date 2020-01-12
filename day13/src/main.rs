use intcode_computer::Program;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Unknown,
}

impl Tile {
    fn from_id(id: usize) -> Tile {
        return match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            invalid_id => panic!("invalid Tile id {}", invalid_id),
        };
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Vec2 {
    x: usize,
    y: usize,
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Vec2 {
        return Vec2 { x, y };
    }
}

pub struct Arcade {
    program: Program,
    size: Vec2,
    tiles: BTreeMap<Vec2, Tile>,
    current_score: usize,
}

impl Arcade {
    pub fn new<'a>(code: impl IntoIterator<Item = &'a i64>) -> Arcade {
        return Arcade {
            program: Program::init(code),
            size: Vec2::new(0, 0),
            tiles: BTreeMap::new(),
            current_score: 0,
        };
    }

    pub fn get_tile(&self, at: &Vec2) -> Tile {
        return *self.tiles.get(at).unwrap_or(&Tile::Unknown);
    }

    pub fn set_tile(&mut self, at: Vec2, tile: Tile) {
        if at.x >= self.size.x {
            self.size.x = at.x + 1;
        }
        if at.y >= self.size.y {
            self.size.y = at.y + 1;
        }
        self.tiles.insert(at, tile);
    }

    pub fn get_paddle_direction(&self) -> i64 {
        let mut paddle_pos = None;
        let mut ball_pos = None;
        for (coordinate, tile_type) in self.tiles.iter() {
            match tile_type {
                Tile::Paddle => paddle_pos = Some(coordinate),
                Tile::Ball => ball_pos = Some(coordinate),
                _ => continue,
            }
        }
        assert_eq!(paddle_pos.is_some(), true);
        assert_eq!(ball_pos.is_some(), true);
        if paddle_pos.as_ref().unwrap().x < ball_pos.as_ref().unwrap().x {
            return 1;
        } else if paddle_pos.unwrap().x > ball_pos.unwrap().x {
            return -1;
        } else {
            return 0;
        }
    }

    /// returns tuple: .0 true when revisualization should be done, .1 true while program runs.
    pub fn run_one_frame(&mut self) -> (bool, bool) {
        let mut outputs = Vec::new();
        let mut redraw_visualization = false;
        loop {
            match self.program.next_opcode() {
                intcode_computer::Opcode::Terminate => return (redraw_visualization, false),
                intcode_computer::Opcode::Output => outputs.push(self.program.step(None).unwrap()),
                intcode_computer::Opcode::Input => {
                    assert_eq!(self.program.step(Some(self.get_paddle_direction())).is_none(), true);
                    redraw_visualization = true;
                }
                _ => assert_eq!(self.program.step(None).is_none(), true),
            };

            if outputs.len() == 3 {
                if outputs[0] == -1 && outputs[1] == 0 {
                    self.current_score = outputs[2] as usize;
                } else {
                    self.set_tile(
                        Vec2::new(outputs[0] as usize, outputs[1] as usize),
                        Tile::from_id(outputs[2] as usize),
                    );
                }
                return (redraw_visualization, true);
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let x = match self.program.run_until_output_or_terminate() {
                Some(x) => x,
                None => break, // breaks if program terminates
            };
            let y = self.program.run_until_output_or_terminate().unwrap();
            let id = self.program.run_until_output_or_terminate().unwrap();
            self.set_tile(
                Vec2::new(x as usize, y as usize),
                Tile::from_id(id as usize),
            );
        }
    }

    pub fn count_block_tiles(&self) -> usize {
        return self
            .tiles
            .iter()
            .filter(|tile| *tile.1 == Tile::Block)
            .count();
    }

    pub fn get_visualization(&self) -> String {
        let mut vis_string = String::from("");
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                vis_string += match self.get_tile(&Vec2::new(x, y)) {
                    Tile::Empty => " ",
                    Tile::Wall => "|",
                    Tile::Block => "b",
                    Tile::Paddle => "=",
                    Tile::Ball => "·",
                    Tile::Unknown => "?",
                };
            }
            vis_string += "\n";
        }
        vis_string += &format!("##### SCORE: {} #####\n", self.current_score);
        return vis_string;
    }
}

fn load_code_with_free_play() -> Vec<i64> {
    let mut code = load_code();
    code[0] = 2;
    return code;
}

fn load_code() -> Vec<i64> {
    return intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
}

fn main() {
    let mut arcade = Arcade::new(&load_code_with_free_play());
    let mut program_running = true;
    while program_running {
        let result = arcade.run_one_frame();
        let revisualize = result.0;
        program_running = result.1;
        if revisualize {
            println!("{}", arcade.get_visualization());
        }
    }
    println!("{}", arcade.get_visualization());
}

#[test]
fn part1_works() {
    let mut arcade = Arcade::new(&load_code());
    arcade.run();
    println!("arcade size after termination: {:?}", arcade.size);
    println!("{} unique tile coordinates added", arcade.tiles.len());
    assert_eq!(arcade.tiles.len(), 798);
    assert_eq!(arcade.size, Vec2::new(38, 21));
    assert_eq!(arcade.count_block_tiles(), 301);
}

#[test]
fn arcade_has_tiles_with_coordinates_and_type() {
    let mut arcade = Arcade::new([].to_vec());
    assert_eq!(arcade.size.x, 0);
    assert_eq!(arcade.size.y, 0);
    let test_coordinate = Vec2::new(43, 72);
    arcade.set_tile(test_coordinate.clone(), Tile::Ball);
    assert_eq!(arcade.size.x, 44);
    assert_eq!(arcade.size.y, 73);
    assert_eq!(arcade.get_tile(&test_coordinate), Tile::Ball);
    assert_eq!(arcade.get_tile(&Vec2::new(3, 2)), Tile::Unknown);
    assert_eq!(arcade.get_tile(&Vec2::new(33, 22)), Tile::Unknown);
}
