use rand::{thread_rng, Rng};
use rocket::serde::{Serialize};

/// Muffin, a struct that holds our muffin data!
/// It can have all sorts of things.
///
/// * Color (the color in the form of RGB) [i32;3]
/// * Seeds (if it has seeds) bool
/// * Weight (size in like lbs or grams or something) f32
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Muffin {
    color : [i32; 3],
    has_seeds : bool,
    weight: f32,
}

pub fn bake_muffin() -> Muffin {
    pub fn random_color() -> i32 {
        let mut rng = thread_rng();
        rng.gen_range(0..=255)
    }

    Muffin {
        color: [random_color(), random_color(), random_color()],
        has_seeds: rand::random(),
        weight: ((rand::random::<f32>()) * 4.0) + 0.5,
    }
}
