extern crate pathfinder;
extern crate rand;

use pathfinder::*;
use rand::{thread_rng, Rng};

#[test]
fn conversion_test() {
    let flight_zone = vec![vec![
        Point::from_degrees(0.3, 0.6, 0f32),
        Point::from_degrees(0.0, 0.6, 0f32),
        Point::from_degrees(0.0, 0.0, 0f32),
        Point::from_degrees(0.3, 0.0, 0f32),
    ]];
    let mut pathfinder = Pathfinder::new();
    pathfinder.init(1.0, flight_zone, Vec::new());
    let test_points = vec![
        Point::from_degrees(30.32247, -97.6009, 0f32),
        Point::from_degrees(30.32307, -97.6005, 0f32),
        Point::from_degrees(30.32373, -97.6012, 0f32),
        Point::from_degrees(30.32366, -97.6019, 0f32),
        Point::from_degrees(30.32321, -97.6025, 0f32),
        Point::from_degrees(30.32521, -97.60230, 0f32),
        Point::from_degrees(30.32466, -97.59856, 0f32),
        Point::from_degrees(30.32107, -97.60032, 0f32),
        Point::from_degrees(30.32247, -97.60325, 0f32),
        Point::from_degrees(30.32473, -97.60410, 0f32),
    ];
    for point in test_points {
        let node1 = point.to_node(&pathfinder);
        let point1 = node1.to_point(&pathfinder);
        // print!("{:.5}, {:.5} => ", point.lat_degree(), point.lon_degree());
        println!("{:.5}, {:.5}", point1.lat_degree(), point1.lon_degree());
        assert!(point.lat_degree() - point1.lat_degree() < 0.001);
        assert!(point.lon_degree() - point1.lon_degree() < 0.001);
    }
}

#[test]
fn random_test() {
    let mut pathfinder = Pathfinder::new();
    pathfinder.init(1.0, Vec::new(), Vec::new());
    let mut rng = thread_rng();
    for _ in 1..100 {
        let point = Point::from_degrees(
            rng.gen_range(-90f64, 90f64),
            rng.gen_range(-180f64, 180f64),
            0f32,
        );
        let node1 = point.to_node(&pathfinder);
        let point1 = node1.to_point(&pathfinder);
        // print!("{:.5}, {:.5} => ", point.lat_degree(), point.lon_degree());
        println!("{:.5}, {:.5}", point1.lat_degree(), point1.lon_degree());
        assert!(point.lat_degree() - point1.lat_degree() < 0.001);
        assert!(point.lon_degree() - point1.lon_degree() < 0.001);
    }
}
