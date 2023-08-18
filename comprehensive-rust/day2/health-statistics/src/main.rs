use std::ops::Sub;

pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

#[derive(Debug)]
pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self { name, age, height, ..Self::default() }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn doctor_visits(&self) -> u32 {
        self.visit_count as u32
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age
    }

    pub fn set_height(&mut self, new_height: f32) {
        self.height = new_height
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        let user = self;
        let patient_name = user.name.as_str();
        let height_change;
        let blood_pressure_change: Option<(i32, i32)>;
        
        height_change = measurements.height.sub(user.height);
        user.height = measurements.height;

        let blood_pressure: ((u32, u32), Option<(u32, u32)>) = ( measurements.blood_pressure, user.last_blood_pressure );
        blood_pressure_change = match blood_pressure {
            ((x, y), Some((prev_x, prev_y))) => Some((x as i32 - prev_x as i32, y as i32- prev_y as i32)),
            ((_, _), None) => None,
        };
        user.last_blood_pressure = Some(measurements.blood_pressure);
        user.visit_count = user.visit_count + 1;

        HealthReport { patient_name, visit_count: user.visit_count as u32, height_change, blood_pressure_change }
    }
}

impl Default for User {
    fn default() -> User {
        User {
            name: "John".to_string(),
            age: 0,
            height: 0.0,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }
}

fn main() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
    println!("Visit count: {}, height: {}.", &bob.doctor_visits(), &bob.height());

    let report = bob.visit_doctor(Measurements {
        height: 155.4,
        blood_pressure: (115, 76),
    });
    println!("{report:?}");
    println!("Visit count: {}, height: {}.", &bob.doctor_visits(), &bob.height());

    let report = bob.visit_doctor(Measurements {
        height: 155.7,
        blood_pressure: (120, 80),
    });
    println!("{report:?}");
    println!("Visit count: {}, height: {}.", &bob.doctor_visits(), &bob.height());

}

#[test]
fn test_height() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.doctor_visits(), 0);
    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (120, 80),
    });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);
    assert_eq!(format!("{:.3}", report.height_change.to_string()), "0.9");

    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (115, 76),
    });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    assert_eq!(bob.height(), 156.1);
}
