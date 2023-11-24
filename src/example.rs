use std::collections::HashMap;

fn main() {
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

  let mut numbers = vec![1, 2, 3, 4];

  // Using the Vec::new() function
  // let mut names: Vec = Vec::new();
  let mut names: Vec<String> = Vec::new();
  names.push(String::from("Alice"));
  names.push(String::from("Bob"));

  let names = vec![String::from("Alice"), String::from("Bob")];
  let first_name = &names[0]; // Accessing the first element using indexing
  println!("The first name is: {}", first_name);

  let numbers = vec![1, 2, 3, 4];
  for number in &numbers {
      println!("Number: {}", number);
  }
  let numbers = vec![1, 2, 3, 4, 5];
  let slice = &numbers[1..4];

  let mut hello = String::from("Hello, ");
  hello.push_str("world!");
  hello.push('!');
  println!("{}", hello); // "Hello, world!!"
  
  let example = String::from("hello");
  let slice = &example[0..2];
  println!("{}", slice); // "he"

  let text = "こんにちは";
  for c in text.chars() {
      println!("{}", c);
  }


  let example = String::from("hello");

  // Iterate over characters
  for c in example.chars() {
      println!("{}", c);
  }

  // Iterate over bytes
  for b in example.bytes() {
      println!("{}", b);
  }

  let mut scores = HashMap::new();

  scores.insert(String::from("Alice"), 10);
  scores.insert(String::from("Bob"), 20);

  let alice_score = scores.get(&String::from("Alice"));
  println!("Alice's score: {:?}", alice_score);
  scores.insert(String::from("Alice"), 30);
  println!("Updated scores: {:?}", scores);
  scores.remove(&String::from("Alice"));
  println!("Scores after removing Alice: {:?}", scores);
  
  for (key, value) in &scores {
    println!("{}: {}", key, value);
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

struct Rectangle {
  width: f64,
  height: f64,
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




