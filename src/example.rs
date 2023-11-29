use std::collections::HashMap;
use std::collections::HashSet;
use delicious_fruits::apple;
use List::{Cons, Nil};
// use std::io;
use bs58;
mod sports_player;
use sports_player::{football_player};
use std::{fs,println};

fn main() {
  // let stdin = io::stdin();
  // let message="Hello Shashank!";
  // let x: i32 = 42;
  // let pi: f64=3.14;
  // let is_rust_fun: bool=true;
  // let letter_a:char ='a';
  // println!("Hello, world!");
  // fn add(x:i32,y: i32)-> i32{
  //      x+y
  // }

  // let x = 4;
  
  // if x>=0 {
  //     println!("x is not negative");
  // }else{
  //     println!("x is negative");
  // }
  // let mut i=1;
  // while i<= 5 {
  //     println!("{}",i);
  //     i+=1;
  // }

  // let _my_first_bool = true;
  // let _my_second_bool:bool=true;

  // //8 16 32 64 128
  // let _days_of_week:i8=7;
  // let _number_of_users:i64=128000;

  // let _number_of_tokens:u64=1000;
  // let _just_a_number=0;

  // //float 32 64
  // let _pi:f32=3.14;

  // //char
  // let _my_char:char='1';

  // //strings
  // let _message: &str="Hi shashank";
  // let _my_string=String::from("Hi Simon");

  // //Array
  // let _days_of_week: [&str;7]=["monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];

  // let _first_element= _days_of_week[0];
  // let _last_element=_days_of_week[_days_of_week.len()-1];

  // //slices
  // let slice=&_days_of_week[1..3];
  // let _first_element_of_slice=slice[0];

  // //Tuples
  // let person=("Alice",30);

  // let _name=person.0;
  // let _age=person.1;

  // //Unit type
  // let _unit_type=();

  // //variables
  // let mut num=5;
  // num=6;

  //     //function
  //     let sum=add(3,5);
  //     println!("The sum is {}",sum);

  //     let day_of_the_week="Sunday";

  //     if day_of_the_week=="sunday"{
  //         println!("The race day");
  //     }else if day_of_the_week=="Saturday"{
  //         println!("Qualifying today");
  //     }else{
  //         println!("Patiently waiting for the day");
  //     }


  //     //while 
  //     let mut counter=0;
  //     while counter <5 {
  //         println!("The counter value is {}",counter);
  //         counter+=1;
  //     }

  //     //for loop
  //     let numbers:[i32;5]=[10,20,3,40,50];
  //     counter=0;
  //     for number in 1..=5{
  //         println!("Number is {}",number);
  //     }

  //     for number in numbers{
  //         println!("The number is {}",number);
  //     }

  //     //loop
  //     counter=0;
  //     loop{
  //         println!("counter value is {}",counter);
  //         counter+=1;
  //         if counter==6{
  //             break;
  //         }
  //     }

  //     //match 
  //     let num=5;
  //     match num{
  //         1 => {
  //             println!("Number is 1");
  //         }
  //         2 => {
  //             println!("Number is 2");
  //         }
  //         3 => println!("Number is 2"),
  //         _=> println!("number is something else")     
  //     }

  //     let result=match num {
  //         1=> "The number is one",
  //         2=> "The number is two",
  //         3=> "The number is three",
  //         _=> "The number is something else",
  //     };
  //     println!("{}",result);
  // }


  // fn add(x:i32,y:i32)-> i32{
  //     let result=x+y;
  //     return result;
  // }

  // fn no_param()->i32{
  //     println!("this just works");
  //  }

  // let s1=String::from("hello");
  // let s2=s1;
  // print!("value of s1 is {}",s1);

  // let x:i32=5;
  // let y=String::from("patika");

  // let z=y;
  // println!("value of x is {} and value of z is {} and value of y is {}",x,z,y);

  // let my_string = String::from("hello, world!"); 
  // let slice = &my_string[0..5]; 
  // println!("{}", slice); 

  // let x = 5; // stored on the stack 
  // let y = String::from("hello"); // stored on the heap 
  // println!("x = {}, y = {}", x, y); 

  // let x = 5; // stored on the stack 
  // let y = String::from("hello"); // stored on the heap 
  // let z = y; // ownership of y is moved to z 
  // println!("x = {}, z = {}", x, z); 


  // let my_string=String::from("Hello World");
  // let my_ref=&my_string;

  // println!("my reference is {}",my_ref);
  // print_String(&my_string); 
  // println!("i still got back my string {}",my_string);   
  // let mut my_string1=String::from("Hello World");
  // change_String(&mut my_string1);

  // println!("changed string {}",my_string1);

  // let r1 = &my_string1; // immutable reference
  // let r2 = &my_string1; // another immutable reference


  // println!("{}, {}", r1, r2); // 

  // let r3 = &mut my_string1; // immutable reference

  // println!("{}", r3); //
  
  // // println!("imutable reference value {}",r1);

  // let r4 = &mut my_string1; // immutable reference

  // println!("{}", r4); //
  // // println!("{}", r3); //

  // let new_string2=String::from("new String");
  // let new_string_ref=return_reference(&new_string2);

  // println!("new string {}",new_string2);

  // let new_string3=new_string2;
  // // println!("new string ref {}",new_string_ref);


  // let original_string=String::from("helo world");
  // let clone_string=original_string.clone();
  // println!("original string {}",original_string);
  // println!("original string {}",clone_string);

  // let original_string=String::from("world"); 
  // let modified_string=modify_string(&original_string);
  // println!("original string {}",original_string);
  // println!("modified string {}",modified_string)

  // let point = Point { x: 1.0, y: 2.0 };
  // let another_point = Point { x: -3.0, y: 4.0 };
  // let mut alice = Person {
  //   name: String::from("Alice"),
  //   age: 30,
  // };

  // // Accessing the 'name' and 'age' fields
  // println!("Name: {}", alice.name);
  // println!("Age: {}", alice.age);

  // // Modifying the 'age' field
  // alice.age = 31;
  // println!("Updated Age: {}", alice.age);

  // let point1 = Point { x: 1.0, y: 1.0 };
  // let point2 = Point { x: 4.0, y: 5.0 };

  // let dist = distance(&point1, &point2);
  // println!("The distance between point1 and point2 is: {}", dist);

  // let mid_point = midpoint(point1, point2);
  // println!("The midpoint of point1 and point2 is: ({}, {})", mid_point.x, mid_point.y);

  // let point = Point3D(1.0, 2.0, 3.0);
  // let x = point.0;
  // let y = point.1;
  // let z = point.2;
  // println!("{}",calculate_distance(&point,&point))

  // let empty_instance = Empty;
  // empty_instance.greet();
  // let my_rectangle = Rectangle {
  //   width: 10.0,
  //   height: 5.0,
  // };
  // let area = my_rectangle.area();
  // println!("The area of the rectangle is: {}", area);


  // let current_weather = Weather::Sunny;
  // let msg = Message::Write(String::from("Hello, Rust!"));
  // let my_pet = Animal::Cat("Fluffy".to_string());
  // if let Animal::Cat(name) = my_pet {
  //     println!("My cat's name is: {}", name);
  // } else {
  //     println!("My pet is not a cat.");
  // }
  // let msg = Message::Write(String::from("Hello, Rust!"));
  // msg.call();

  // let mut numbers = vec![1, 2, 3, 4];
  // // Using the Vec::new() function
  // // let mut names: Vec = Vec::new();
  // let mut names: Vec<String> = Vec::new();
  // names.push(String::from("Alice"));
  // names.push(String::from("Bob"));

  // let names = vec![String::from("Alice"), String::from("Bob")];
  // let first_name = &names[0]; // Accessing the first element using indexing
  // println!("The first name is: {}", first_name);

  // let numbers = vec![1, 2, 3, 4];
  // for number in &numbers {
  //     println!("Number: {}", number);
  // }
  // let numbers = vec![1, 2, 3, 4, 5];
  // let slice = &numbers[1..4];

  // let mut hello = String::from("Hello, ");
  // hello.push_str("world!");
  // hello.push('!');
  // println!("{}", hello); // "Hello, world!!"
  
  // let example = String::from("hello");
  // let slice = &example[0..2];
  // println!("{}", slice); // "he"

  // let text = "こんにちは";
  // for c in text.chars() {
  //     println!("{}", c);
  // }


  // let example = String::from("hello");
  // // Iterate over characters
  // for c in example.chars() {
  //     println!("{}", c);
  // }

  // // Iterate over bytes
  // for b in example.bytes() {
  //     println!("{}", b);
  // }

  // let mut scores = HashMap::new();
  // scores.insert(String::from("Alice"), 10);
  // scores.insert(String::from("Bob"), 20);

  // let alice_score = scores.get(&String::from("Alice"));
  // println!("Alice's score: {:?}", alice_score);
  // scores.insert(String::from("Alice"), 30);
  // println!("Updated scores: {:?}", scores);
  // scores.remove(&String::from("Alice"));
  // println!("Scores after removing Alice: {:?}", scores);
  // for (key, value) in &scores {
  //   println!("{}: {}", key, value);
  // }

  //iterators
  // let mut fib = Fibonacci { current: 0, next: 1 };

  // for _ in 0..10 {
  //     println!("{}", fib.next().unwrap());
  // }

  // let vec = vec![1, 2, 3, 4, 5];
  // for item in vec.iter() {
  //     println!("{}", item);
  // }
  
  // let mut vec = vec![1, 2, 3, 4, 5];
  // for item in vec.iter_mut() {
  //     *item *= 2;
  // }
  
  // for item in vec.iter() {
  //   println!("{}", item);
  // }

  // let countdown = Countdown { remaining: 5 };

  // for i in countdown {
  //     println!("Remaining: {}", i);
  // }


  // //HashMap
  // let mut scores = HashMap::new();
  // scores.insert(String::from("Alice"), 10);
  // scores.insert(String::from("Bob"), 20);
  
  // for (key, value) in scores.iter() {
  //     println!("{}: {}", key, value);
  // }  
  
  // let numbers = vec![1, 2, 3, 4, 5];
  // let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

  // println!("{:?}", doubled); // Output: [2, 4, 6, 8, 10]

  // let numbers = vec![1, 2, 3, 4, 5];
  // let even_numbers: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

  // println!("{:?}", even_numbers); // Output: [2, 4]

  // let numbers = vec![1, 2, 3, 4, 5];
  // let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);

  // println!("The sum of the numbers is: {}", sum); // Output: The sum of the numbers is: 15
  
  // //chaining with iterator
  // let numbers = vec![1, 2, 3, 4, 5];
  // let doubled_evens: Vec<_> = numbers
  //     .iter()
  //     .filter(|n| *n % 2 == 0)
  //     .map(|n| n * 2)
  //     .collect();
  
  // println!("{:?}", doubled_evens);

  // let words = vec!["apple", "banana", "cherry", "date", "fig"];
  // let result: Vec<_> = words
  //     .iter()
  //     .enumerate()
  //     .filter(|(i, _)| i % 2 == 0)
  //     .map(|(i, w)| format!("{}: {}", i + 1, w.to_uppercase()))
  //     .collect();
  
  // println!("{:?}", result);

  // let numbers = vec![1, 2, 3, 4, 5];
  // let doubled_numbers: Vec<_> = numbers.iter().map(|n| n * 2).collect();
  
  // println!("{:?}", doubled_numbers);
   
  // let numbers = vec![1, 2, 3, 4, 5];
  // let unique_squares: HashSet<_> = numbers
  //     .iter()
  //     .map(|n| n * n)
  //     .collect();
  
  // println!("{:?}", unique_squares);  


  // let numbers = vec![1, 2, 3, 4, 5];
  // let number_squares: HashMap<_, _> = numbers
  //     .iter()
  //     .map(|n| (n, n * n))
  //     .collect();
  
  // println!("{:?}", number_squares);  

  // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  // let even_sum: i32 = numbers
  //     .iter()
  //     .filter(|n| *n % 2 == 0)
  //     .sum();
  
  // println!("Sum of even numbers: {}", even_sum);  

  //closures
  //   let print_text = || println!("Defining Closure");
  //   print_text();

  //   let is_even = |x: i32| -> bool { x % 2 == 0 };
  //   println!("Is 2 even? {}", is_even(2)); // true
  //   println!("Is 3 even? {}", is_even(3)); // false

  //   let numbers = vec![1, 2, 3, 4, 5];

  //   // filter out only the even numbers using a closure
  //   let evens: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();

  //   // print the filtered vector
  //   println!("The even numbers are: {:?}", evens); // [2, 4]

  //   let print_data = |data: &str| {
  //     println!("Received data: {}", data);
  // };

  //   // call the download function with a url and a closure as arguments
  //   download_data("https://www.rust-lang.org", print_data);

  // sports::football();    // This works!
  // sports::basketball();

  // apple::munch();  // Look, Ma! No hands!

  // football_player()

  // chop("carrot");
  // chop("fruit");

  //option
  // let number = 48985.0;
  // let square_root = find_square_root(number);

  // match square_root {
  //   MyOption::Some(value) => println!("The square root of {} is: {}", number, value),
  //   MyOption::None => println!("The square root of {} is not a real number.", number),
  // }  
    
  //Option
  // let a = 10.0;
  // let b = 0.0;
  // let division_result = divide(a, b);

  // match division_result {
  //   Result::Ok(value) => println!("{} divided by {} is: {}", a, b, value),
  //   Result::Err(error_message) => println!("Error: {}", error_message),
  // }

  // let base = get_from_database("base");
  // let height = get_from_database("height");
  // let area_result = calculate_triangle_area(base, height);

  // match area_result {
  //   Result::Ok(area) => println!("The area of the triangle is: {} square units.", area),
  //   Result::Err(error_message) => println!("Error: {}", error_message),
  // }

  //traits
  // let books = Books {
  //   title: String::from("The Rust Book"),
  // };
  // let magazine = Magazine {
  //     name: String::from("Rust Monthly"),
  // };

  // print_something(&books);
  // print_something(&magazine);

  // let dogg = Dogg;
  // let catt = Catt;

  // dogg.speak(); // Prints "The animal says: Woof!"
  // catt.speak(); // Prints "The animal says: Meow!"

  //box is used to keep the elements in memory
  // let b = Box::new(5);
  // println!("b = {}", *b);

	// let list = Cons(1,
  //   Box::new(Cons(2,
  //       Box::new(Cons(3,
  //           Box::new(Nil))))));

  // let circle = Circle{radius: 5.0};
  // let rectangle = Rectangle{width: 4.0, height: 3.0};

  // draw_object(&circle); // Output: Drawing a circle with radius 5
  // draw_object(&rectangle); // Output: Drawing a rectangle with width 4 and height 3
  
  // let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();
  // let mut printable_objects: HashMap<String, Box<dyn Printable>> = HashMap::new();

  // let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
  // shapes.push(Box::new(Circle{radius: 5.0}));
  // shapes.push(Box::new(Rectangle{width: 4.0, height: 6.0}));
  // for shape in shapes {
  //     println!("Shape area: {}", shape.area());
  // }

  //Generics
                 


}

fn dividee(numerator: f64, denominator: f64) -> Option<f64> {
  if denominator == 0.0 {
      None
  } else {
      Some(numerator / denominator)
  }
}

enum List {
	Cons(i32, Box<List>),
	Nil,
}

trait PartyTrick {
  fn perform(&self);
}

fn invite_to_perform(trickster: Box<dyn PartyTrick>) {
  println!("Ladies and gentlemen, please welcome our next performer!");
  trickster.perform();
}

trait Speak {
  fn speak(&self);
}
struct Dog {
  name: String,
}

trait Animals {
    fn speak(&self);
}

trait Mammal: Animals {
    fn walk(&self);
}

struct Dogs;

impl Animals for Dogs {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Mammal for Dogs {
    fn walk(&self) {
        println!("The dog is walking.");
    }
}

impl Speak for Dog {
  fn speak(&self) {
      println!("{} says: Woof!", self.name);
  }
}

trait Animall {
  fn make_sound(&self) -> &str;

  fn speak(&self) {
      println!("The animal says: {}", self.make_sound());
  }
}
struct Dogg;
struct Catt;

impl Animall for Dogg {
    fn make_sound(&self) -> &str {
        "Woof!"
    }
    fn speak(&self) {
      println!("The dog barks: {}", self.make_sound());
  }
}

impl Animall for Catt {
    fn make_sound(&self) -> &str {
        "Meow!"
    }
}

trait Display {
  fn display(&self) -> String;
}

trait Drawable {
  fn draw(&self);
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
  radius: f64,
}

impl Shape for Circle {
  fn area(&self) -> f64 {
      std::f64::consts::PI * self.radius * self.radius
  }
}

impl Drawable for Circle {
  fn draw(&self) {
      println!("Drawing a circle with radius {}", self.radius);
  }
}

impl Drawable for Rectangle {
  fn draw(&self) {
      println!("Drawing a rectangle with width {} and height {}", self.width, self.height);
  }
}

fn draw_object(d: &dyn Drawable) {
  d.draw();
}

impl Display for Circle {
  fn display(&self) -> String {
      format!("Circle with radius: {}", self.radius)
  }
}
struct Rectangle {
  width: f64,
  height: f64,
}

impl Shape for Rectangle {
  fn area(&self) -> f64 {
      self.width * self.height
  }
}

impl Display for Rectangle {
  fn display(&self) -> String {
      format!("Rectangle with width: {} and height: {}", self.width, self.height)
  }
}

impl Display for String {
  fn display(&self) -> String {
      self.clone()
  }
}

trait Printable {
  fn print(&self);
}

fn print_something<T: Printable>(item: &T) {
  item.print();
}

struct Books {
  title: String,
}

impl Printable for Books {
  fn print(&self) {
      println!("Books: {}", self.title);
  }
}

struct Magazine {
  name: String,
}

impl Printable for Magazine {
  fn print(&self) {
      println!("Magazine: {}", self.name);
  }
}

pub enum Result<T, E> {
  Ok(T),
  Err(E),
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
      Result::Err("Division by zero is not allowed.".to_string())
  } else {
    Result::Ok(a / b)
  }
}

fn get_from_database(key: &str) -> Option<f64> {
  let database = vec![
      ("base", Some(4.0)),
      ("height", Some(6.0)),
  ];

  for (k, v) in database {
      if k == key {
          return v;
      }
  }
  None
}

fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
  match (base, height) {
      (Some(b), Some(h)) => {
          if b <= 0.0 || h <= 0.0 {
            Result::Err("Both base and height must be positive numbers.".to_string())
          } else {
            Result::Ok(0.5 * b * h)
          }
      },
      (None, _) => Result::Err("The base is missing.".to_string()),
      (_, None) => Result::Err("The height is missing.".to_string()),
  }
}

enum MyOption {
  Some(f64),
  None,
}

fn find_square_root(number: f64) -> MyOption {
  if number >= 0.0 {
      MyOption::Some(number.sqrt())
  } else {
      MyOption::None
  }
}

fn chop(vegetable: &str) {
  match vegetable {
      "carrot" => println!("Chopping a carrot."),
      "celery" => println!("Chopping a celery."),
      "tomato" => panic!("Don't know how to handle a tomato!"),
      _ => panic!("Chopping some unknown vegetable."),
  }
}

mod delicious_fruits {
  pub mod apple {
      pub fn munch() {
          println!("Munching an apple...");
      }
  }
}

mod sports {
  pub fn football() {
      println!("Football is fun!");
  }

  pub fn basketball() {
      println!("Basketball is intense!");
  }
}

fn download_data(url: &str, callback: impl FnOnce(&str)) {
  // simulate downloading data by printing some text
  println!("Downloading data from {}...", url);

  // simulate some delay by sleeping for one second
  std::thread::sleep(std::time::Duration::from_secs(1));

  // simulate some data by creating a string
  let data = format!("Some data from {}", url);

  // execute the callback with the data as an argument
  callback(&data);
}

struct Countdown {
  remaining: i32,
}

impl Iterator for Countdown {
  type Item = i32;

  fn next(&mut self) -> Option<i32> {
      if self.remaining > 0 {
          let current = self.remaining;
          self.remaining -= 1;
          Some(current)
      } else {
          None
      }
  }
}

struct Fibonacci {
  current: u32,
  next: u32,
}

impl Iterator for Fibonacci {
  type Item = u32;
  fn next(&mut self) -> Option<u32> {
      let current = self.current;
      self.current = self.next;
      self.next = current + self.next;
      Some(current)
  }
}

enum Animal {
  Dog(String),
  Cat(String),
  Bird(String),
}
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
impl Message {
  fn call(&self) {
      match self {
          Message::Quit => println!("Quit"),
          Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
          Message::Write(s) => println!("Write: {}", s),
          Message::ChangeColor(r, g, b) => println!("Change color to R: {}, G: {}, B: {}", r, g, b),
      }
  }
}

fn process_message(msg: Message) {
  match msg {
      Message::Quit => {
          println!("The Quit variant has no data.");
      }
      Message::Move { x, y } => {
          println!("Move to coordinates x: {}, y: {}", x, y);
      }
      Message::Write(text) => {
          println!("Text message: {}", text);
      }
      Message::ChangeColor(r, g, b) => {
          println!("Change the color to red: {}, green: {}, blue: {}", r, g, b);
      }
  }
}
enum Weather {
  Sunny,
  Cloudy,
  Rainy,
  Snowy,
}


impl Rectangle {
  fn area(&self) -> f64 {
      self.width * self.height
  }
}

struct Empty;
impl Empty {
    fn greet(&self) {
        println!("Hello, I am an empty struct!");
    }
}

struct Point3D(f32, f32, f32);

fn calculate_distance(point1: &Point3D, point2: &Point3D) -> f32 {
  let dx = point1.0 - point2.0;
  let dy = point1.1 - point2.1;
  let dz = point1.2 - point2.2;

  (dx*dx + dy*dy + dz*dz).sqrt()
}

struct Book {
  title: String,
  author: String,
  publication_year: u32,
}

struct Point {
  x: f32,
  y: f32,
}

struct Person {
  name: String,
  age: u32,
}

fn distance(p1: &Point, p2: &Point) -> f32 {
  let x_diff = p1.x - p2.x;
  let y_diff = p1.y - p2.y;
  (x_diff * x_diff + y_diff * y_diff).sqrt()
}

fn midpoint(p1: Point, p2: Point) -> Point {
  let x_mid = (p1.x + p2.x) / 2.0;
  let y_mid = (p1.y + p2.y) / 2.0;
  Point { x: x_mid, y: y_mid }
}

fn modify_string(s:&String)-> String{
  let mut cloned_string=s.clone();
  cloned_string.push_str("modified");
  cloned_string

}

fn print_String(s:&String){
  println!("{}",s);
}

fn change_String(s: &mut String){
  s.push_str("world");
}

fn return_reference(some_string:&String)-> &String{
  some_string
} 




