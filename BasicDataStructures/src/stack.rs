/// Simple linked list implementation of a stack.
/// About 3 times slower than a Vec based implementation
pub struct Stack<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    /// Returns a new empty stack
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Push an element to the root of the stack
    ///
    /// Example:
    /// ```
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(10);
    /// stack.push(20);
    /// stack.push(30);
    ///
    /// // Stack now looks like:
    /// // [30] -> [20] -> [10] -> {}
    ///
    /// assert_eq!(stack.pop(), Some(30));
    /// ```
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node::<T> {
            data,
            next: self.root.take(),
        });

        self.root = Some(new_node);
    }

    /// Return the root element of the stack, updating the
    /// root to point to the second element. Returns none if
    /// the stack is empty
    ///
    /// Example:
    /// ```
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(10);
    /// assert_eq!(stack.pop(), Some(10))
    /// assert_eq!(stack.pop(), None)
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let root_node = self.root.take()?;
        self.root = root_node.next;
        Some(root_node.data)
    }

    /// Return an optional immutable reference to the root of
    /// the stack, none if the stack is empty
    ///
    /// Example:
    /// ```
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(10);
    /// assert_eq!(stack.peek(), Some(10));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.data)
    }

    /// Return an optional mutable reference to the roof of
    /// the stack, none if the stack is empty
    ///
    /// Example:
    /// ```
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(10);
    /// assert_eq!(list.peek(), Some(&10));
    /// list.peek_mut().map(|data| { *data = 15 });
    /// assert_eq!(list.peek(), Some(&15));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.root.as_mut().map(|node| &mut node.data)
    }

    /// Returns true if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Return an [`Iterator`] which iterates the stack.
    ///
    /// Example:
    /// ```
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(10);
    /// stack.push(20);
    ///
    /// let mut sum = 0;
    /// for element in stack.iter() {
    ///     sum += *element;
    /// }
    /// assert_eq!(sum, 30);
    /// ```
    pub fn iter(&self) -> StackIter<T> {
        StackIter {
            cursor: self.root.as_deref(),
        }
    }

    // pub fn iter_mut(&mut self) -> StackIterMut<T> {
    //     StackIterMut { cursor: self.root.as_deref_mut() }
    // }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIntoIter<T>;

    /// Converts the stack into an [`Iterator`]
    ///
    /// Example:
    /// ```
    /// let mut stack = Stack::<i32>::new();
    ///
    /// stack.push(10);
    /// stack.push(20);
    ///
    /// for _ in stack.into_iter() {
    ///     // do whatever
    /// }
    ///
    /// assert!(stack.is_empty());
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        StackIntoIter(self)
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        // Help the compiler drop the nodes in the order
        // in which they point to each other
        let mut root = self.root.take();
        while let Some(mut next) = root {
            root = next.next.take();
        }
    }
}

pub struct StackIntoIter<T>(Stack<T>);

impl<T> Iterator for StackIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct StackIter<'s, T> {
    cursor: Option<&'s Node<T>>,
}

impl<'s, T> Iterator for StackIter<'s, T> {
    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.map(|node| {
            self.cursor = node.next.as_deref();
            &node.data
        })
    }
}

// pub struct StackIterMut<'s, T> {
//     cursor: Option<&'s mut Node<T>>,
// }

// impl<'s, T> Iterator for StackIterMut<'s, T> {
//     type Item = &'s mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.cursor.as_mut().map(|node| {
//             self.cursor = node.next.as_deref_mut();
//             &mut node.data
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use crate::stack::*;

    #[test]
    fn push_pop() {
        let mut list = Stack::<i32>::new();
        list.push(10);
        list.push(20);
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = Stack::<i32>::new();
        list.push(10);
        list.push(20);
        assert_eq!(list.peek(), Some(&20));
        list.peek_mut().map(|data| *data = 15);
        assert_eq!(list.peek(), Some(&15));
    }

    #[test]
    fn iter() {
        let mut list = Stack::<i32>::new();
        list.push(10);
        list.push(20);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), None);
    }

    //     #[test]
    //     fn iter_mut() {
    //         let mut list = Stack::<i32>::new();
    //         list.push(10); list.push(20); list.push(30);

    //         let mut iter = list.iter_mut();
    //         assert_eq!(iter.next(), Some(&mut 3));
    //         assert_eq!(iter.next(), Some(&mut 2));
    //         assert_eq!(iter.next(), Some(&mut 1));
    //     }
}
