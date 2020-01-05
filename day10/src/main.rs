use std::collections::HashSet;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::convert::TryInto;
use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Space {
    Empty,
    Asteroid,
    Laser,
    Vaporized(usize),
}

impl Space {
    fn from_char(c: &char) -> Space {
        return match c {
            '.' => Space::Empty,
            '#' => Space::Asteroid,
            _ => panic!("invalid char {}", c),
        };
    }

    fn to_string(&self) -> String {
        return match self {
            Space::Empty => String::from(" . "),
            Space::Asteroid => String::from(" # "),
            Space::Laser => String::from(" X "),
            Space::Vaporized(n) => String::from(format!("{:03}", n)),
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

#[derive(PartialEq, Debug, Eq)]
struct Ray {
    normalized_direction: Direction,
    step_count: usize,
}

pub struct Map {
    asteroids: HashSet<Coordinate>,
    size: Coordinate,
    laser: Option<Coordinate>,
    vaporizations: Option<Vec<Coordinate>>,
}

impl Ray {
    fn get_target_when_shot_from(&self, shot_from: &Coordinate) -> Coordinate {
        return Coordinate {
            x: (shot_from.x as i32 + (self.normalized_direction.x * self.step_count as i32)).try_into().expect("Illegal coordinate"),
            y: (shot_from.y as i32 + (self.normalized_direction.y * self.step_count as i32)).try_into().expect("Illegal coordinate"),
        }
    }
}

impl Ord for Ray {
    /// first order by step_count, then by the associated direction
    fn cmp(&self, other: &Self) -> Ordering {
        match self.step_count.cmp(&other.step_count) {
            Ordering::Equal => return self.normalized_direction.cmp(&other.normalized_direction),
            x => return x,
        }
    }
}

impl PartialOrd for Ray {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Direction {
    /// order directions by angle. Smallest value is the direction straight up (towards y->inf, x=0), ascending clockwise.
    fn cmp(&self, other: &Self) -> Ordering {
        let atan_self = (self.x as f64).atan2(-self.y as f64);
        let atan_other = (other.x as f64).atan2(-other.y as f64);
        let angle_self = if atan_self >= 0. {atan_self} else {atan_self + std::f64::consts::PI * 2.0};
        let angle_other = if atan_other >= 0. {atan_other} else {atan_other + std::f64::consts::PI * 2.0};
        return angle_self.partial_cmp(&angle_other).unwrap();
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
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
            normalized_direction: normalized_vector_to_target.0,
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
            laser: None,
            vaporizations: None,
        };
    }

    pub fn to_string(&self) -> String {
        let mut map_string = String::new();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let current_coordinate = Coordinate { x, y };
                map_string += &self.at(&current_coordinate).to_string();
            }
            map_string.push('\n');
            map_string.push('\n');
        }
        map_string.pop(); // remove last \n
        return map_string;
    }

    pub fn at(&self, coordinate: &Coordinate) -> Space {
        if self.asteroids.contains(coordinate) {
            if self.laser.is_some() && self.laser.as_ref().unwrap() == coordinate {
                    return Space::Laser;
            } else if self.vaporizations.is_some() {
                for (nr, target) in self.vaporizations.as_ref().unwrap().iter().enumerate() {
                    if target == coordinate {
                        return Space::Vaporized(nr);
                    }
                }
                return Space::Vaporized(99999);
            } else {
                return Space::Asteroid;
            }
        } else {
            return Space::Empty;
        }
    }

    /// initializes the laser at the position with most visible asteroids and returns the number of visible asteroids
    pub fn setup_laser(&mut self) -> usize {
        let mut max_count = 0;
        for asteroid in self.asteroids.iter() {
            let count = self.count_visible_asteroids(&asteroid);
            if count > max_count {
                max_count = count;
                self.laser = Some(asteroid.clone());
            }
        }
        return max_count;
    }

    fn count_visible_asteroids(&self, at: &Coordinate) -> usize {
        let mut unique_ray_directions = HashSet::new();
        for asteroid in self.asteroids.iter() {
            // don't include a ray to itself
            if asteroid != at {
                let ray = at.ray_to(&asteroid);
                unique_ray_directions.insert(ray.normalized_direction.norm());
            }
        }
        return unique_ray_directions.len();
    }

    /// vaporizes all asteroids
    pub fn vaporize(&mut self) {
        let laser = self.laser.as_ref().unwrap();
        let mut laser_targets = self.get_laser_targets();
        let mut finished_directions = HashSet::new();
        let mut vaporization_order = Vec::new();
        while finished_directions.len() < laser_targets.len() {
            for (direction, targets) in laser_targets.iter_mut() {
                if finished_directions.contains(direction) {
                    continue;
                }
                let next_target = targets.pop().unwrap();
                vaporization_order.push(next_target.get_target_when_shot_from(laser));
                if targets.len() == 0 {
                    finished_directions.insert(direction.clone());
                }
            }
        }
        self.vaporizations = Some(vaporization_order);
    }

    fn get_laser_targets(&self) -> BTreeMap<Direction,Vec<Ray>> {
        let laser = self.laser.as_ref().expect("can't get laser targets before setting up laser");
        let mut laser_targets: BTreeMap<Direction,Vec<Ray>> = BTreeMap::new();
        for asteroid in self.asteroids.iter() {
            if asteroid != laser {
                let laser_ray = laser.ray_to(asteroid);
                let laser_direction = &laser_ray.normalized_direction;
                if !laser_targets.contains_key(laser_direction) {
                    laser_targets.insert(laser_ray.normalized_direction.clone(), Vec::new());
                }
                laser_targets.get_mut(laser_direction).unwrap().push(laser_ray);
            }
        }
        for (_direction, targets) in laser_targets.iter_mut() {
            targets.sort();
            targets.reverse();
        }
        return laser_targets;
    }
}

fn main() {
    let map_string = read_to_string("input.txt").unwrap();
    let mut map = Map::from_str(&map_string);
    let visible_asteroids_count = map.setup_laser();
    let best_asteroid = map.laser.clone().unwrap();
    println!(
        "Setup laser at asteroid {:?} (it can directly observe {} other asteroids).",
        best_asteroid, visible_asteroids_count
    );
    map.vaporize();
    let target_200th = &map.vaporizations.as_ref().unwrap()[199];
    println!("Map\n{}", map.to_string());
    let result = target_200th.x * 100 + target_200th.y;
    println!("The 200th asteroid vaporized is at {x}, {y}, with 100*{x}+{y}={result}", x=target_200th.x, y=target_200th.y, result=result);
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

//#[test]
//fn map_to_string() {
//    let map_string = " .  #..#\n.....\n#####\n....#\n...##";
//    let map = Map::from_str(map_string);
//    println!("string:\n{}", map_string);
//    println!("map:\n{}", map.to_string());
//    assert_eq!(map.to_string(), map_string);
//}

#[test]
fn map_from_string_has_correct_size() {
    let map = Map::from_str(".#..#\n.....\n#####\n....#\n...##\n");
    assert_eq!(map.size, Coordinate { x: 5, y: 5 });
}

#[test]
fn map_from_string_has_coordinate_access() {
    let map = Map::from_str(".#..#\n.....\n#####\n....#\n...##\n");
    assert_eq!(map.at(&Coordinate { x: 0, y: 0 }), Space::Empty);
    assert_eq!(map.at(&Coordinate { x: 1, y: 0 }), Space::Asteroid);
    assert_eq!(map.at(&Coordinate { x: 0, y: 1 }), Space::Empty);
    assert_eq!(map.at(&Coordinate { x: 4, y: 4 }), Space::Asteroid);
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
    assert_eq!(empty_space.to_string(), " . ");
    assert_eq!(asteroid.to_string(), " # ");
}

#[test]
fn part_one_works() {
    let map_string = read_to_string("input.txt").unwrap();
    let mut map = Map::from_str(&map_string);
    println!("Got map\n{}", map.to_string());
    let visible_asteroids_count = map.setup_laser();
    println!(
        "Best asteroid at {:?} can observe {} other asteroids.",
        map.laser, visible_asteroids_count
    );
    assert_eq!(map.laser.unwrap(), Coordinate { x: 27, y: 19 });
    assert_eq!(visible_asteroids_count, 314);
}

#[test]
fn direction_ordering() {
    let deg000 = Direction {x: 0, y: -1};
    let deg045 = Direction {x: 5, y: -5};
    let deg090 = Direction {x: 1, y: 0};
    let deg135 = Direction {x: 3, y: 3};
    let deg180 = Direction {x: 0, y: 1};
    let deg225 = Direction {x: -329, y: 329};
    let deg270 = Direction {x: -1, y: 0};
    let deg315 = Direction {x: -718, y: -718};
    println!("deg000 < deg045: {}",deg000 < deg045);
    println!("deg045 < deg090: {}",deg045 < deg090);
    println!("deg090 < deg135: {}",deg090 < deg135);
    println!("deg135 < deg180: {}",deg135 < deg180);
    println!("deg180 < deg225: {}",deg180 < deg225);
    println!("deg225 < deg270: {}",deg225 < deg270);
    println!("deg270 < deg315: {}",deg270 < deg315);
    println!("deg000 > deg045: {}",deg000 > deg045);
    println!("deg000 > deg090: {}",deg000 > deg090);
    println!("deg000 > deg135: {}",deg000 > deg135);
    println!("deg000 > deg180: {}",deg000 > deg180);
    println!("deg000 > deg225: {}",deg000 > deg225);
    println!("deg000 > deg270: {}",deg000 > deg270);
    println!("deg000 > deg315: {}",deg000 > deg315);
    assert_eq!(deg000 < deg045, true, "deg0 < deg045");
    assert_eq!(deg045 < deg090, true);
    assert_eq!(deg090 < deg135, true);
    assert_eq!(deg135 < deg180, true);
    assert_eq!(deg180 < deg225, true);
    assert_eq!(deg225 < deg270, true);
    assert_eq!(deg270 < deg315, true);
    assert_eq!(deg000 > deg045, false);
    assert_eq!(deg000 > deg090, false);
    assert_eq!(deg000 > deg135, false);
    assert_eq!(deg000 > deg180, false);
    assert_eq!(deg000 > deg225, false);
    assert_eq!(deg000 > deg270, false);
    assert_eq!(deg000 > deg315, false);
}

#[test]
fn test_best_laser_position() {
    let mut map = Map::from_str(".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....#...###..\n..#.#.....#....##");
    map.setup_laser();
    assert_eq!(map.laser.unwrap(), Coordinate{x:8, y:3});
}

#[test]
fn test_best_laser_position_with_visibility_count() {
    let mut map = Map::from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##");
    let nr_visible_asteroids = map.setup_laser();
    assert_eq!(map.laser.unwrap(), Coordinate{x:11, y:13});
    assert_eq!(nr_visible_asteroids, 210);
}

#[test]
fn test_vaporization_order() {
    let mut map = Map::from_str(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##");
    map.setup_laser();
    map.vaporize();
    let vaporizations = map.vaporizations.as_ref().unwrap();
    assert_eq!(vaporizations[0], Coordinate{x: 11, y: 12});
    assert_eq!(vaporizations[1], Coordinate{x: 12, y: 1});
    assert_eq!(vaporizations[2], Coordinate{x: 12, y: 2});
    assert_eq!(vaporizations[9], Coordinate{x: 12, y: 8});
    assert_eq!(vaporizations[19], Coordinate{x: 16, y: 0});
    assert_eq!(vaporizations[49], Coordinate{x: 16, y: 9});
    assert_eq!(vaporizations[99], Coordinate{x: 10, y: 16});
    assert_eq!(vaporizations[198], Coordinate{x: 9, y: 6});
    assert_eq!(vaporizations[199], Coordinate{x: 8, y: 2});
    assert_eq!(vaporizations[200], Coordinate{x: 10, y: 9});
    assert_eq!(vaporizations[298], Coordinate{x: 11, y: 1});
}
