use intcode_computer;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::fs::read_to_string;
use std::convert::TryInto;

#[derive(Debug, PartialEq, Copy, Clone)]
enum PanelColor {
    Black,
    White,
}

impl PanelColor {
    fn from_i32(value: i32) -> PanelColor {
        return match value {
            0 => PanelColor::Black,
            1 => PanelColor::White,
            _ => panic!("invalid panel color"),
        };
    }

    fn to_i32(&self) -> i32 {
        return match self {
            PanelColor::Black => 0,
            PanelColor::White => 1,
        };
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_coordinate(&self) -> Coordinate {
        return match self {
            Direction::Up => Coordinate { x: 0, y: 1 },
            Direction::Down => Coordinate { x: 0, y: -1 },
            Direction::Left => Coordinate { x: -1, y: 0 },
            Direction::Right => Coordinate { x: 1, y: 0 },
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

pub struct Robot {
    position: Coordinate,
    facing: Direction,
    program: intcode_computer::Program,
}

impl Robot {
    fn init() -> Robot {
        let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
        Robot {
            position: Coordinate { x: 0, y: 0 },
            facing: Direction::Up,
            program: intcode_computer::Program::init(&code),
        }
    }

    /// Rotates the robot clockwise and moves its position forward once.
    pub fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self.position += self.facing.to_coordinate();
    }

    /// Rotates the robot counter-clockwise and moves its position forward once.
    pub fn turn_left(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        };
        self.position += self.facing.to_coordinate();
    }

    /// returns false when the robot's program has terminated
    fn move_robot(&mut self) {
        let output = self.program.run_until_output_or_terminate();
        if output.is_some() {
            match output.unwrap() {
                0 => self.turn_left(),
                1 => self.turn_right(),
                x => panic!("invalid movement command received: {}", x),
            }
        }
    }

    /// returns false when the robot's program has terminated
    fn paint_color(&mut self, hull: &mut ShipHull) {
        let output = self.program.run_until_output_or_terminate();
        if output.is_some() {
            hull.paint(&self.position, PanelColor::from_i32(output.unwrap().try_into().unwrap()));
        }
    }

    fn measure_panel_color(&mut self, color: &PanelColor) {
        self.program.run_until_input(color.to_i32().into());
    }

    pub fn run_on_ship_hull(&mut self, hull: &mut ShipHull) {
        while !self.program.will_terminate() {
            self.measure_panel_color(&hull.get_color(&self.position));
            self.paint_color(hull);
            self.move_robot();
        }
        self.measure_panel_color(&hull.get_color(&self.position));
    }
}

pub struct ShipHull {
    panels: HashMap<Coordinate, PanelColor>,
}

impl ShipHull {
    fn init() -> ShipHull {
        return ShipHull {
            panels: HashMap::new(),
        };
    }

    fn get_color(&self, at: &Coordinate) -> PanelColor {
        if self.panels.contains_key(&at) {
            return self.panels[at];
        } else {
            return PanelColor::Black;
        }
    }

    fn paint(&mut self, at: &Coordinate, color: PanelColor) {
        self.panels.insert(at.clone(), color);
    }

    fn min_number_of_visited_panels(&self) -> usize {
        return self.panels.len();
    }
}

fn main() {
    let mut robot = Robot::init();
    let mut hull = ShipHull::init();
    robot.run_on_ship_hull(&mut hull);
    println!("{} visited panels.", hull.min_number_of_visited_panels());
}

#[test]
fn robot_starts_facing_up() {
    let robot = Robot::init();
    assert_eq!(robot.facing, Direction::Up);
}

#[test]
fn robot_moves_correctly() {
    let mut robot = Robot::init();
    robot.turn_left();
    assert_eq!(robot.facing, Direction::Left);
    assert_eq!(robot.position, Coordinate { x: -1, y: 0 });
    robot.turn_left();
    assert_eq!(robot.facing, Direction::Down);
    assert_eq!(robot.position, Coordinate { x: -1, y: -1 });
    robot.turn_left();
    assert_eq!(robot.facing, Direction::Right);
    assert_eq!(robot.position, Coordinate { x: 0, y: -1 });
    robot.turn_left();
    assert_eq!(robot.facing, Direction::Up);
    assert_eq!(robot.position, Coordinate { x: 0, y: 0 });
    robot.turn_right();
    assert_eq!(robot.facing, Direction::Right);
    assert_eq!(robot.position, Coordinate { x: 1, y: 0 });
}

#[test]
fn ship_hull_panel_painting() {
    let mut ship_hull = ShipHull::init();
    let painted_black_coordinate = Coordinate { x: 84, y: -511 };
    let painted_white_coordinate = Coordinate { x: -41, y: 21 };
    ship_hull.paint(&painted_black_coordinate, PanelColor::Black);
    ship_hull.paint(&painted_white_coordinate, PanelColor::White);
    assert_eq!(
        ship_hull.get_color(&Coordinate { x: 0, y: 1 }),
        PanelColor::Black
    );
    assert_eq!(
        ship_hull.get_color(&Coordinate { x: -319410, y: 1 }),
        PanelColor::Black
    );
    assert_eq!(
        ship_hull.get_color(&Coordinate {
            x: 0818,
            y: 18319481
        }),
        PanelColor::Black
    );
    assert_eq!(
        ship_hull.get_color(&painted_black_coordinate),
        PanelColor::Black
    );
    assert_eq!(
        ship_hull.get_color(&painted_white_coordinate),
        PanelColor::White
    );
}

#[test]
fn ship_hull_panels_access() {
    let ship_hull = ShipHull::init();
    assert_eq!(
        ship_hull.get_color(&Coordinate { x: 0, y: 1 }),
        PanelColor::Black
    );
    assert_eq!(
        ship_hull.get_color(&Coordinate { x: -319410, y: 1 }),
        PanelColor::Black
    );
    assert_eq!(
        ship_hull.get_color(&Coordinate {
            x: 0818,
            y: 18319481
        }),
        PanelColor::Black
    );
}
