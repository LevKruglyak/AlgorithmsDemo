mod stack;
mod list;

pub use stack::*;
pub use list::*;

fn main() {
    let mut stack = Stack::<i32>::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    let mut sum = 0;
    for element in stack.iter() {
        sum += *element;
    }
    println!("{sum}");
}
