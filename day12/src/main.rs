#[derive(Debug, PartialEq, Clone)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new() -> Vec3 {
        return Vec3 { x: 0, y: 0, z: 0 };
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    pub fn total_energy(&self) -> i32 {
        return self.kinetic_energy() * self.potential_energy();
    }

    fn kinetic_energy(&self) -> i32 {
        return self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs();
    }

    fn potential_energy(&self) -> i32 {
        return self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs();
    }

    pub fn apply_gravity(&mut self, other: &mut Moon) {
        if self.pos.x < other.pos.x {
            self.vel.x += 1;
            other.vel.x -= 1;
        } else if self.pos.x > other.pos.x {
            self.vel.x -= 1;
            other.vel.x += 1;
        }
        if self.pos.y < other.pos.y {
            self.vel.y += 1;
            other.vel.y -= 1;
        } else if self.pos.y > other.pos.y {
            self.vel.y -= 1;
            other.vel.y += 1;
        }
        if self.pos.z < other.pos.z {
            self.vel.z += 1;
            other.vel.z -= 1;
        } else if self.pos.z > other.pos.z {
            self.vel.z -= 1;
            other.vel.z += 1;
        }
    }

    pub fn update_position(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
}

fn main() {
    // hardcoded input:
    let moon1_pos = Vec3 { x: -6, y: 2, z: -9 };
    let moon2_pos = Vec3 {
        x: 12,
        y: -14,
        z: -4,
    };
    let moon3_pos = Vec3 { x: 9, y: 5, z: -6 };
    let moon4_pos = Vec3 { x: -1, y: -4, z: 9 };
    // build moons
    let mut moons = [
        Moon {
            pos: moon1_pos,
            vel: Vec3::new(),
        },
        Moon {
            pos: moon2_pos,
            vel: Vec3::new(),
        },
        Moon {
            pos: moon3_pos,
            vel: Vec3::new(),
        },
        Moon {
            pos: moon4_pos,
            vel: Vec3::new(),
        },
    ];
    let initial_moons = moons.clone();
    for _iteration_nr in 0.. {
        for split_mid in 0..moons.len() {
            let (moons_left, moons_right) = moons.split_at_mut(split_mid);
            let moon_right = &mut moons_right[0];
            for moon_left in moons_left {
                moon_right.apply_gravity(moon_left);
            }
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }
        if moons == initial_moons {
            println!("rediscovered initial state at iteration {}", _iteration_nr);
            break;
        }
    }
}

#[test]
fn update_positions() {
    let mut moons = [
        Moon {
            pos: Vec3 { x: -1, y: 0, z: 2 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 {
                x: 2,
                y: -10,
                z: -7,
            },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 { x: 4, y: -8, z: 8 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 { x: 3, y: 5, z: -1 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
    ];
    for split_mid in 0..moons.len() {
        let (moons_left, moons_right) = moons.split_at_mut(split_mid);
        let moon_right = &mut moons_right[0];
        for moon_left in moons_left {
            moon_right.apply_gravity(moon_left);
        }
    }

    for moon in moons.iter_mut() {
        moon.update_position();
    }

    assert_eq!(
        moons[0],
        Moon {
            pos: Vec3 { x: 2, y: -1, z: 1 },
            vel: Vec3 { x: 3, y: -1, z: -1 }
        }
    );
    assert_eq!(
        moons[1],
        Moon {
            pos: Vec3 { x: 3, y: -7, z: -4 },
            vel: Vec3 { x: 1, y: 3, z: 3 }
        }
    );
    assert_eq!(
        moons[2],
        Moon {
            pos: Vec3 { x: 1, y: -7, z: 5 },
            vel: Vec3 { x: -3, y: 1, z: -3 }
        }
    );
    assert_eq!(
        moons[3],
        Moon {
            pos: Vec3 { x: 2, y: 2, z: 0 },
            vel: Vec3 { x: -1, y: -3, z: 1 }
        }
    );
}

#[test]
fn apply_gravity() {
    let mut moons = [
        Moon {
            pos: Vec3 { x: -1, y: 0, z: 2 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 {
                x: 2,
                y: -10,
                z: -7,
            },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 { x: 4, y: -8, z: 8 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 { x: 3, y: 5, z: -1 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
    ];
    for split_mid in 0..moons.len() {
        let (moons_left, moons_right) = moons.split_at_mut(split_mid);
        let moon_right = &mut moons_right[0];
        for moon_left in moons_left {
            moon_right.apply_gravity(moon_left);
        }
    }

    assert_eq!(
        moons[0],
        Moon {
            pos: Vec3 { x: -1, y: 0, z: 2 },
            vel: Vec3 { x: 3, y: -1, z: -1 }
        }
    );
    assert_eq!(
        moons[1],
        Moon {
            pos: Vec3 {
                x: 2,
                y: -10,
                z: -7,
            },
            vel: Vec3 { x: 1, y: 3, z: 3 }
        }
    );
    assert_eq!(
        moons[2],
        Moon {
            pos: Vec3 { x: 4, y: -8, z: 8 },
            vel: Vec3 { x: -3, y: 1, z: -3 }
        }
    );
    assert_eq!(
        moons[3],
        Moon {
            pos: Vec3 { x: 3, y: 5, z: -1 },
            vel: Vec3 { x: -1, y: -3, z: 1 }
        }
    );
}

#[test]
fn calculate_total_energy() {
    let moon1 = Moon {
        pos: Vec3 {
            x: 8,
            y: -12,
            z: -9,
        },
        vel: Vec3 { x: -7, y: 3, z: 0 },
    };
    let moon2 = Moon {
        pos: Vec3 {
            x: 13,
            y: 16,
            z: -3,
        },
        vel: Vec3 {
            x: 3,
            y: -11,
            z: -5,
        },
    };
    let moon3 = Moon {
        pos: Vec3 {
            x: -29,
            y: -11,
            z: -1,
        },
        vel: Vec3 { x: -3, y: 7, z: 4 },
    };
    let moon4 = Moon {
        pos: Vec3 {
            x: 16,
            y: -13,
            z: 23,
        },
        vel: Vec3 { x: 7, y: 1, z: 1 },
    };
    assert_eq!(moon1.total_energy(), 290);
    assert_eq!(moon2.total_energy(), 608);
    assert_eq!(moon3.total_energy(), 574);
    assert_eq!(moon4.total_energy(), 468);
}

#[test]
fn calculate_potential_energy() {
    let moon1 = Moon {
        pos: Vec3 {
            x: 8,
            y: -12,
            z: -9,
        },
        vel: Vec3 { x: -7, y: 3, z: 0 },
    };
    let moon2 = Moon {
        pos: Vec3 {
            x: 13,
            y: 16,
            z: -3,
        },
        vel: Vec3 {
            x: 3,
            y: -11,
            z: -5,
        },
    };
    let moon3 = Moon {
        pos: Vec3 {
            x: -29,
            y: -11,
            z: -1,
        },
        vel: Vec3 { x: -3, y: 7, z: 4 },
    };
    let moon4 = Moon {
        pos: Vec3 {
            x: 16,
            y: -13,
            z: 23,
        },
        vel: Vec3 { x: 7, y: 1, z: 1 },
    };
    assert_eq!(moon1.potential_energy(), 29);
    assert_eq!(moon2.potential_energy(), 32);
    assert_eq!(moon3.potential_energy(), 41);
    assert_eq!(moon4.potential_energy(), 52);
}

#[test]
fn calculate_kinetic_energy() {
    let moon1 = Moon {
        pos: Vec3 {
            x: 8,
            y: -12,
            z: -9,
        },
        vel: Vec3 { x: -7, y: 3, z: 0 },
    };
    let moon2 = Moon {
        pos: Vec3 {
            x: 13,
            y: 16,
            z: -3,
        },
        vel: Vec3 {
            x: 3,
            y: -11,
            z: -5,
        },
    };
    let moon3 = Moon {
        pos: Vec3 {
            x: -29,
            y: -11,
            z: -1,
        },
        vel: Vec3 { x: -3, y: 7, z: 4 },
    };
    let moon4 = Moon {
        pos: Vec3 {
            x: 16,
            y: -13,
            z: 23,
        },
        vel: Vec3 { x: 7, y: 1, z: 1 },
    };
    assert_eq!(moon1.kinetic_energy(), 10);
    assert_eq!(moon2.kinetic_energy(), 19);
    assert_eq!(moon3.kinetic_energy(), 14);
    assert_eq!(moon4.kinetic_energy(), 9);
}

#[test]
fn test_scenario() {
    let mut moons = [
        Moon {
            pos: Vec3 {
                x: -8,
                y: -10,
                z: 0,
            },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 { x: 5, y: 5, z: 10 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 { x: 2, y: -7, z: 3 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
        Moon {
            pos: Vec3 { x: 9, y: -8, z: -3 },
            vel: Vec3 { x: 0, y: 0, z: 0 },
        },
    ];
    for _iteration_nr in 0..10 {
        for split_mid in 0..moons.len() {
            let (moons_left, moons_right) = moons.split_at_mut(split_mid);
            let moon_right = &mut moons_right[0];
            for moon_left in moons_left {
                moon_right.apply_gravity(moon_left);
            }
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }
        println!("after iteration {}:", _iteration_nr);
        for (moon_id, moon) in moons.iter().enumerate() {
            println! {"Moon {}: {:?}", moon_id, moon};
        }
    }
    assert_eq!(
        moons[0],
        Moon {
            pos: Vec3 {
                x: -9,
                y: -10,
                z: 1
            },
            vel: Vec3 {
                x: -2,
                y: -2,
                z: -1
            }
        }
    );
    assert_eq!(
        moons[1],
        Moon {
            pos: Vec3 { x: 4, y: 10, z: 9 },
            vel: Vec3 { x: -3, y: 7, z: -2 }
        }
    );
    assert_eq!(
        moons[2],
        Moon {
            pos: Vec3 {
                x: 8,
                y: -10,
                z: -3
            },
            vel: Vec3 { x: 5, y: -1, z: -2 }
        }
    );
    assert_eq!(
        moons[3],
        Moon {
            pos: Vec3 { x: 5, y: -10, z: 3 },
            vel: Vec3 { x: 0, y: -4, z: 5 }
        }
    );
}

#[test]
fn part_one_works() {
    let moon1_pos = Vec3 { x: -6, y: 2, z: -9 };
    let moon2_pos = Vec3 {
        x: 12,
        y: -14,
        z: -4,
    };
    let moon3_pos = Vec3 { x: 9, y: 5, z: -6 };
    let moon4_pos = Vec3 { x: -1, y: -4, z: 9 };
    // build moons
    let mut moons = [
        Moon {
            pos: moon1_pos,
            vel: Vec3::new(),
        },
        Moon {
            pos: moon2_pos,
            vel: Vec3::new(),
        },
        Moon {
            pos: moon3_pos,
            vel: Vec3::new(),
        },
        Moon {
            pos: moon4_pos,
            vel: Vec3::new(),
        },
    ];
    for _iteration_nr in 0..1000 {
        for split_mid in 0..moons.len() {
            let (moons_left, moons_right) = moons.split_at_mut(split_mid);
            let moon_right = &mut moons_right[0];
            for moon_left in moons_left {
                moon_right.apply_gravity(moon_left);
            }
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }
    }
    let total_energy: i32 = moons.iter().map(|moon| moon.total_energy()).sum();
    assert_eq!(total_energy, 14907);
}
