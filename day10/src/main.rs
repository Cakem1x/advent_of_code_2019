use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Space {
    Empty,
    Asteroid,
}

impl Space {
    fn from_char(c: &char) -> Space {
        return match c {
            '.' => Space::Empty,
            '#' => Space::Asteroid,
            _ => panic!("invalid char {}", c),
        };
    }

    fn to_char(&self) -> char {
        return match self {
            Space::Empty => '.',
            Space::Asteroid => '#',
        };
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
struct Direction {
    x: i32,
    y: i32,
}

struct Ray {
    start: Coordinate,
    direction: Direction,
    step_count: usize,
}

pub struct Map {
    asteroids: HashSet<Coordinate>,
    size: Coordinate,
}

impl Direction {
    /// Returns the normalized vector and a usize that contains the value with which normlization took place
    fn norm(&self) -> (Direction, usize) {
        let mut bigger_value = if self.x > self.y { self.x } else { self.y };
        let mut smaller_value = if self.x < self.y { self.x } else { self.y };
        while smaller_value != 0 {
            let temp = smaller_value;
            smaller_value = bigger_value % smaller_value;
            bigger_value = temp;
        }
        if bigger_value < 0 {
            bigger_value *= -1;
        }
        return (
            Direction {
                x: self.x / bigger_value,
                y: self.y / bigger_value,
            },
            bigger_value as usize,
        );
    }
}

impl Coordinate {
    fn ray_to(&self, target: &Coordinate) -> Ray {
        let vector_to_target = Direction {
            x: target.x as i32 - self.x as i32,
            y: target.y as i32 - self.y as i32,
        };
        let normalized_vector_to_target = vector_to_target.norm();
        Ray {
            start: self.clone(),
            direction: normalized_vector_to_target.0,
            step_count: normalized_vector_to_target.1,
        }
    }
}

impl Map {
    pub fn from_str(map_string: &str) -> Map {
        let mut asteroids = HashSet::new();
        let map_string_lines = map_string.split('\n');
        let mut size_y = 0;
        let mut size_x = 0;
        for (y, line) in map_string_lines.enumerate() {
            for (x, character) in line.chars().enumerate() {
                if Space::from_char(&character) == Space::Asteroid {
                    asteroids.insert(Coordinate { x, y });
                }
                if character != '\n' {
                    size_x += 1;
                }
            }
            if line != "" {
                size_y += 1;
            }
        }
        assert_eq!(size_x % size_y, 0, "x: {} mod y: {}", size_x, size_y); // map needs to be rectangular!
        size_x = size_x / size_y; // fix the fact that size_x is increased each y-loop
        return Map {
            asteroids,
            size: Coordinate {
                x: size_x,
                y: size_y,
            },
        };
    }

    pub fn to_string(&self) -> String {
        let mut map_string = String::new();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let current_coordinate = Coordinate { x, y };
                map_string.push(self.at(current_coordinate).to_char());
            }
            map_string.push('\n');
        }
        map_string.pop(); // remove last \n
        return map_string;
    }

    pub fn at(&self, coordinate: Coordinate) -> Space {
        return if self.asteroids.contains(&coordinate) {
            Space::Asteroid
        } else {
            Space::Empty
        };
    }

    pub fn get_best_asteroid(&self) -> (usize, Coordinate) {
        let mut max_count = 0;
        let mut max_count_asteroid = &Coordinate { x: 0, y: 0 };
        for asteroid in self.asteroids.iter() {
            let count = self.count_visible_asteroids(&asteroid);
            if count > max_count {
                max_count = count;
                max_count_asteroid = asteroid;
            }
        }
        return (max_count, max_count_asteroid.clone());
    }

    fn count_visible_asteroids(&self, at: &Coordinate) -> usize {
        let mut unique_ray_directions = HashSet::new();
        for asteroid in self.asteroids.iter() {
            // don't include a ray to itself
            if asteroid != at {
                let ray = at.ray_to(&asteroid);
                unique_ray_directions.insert(ray.direction.norm());
            }
        }
        return unique_ray_directions.len();
    }
}

fn main() {
    let map_string = read_to_string("input.txt").unwrap();
    let map = Map::from_str(&map_string);
    println!("Got map\n{}", map.to_string());
    let (visible_asteroids_count, best_asteroid) = map.get_best_asteroid();
    println!(
        "Best asteroid at {:?} can observe {} other asteroids.",
        best_asteroid, visible_asteroids_count
    );
}

#[test]
fn direction_normalization() {
    let ray_with_prime = Direction { x: 21, y: 5 };
    assert_eq!(ray_with_prime, ray_with_prime.norm().0);
    let ray_all_positive = Direction { x: 420, y: 10 };
    assert_eq!(ray_all_positive.norm().0, Direction { x: 42, y: 1 });
    let ray_all_positive_big_y = Direction { x: 42, y: 13818 };
    assert_eq!(ray_all_positive_big_y.norm().0, Direction { x: 1, y: 329 });
    let ray_all_positive_both_divisible = Direction { x: 34902, y: 13818 };
    assert_eq!(
        ray_all_positive_both_divisible.norm().0,
        Direction { x: 831, y: 329 }
    );
    let ray_partial_negative_x = Direction { x: -420, y: 10 };
    assert_eq!(ray_partial_negative_x.norm().0, Direction { x: -42, y: 1 });
    let ray_partial_negative_y = Direction { x: 420, y: -10 };
    assert_eq!(ray_partial_negative_y.norm().0, Direction { x: 42, y: -1 });
    let ray_all_negative = Direction { x: -420, y: -10 };
    assert_eq!(ray_all_negative.norm().0, Direction { x: -42, y: -1 });
}

#[test]
fn map_to_string() {
    let map_string = ".#..#\n.....\n#####\n....#\n...##";
    let map = Map::from_str(map_string);
    println!("string:\n{}", map_string);
    println!("map:\n{}", map.to_string());
    assert_eq!(map.to_string(), map_string);
}

#[test]
fn map_from_string_has_correct_size() {
    let map = Map::from_str(".#..#\n.....\n#####\n....#\n...##\n");
    assert_eq!(map.size, Coordinate { x: 5, y: 5 });
}

#[test]
fn map_from_string_has_coordinate_access() {
    let map = Map::from_str(".#..#\n.....\n#####\n....#\n...##\n");
    assert_eq!(map.at(Coordinate { x: 0, y: 0 }), Space::Empty);
    assert_eq!(map.at(Coordinate { x: 1, y: 0 }), Space::Asteroid);
    assert_eq!(map.at(Coordinate { x: 0, y: 1 }), Space::Empty);
    assert_eq!(map.at(Coordinate { x: 4, y: 4 }), Space::Asteroid);
}

#[test]
fn char_to_space_conversion() {
    let empty_space = '.';
    let asteroid = '#';
    assert_eq!(Space::from_char(&empty_space), Space::Empty);
    assert_eq!(Space::from_char(&asteroid), Space::Asteroid);
}

#[test]
fn space_to_char_conversion() {
    let empty_space = Space::Empty;
    let asteroid = Space::Asteroid;
    assert_eq!(empty_space.to_char(), '.');
    assert_eq!(asteroid.to_char(), '#');
}

#[test]
fn part_one_works() {
    let map_string = read_to_string("input.txt").unwrap();
    let map = Map::from_str(&map_string);
    println!("Got map\n{}", map.to_string());
    let (visible_asteroids_count, best_asteroid) = map.get_best_asteroid();
    println!(
        "Best asteroid at {:?} can observe {} other asteroids.",
        best_asteroid, visible_asteroids_count
    );
    assert_eq!(best_asteroid, Coordinate { x: 27, y: 19 });
    assert_eq!(visible_asteroids_count, 314);
}
