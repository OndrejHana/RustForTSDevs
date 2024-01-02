mod reader;
mod shapes;

use reader::custom_file::read_file;
use shapes::collisions::Collidable;

use crate::shapes::{rect::Rect, circle::Circle, area::Area};

#[derive(Debug)]
struct User {
    name: String,
    age: usize,
}

#[derive(Debug)]
enum Item {
    MyNumber(i32),
    MyString(String),
    MyUser(User)
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::MyString("Hello world".into()));
}

fn multiply(num: Option<i32>) -> Option<i32> {
    return match num {
        Some(num) => Some(num*5),
        None => None,
    };
}

fn print_all(items: &Vec<NumberItem>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn practice(nums: &Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index)*5;
}

fn run1() {
    let mut items = Vec::new();

    append(&mut items);
    append(&mut items);

    println!("{:#?}", items);

    println!("{:?}, {:?}, {:?}", multiply(Some(2)), multiply(Some(10)), multiply(None));

    let nums1 = vec![1,10,69,420];

    println!("{}, {}, {}", practice(&nums1, 0), practice(&nums1, 3), practice(&nums1, 5));
}

fn run2() {
    let file_path = std::env::args().nth(1)
        .expect("required argument not found ");

    let file_content = std::fs::read_to_string(file_path)
        .expect("file could not be read");

    for line in file_content.lines() {
        if let Ok(num) = line.parse::<isize>() {
            println!("number {}", num);
        }
    }
}

#[derive(Debug)]
struct NumberItem(usize);

fn add_one(item: &mut NumberItem) {
    item.0 += 1;
}

fn run3() {

    let mut item = NumberItem(1);
    add_one(&mut item);
    println!("{:?}", item);

    let mut items: Vec<NumberItem> = Vec::new();
    items.push(item);

    let first_item= items.first_mut().unwrap();

    println!("{:?}", first_item);

    print_all(&items);

    let items: Vec<_> = vec![1,2,3].iter().map(|x| x+1).collect();

    println!("{:?}", items);
}



fn run4() {
    let rect = Rect {
        x: 0.,
        y: 0.,
        width: 10.,
        height: 10.,
    };

    let circ = Circle {
        x: 0.,
        y: 0.,
        radius: 10.,
    };

    println!("{}", rect);
    println!("{}", circ);
    println!("{}", rect.area());
    println!("{}", circ.area());



    println!("{}", rect);
}

fn run5() {
    let rect = Rect::default();
    let rect2= Rect::default();

    let circle = Circle {
        x: 0.,
        y: 0.,
        radius: 2.5,
    }; 
    let circle2 = Circle {
        x: 1.5,
        y: 1.5,
        radius: 4.,
    }; 

    let mut idk = Vec::new();
    idk.push(rect.collide(&rect2));
    idk.push(circle.collide(&circle2));
    idk.push(rect.collide(&circle));

    println!("{idk:?}");

}

fn run6() {
    read_file("./shapes".into()).unwrap();
}



fn main() {
    // run1();
    // run2();
    // run3();
    // run4();
    // run4();
    // run5();
    run6();
}
