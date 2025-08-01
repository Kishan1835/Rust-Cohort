// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sigin_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("ts.kishan2003@gmail.com"),
//         email: String::from("ts.kishan2003@gmail.com"),
//         sigin_in_count: 2,
//     };
//     println!(
//         "User 1  username{:?} email {:?}",
//         user1.username, user1.email
//     )
// }

// pattern mathcing

// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangel(f64, f64),
// }

// fn calculate_data(Shape: Shape) -> f64 {
//     let result = match Shape {
//         Shape::Circle(radius) => 3.14 * radius * radius,
//         Shape::Rectangel(width, height) => width * height,
//         Shape::Square(side) => side * side,
//     };
//     return result;
// }

// fn main() {
//     let circle_data: Shape = Shape::Circle(3.4);
//     let square_data: Shape = Shape::Square(11.3);
//     let rectangel_data: Shape = Shape::Rectangel(2.3, 5.8);

//     println!("the area of circle {:?}", calculate_data(circle_data));
//     println!("the area of square {:?}", calculate_data(square_data));
//     println!("the area of rectangle {:?}", calculate_data(rectangel_data));
// }


// Error handling 
