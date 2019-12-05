use std::fs::read_to_string;

fn main() {
    println!("loading wires from file.");
    let input_string = read_to_string("input.txt").unwrap();
    let wire_strings: Vec<&str> = input_string.split("\n").collect();
    let wire1 = wire_from_string(&wire_strings[0]);
    let wire2 = wire_from_string(&wire_strings[1]);
    // day3 part1
    let closest_intersection =
        get_closest_intersection(&wire1, &wire2).expect("No interesction found!");
    println!(
        "Closest intersection point is {:?} with distance {}",
        closest_intersection.0, closest_intersection.1
    );
    // day3 part2
    let closest_intersection =
        get_closest_intersection_by_steps(&wire1, &wire2).expect("No interesction found!");
    println!(
        "Closest intersection point by steps is {:?} with nr of steps {}",
        closest_intersection.0, closest_intersection.1
    );
}

#[derive(Debug, PartialEq)]
struct GridLine(Point, Point);

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn manhattan_distance(p1: &Point, p2: &Point) -> u32 {
    let xdist = if p1.x > p2.x {
        (p1.x - p2.x) as u32
    } else {
        (p2.x - p1.x) as u32
    };
    let ydist = if p1.y > p2.y {
        (p1.y - p2.y) as u32
    } else {
        (p2.y - p1.y) as u32
    };
    return xdist + ydist;
}

impl GridLine {
    fn intersect(&self, other: &GridLine) -> Option<Point> {
        if self.is_horizontal() {
            if other.is_horizontal() {
                None
            } else if other.is_vertical() {
                return self.intersect_horizontal_on_vertical(other);
            } else {
                panic!("GridLine {:?} is not a grid line!", other);
            }
        } else if self.is_vertical() {
            if other.is_vertical() {
                None // lines are parallel
            } else if other.is_horizontal() {
                return other.intersect_horizontal_on_vertical(self);
            } else {
                panic!("GridLine {:?} is not a grid line!", other);
            }
        } else {
            panic!("GridLine {:?} is not a grid line!", self);
        }
    }

    fn intersect_horizontal_on_vertical(&self, other: &GridLine) -> Option<Point> {
        assert_eq!(self.is_horizontal(), true);
        assert_eq!(other.is_vertical(), true);
        if (self.0.x..self.1.x + 1).contains(&other.0.x)
            || (self.1.x..self.0.x + 1).contains(&other.0.x)
        {
            if (other.0.y..other.1.y + 1).contains(&self.0.y)
                || (other.1.y..other.0.y + 1).contains(&self.0.y)
            {
                return Some(Point {
                    x: other.0.x,
                    y: self.0.y,
                });
            }
        }
        None
    }

    fn is_horizontal(&self) -> bool {
        return self.0.y == self.1.y;
    }

    fn is_vertical(&self) -> bool {
        return self.0.x == self.1.x;
    }

    fn contains(&self, p: &Point) -> bool {
        if self.is_horizontal() {
            self.0.y == p.y
                && ((self.0.x..self.1.x + 1).contains(&p.x)
                    || (self.1.x..self.0.x + 1).contains(&p.x))
        } else if self.is_vertical() {
            self.0.x == p.x
                && ((self.0.y..self.1.y + 1).contains(&p.y)
                    || (self.1.y..self.0.y + 1).contains(&p.y))
        } else {
            panic!("{:?} is not a GridLine!", self);
        }
    }

    fn len(&self) -> u32 {
        return manhattan_distance(&self.0, &self.1);
    }
}

fn wire_from_string(string: &str) -> Vec<GridLine> {
    let mut wire: Vec<GridLine> = Vec::new();
    for direction_and_distance in string.split(",") {
        let last_point = match wire.last() {
            None => Point { x: 0, y: 0 }, // origin
            Some(grid_line) => grid_line.1.clone(),
        };
        let direction = &direction_and_distance[0..1];
        let distance: i32 = direction_and_distance[1..]
            .parse()
            .expect("Failed to get distance from string");
        let next_point = match direction {
            "R" => Point {
                x: last_point.x + distance,
                y: last_point.y,
            },
            "L" => Point {
                x: last_point.x - distance,
                y: last_point.y,
            },
            "U" => Point {
                x: last_point.x,
                y: last_point.y + distance,
            },
            "D" => Point {
                x: last_point.x,
                y: last_point.y - distance,
            },
            _ => panic!("unknown direction"),
        };
        let gl = GridLine(last_point, next_point);
        wire.push(gl);
    }
    return wire;
}

fn intersect_wires(wire1: &Vec<GridLine>, wire2: &Vec<GridLine>) -> Vec<Point> {
    let origin = Point { x: 0, y: 0 };
    let mut intersection_points = Vec::new();
    // don't intersect the first lines, otherwise the origin will be an intersection point
    for line1 in &wire1[..] {
        for line2 in &wire2[..] {
            match line1.intersect(line2) {
                Some(intersection_point) => {
                    if intersection_point != origin {
                        intersection_points.push(intersection_point)
                    } else {
                        ()
                    }
                }
                None => (),
            }
        }
    }
    return intersection_points;
}

fn wire_steps_to_point(wire: &Vec<GridLine>, point: &Point) -> Option<u32> {
    let mut steps = 0;
    for l in wire {
        if l.contains(&point) {
            return Some(steps + manhattan_distance(&l.0, point));
        } else {
            steps += l.len();
        }
    }
    return None;
}

fn get_closest_intersection_by_steps(
    wire1: &Vec<GridLine>,
    wire2: &Vec<GridLine>,
) -> Option<(Point, u32)> {
    let intersection_points = intersect_wires(wire1, wire2);
    let mut closest_intersection_point = None;
    let mut closest_intersection_distance = u32::max_value();
    for point in intersection_points {
        let steps = wire_steps_to_point(&wire1, &point).expect("Intersection point not on wire1!")
            + wire_steps_to_point(&wire2, &point).expect("Intersection point not on wire1!");
        if steps < closest_intersection_distance {
            closest_intersection_distance = steps;
            closest_intersection_point = Some(point);
        }
    }
    match closest_intersection_point {
        None => return None,
        Some(point) => return Some((point, closest_intersection_distance)),
    }
}

fn get_closest_intersection(wire1: &Vec<GridLine>, wire2: &Vec<GridLine>) -> Option<(Point, u32)> {
    let intersection_points = intersect_wires(wire1, wire2);
    let mut closest_intersection_point = None;
    let mut closest_intersection_distance = u32::max_value();
    let origin = Point { x: 0, y: 0 };
    for point in intersection_points {
        let dist = manhattan_distance(&origin, &point);
        if dist < closest_intersection_distance {
            closest_intersection_distance = dist;
            closest_intersection_point = Some(point);
        }
    }
    match closest_intersection_point {
        None => return None,
        Some(point) => return Some((point, closest_intersection_distance)),
    }
}

#[test]
fn test_wire_from_string_first() {
    let wire_string = "R8,U5,L5,D3";
    let wire = [
        GridLine(Point { x: 0, y: 0 }, Point { x: 8, y: 0 }),
        GridLine(Point { x: 8, y: 0 }, Point { x: 8, y: 5 }),
        GridLine(Point { x: 8, y: 5 }, Point { x: 3, y: 5 }),
        GridLine(Point { x: 3, y: 5 }, Point { x: 3, y: 2 }),
    ];
    assert_eq!(wire_from_string(wire_string), wire);
}

#[test]
fn test_wire_from_string_second() {
    let wire_string = "U7,R6,D4,L4";
    let wire = [
        GridLine(Point { x: 0, y: 0 }, Point { x: 0, y: 7 }),
        GridLine(Point { x: 0, y: 7 }, Point { x: 6, y: 7 }),
        GridLine(Point { x: 6, y: 7 }, Point { x: 6, y: 3 }),
        GridLine(Point { x: 6, y: 3 }, Point { x: 2, y: 3 }),
    ];
    assert_eq!(wire_from_string(&wire_string), wire);
}

#[test]
#[should_panic]
fn test_grid_line_intersection_no_grid_line() {
    let l1 = GridLine(Point { x: 4, y: 10 }, Point { x: -4, y: 10 });
    let l_no_grid_line = GridLine(Point { x: 0, y: -11 }, Point { x: -2, y: 10 });
    l1.intersect(&l_no_grid_line);
}

#[test]
fn test_grid_line_no_intersection() {
    let l2 = GridLine(Point { x: 0, y: 0 }, Point { x: 0, y: 10 });
    let l_no_intersect = GridLine(Point { x: 4, y: 11 }, Point { x: -4, y: 11 });
    assert_eq!(l2.intersect(&l_no_intersect).is_none(), true);
}

#[test]
fn test_grid_line_intersection() {
    let l1 = GridLine(Point { x: 4, y: 10 }, Point { x: -4, y: 10 });
    let l2 = GridLine(Point { x: 0, y: 0 }, Point { x: 0, y: 10 });
    assert_eq!(
        l1.intersect(&l2).expect("no intersection!"),
        Point { x: 0, y: 10 }
    );
}

#[test]
fn test_grid_line_contains_self() {
    let l = GridLine(Point { x: 12, y: 17 }, Point { x: 12, y: 20 });
    assert_eq!(l.contains(&l.0), true);
    assert_eq!(l.contains(&l.1), true);
}

#[test]
#[should_panic]
fn test_grid_line_contains_panics() {
    let l = GridLine(Point { x: 10, y: 17 }, Point { x: 12, y: 20 });
    assert_eq!(l.contains(&l.0), true);
}

#[test]
fn test_grid_line_horizontal_contains() {
    let l = GridLine(Point { x: -12, y: 1 }, Point { x: 12, y: 1 });
    assert_eq!(l.contains(&Point { x: 10, y: 2 }), false);
    assert_eq!(l.contains(&Point { x: 0, y: 1 }), true);
}

#[test]
fn test_grid_line_vertical_contains() {
    let l = GridLine(Point { x: 12, y: 17 }, Point { x: 12, y: 20 });
    assert_eq!(l.contains(&Point { x: 10, y: 18 }), false);
    assert_eq!(l.contains(&Point { x: 12, y: 18 }), true);
}

#[test]
fn test_wire_intersection_distance_by_steps1() {
    let wire1 = wire_from_string("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = wire_from_string("U62,R66,U55,R34,D71,R55,D58,R83");
    let closest_intersection = get_closest_intersection_by_steps(&wire1, &wire2);
    assert_eq!(closest_intersection.expect("No intersection found!").1, 610);
}

#[test]
fn test_wire_intersection_distance_by_steps2() {
    let wire1 = wire_from_string("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = wire_from_string("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    let closest_intersection = get_closest_intersection_by_steps(&wire1, &wire2);
    assert_eq!(closest_intersection.expect("No intersection found!").1, 410);
}

#[test]
fn test_wire_intersection_distance1() {
    let wire1 = wire_from_string("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = wire_from_string("U62,R66,U55,R34,D71,R55,D58,R83");
    let closest_intersection = get_closest_intersection(&wire1, &wire2);
    assert_eq!(closest_intersection.expect("No intersection found!").1, 159);
}

#[test]
fn test_wire_intersection_distance2() {
    let wire1 = wire_from_string("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = wire_from_string("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    let closest_intersection = get_closest_intersection(&wire1, &wire2);
    assert_eq!(closest_intersection.expect("No intersection found!").1, 135);
}

#[test]
fn test_manhattan_distance() {
    let p1 = Point { x: 5, y: 9 };
    let p2 = Point { x: 1, y: 4 };
    assert_eq!(manhattan_distance(&p1, &p2), 9);
    assert_eq!(manhattan_distance(&p2, &p1), 9);
    let p1 = Point { x: -5, y: -9 };
    let p2 = Point { x: -1, y: -4 };
    assert_eq!(manhattan_distance(&p1, &p2), 9);
    assert_eq!(manhattan_distance(&p2, &p1), 9);
    let p1 = Point { x: -5, y: 9 };
    let p2 = Point { x: 1, y: -4 };
    assert_eq!(manhattan_distance(&p1, &p2), 19);
    assert_eq!(manhattan_distance(&p2, &p1), 19);
}
