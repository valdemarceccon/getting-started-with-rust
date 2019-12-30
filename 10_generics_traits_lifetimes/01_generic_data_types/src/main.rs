struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// Do not work
//fn largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list.iter() {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}

fn main() {
//    let number_list = vec![34, 50, 25, 100, 65];
//
//    let result = largest(&number_list);
//
//    println!("The largest number is {}!", result);
//
//    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//
//    let result = largest(&number_list);
//
//    println!("The largest number is {}!", result);

    let p = Point{x:5, y:6};
    println!("X value: {}\nY value: {}", p.x(), p.y);

    let p = Point{x: 5.0, y: 6.0};
    println!("Distance from origin: {}", p.distance_from_origin());
}
