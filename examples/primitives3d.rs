extern crate rand;
extern crate nalgebra as na;
extern crate ncollide;

use na::Pnt3;
use ncollide::transformation;

fn main() {
    let mut points = Vec::new();
    for _ in 0usize .. 100000 {
        points.push(rand::random::<Pnt3<f32>>() * 2.0);
    }

    let _ = transformation::convex_hull3(&points[..]);
}
