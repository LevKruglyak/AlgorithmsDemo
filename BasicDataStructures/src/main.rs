mod list;
mod stack;

pub use list::*;
pub use stack::*;

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
