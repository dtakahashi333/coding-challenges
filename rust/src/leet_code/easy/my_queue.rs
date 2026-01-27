// rust/src/leet_code/easy/my_queue.rs

// 232. Implement Queue using Stacks
// https://leetcode.com/problems/implement-queue-using-stacks/description/

use std::cell::RefCell;

#[derive(Debug)]
pub struct MyQueue {
    v1: RefCell<Vec<i32>>,
    v2: RefCell<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            v1: RefCell::new(Vec::new()),
            v2: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, x: i32) {
        if self.v1.borrow().is_empty() {
            self.v2.borrow_mut().push(x);
        } else {
            self.v1.borrow_mut().push(x);
        }
    }

    pub fn pop(&self) -> i32 {
        let mut active;
        let mut spare;
        if self.v1.borrow().is_empty() {
            active = self.v2.borrow_mut();
            spare = self.v1.borrow_mut();
        } else {
            active = self.v1.borrow_mut();
            spare = self.v2.borrow_mut();
        }
        while !active.is_empty() {
            spare.push(active.pop().unwrap());
        }
        let first = spare.pop().unwrap();
        while !spare.is_empty() {
            active.push(spare.pop().unwrap());
        }
        first
    }

    pub fn peek(&self) -> i32 {
        if let Some(first) = self.v1.borrow().first() {
            *first
        } else {
            *self.v2.borrow().first().unwrap()
        }
    }

    pub fn empty(&self) -> bool {
        self.v1.borrow().is_empty() && self.v2.borrow().is_empty()
    }
}

#[derive(Debug)]
pub struct MyQueue2 {
    input: Vec<i32>,
    output: Vec<i32>,
}

/**
 * Canonical solution
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue2 {
    pub fn new() -> Self {
        MyQueue2 {
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.input.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.output.is_empty() {
            while let Some(v) = self.input.pop() {
                self.output.push(v);
            }
        }
        self.output.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        if self.output.is_empty() {
            while let Some(v) = self.input.pop() {
                self.output.push(v);
            }
        }
        *self.output.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }
}

// /**
//  * Your MyQueue object will be instantiated and called as such:
//  * let obj = MyQueue::new();
//  * obj.push(x);
//  * let ret_2: i32 = obj.pop();
//  * let ret_3: i32 = obj.peek();
//  * let ret_4: bool = obj.empty();
//  */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert_eq!(obj.empty(), false);
    }

    #[test]
    fn test2() {
        let obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        obj.push(3);
        obj.push(4);
        assert_eq!(obj.pop(), 1);
        obj.push(5);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.pop(), 3);
        assert_eq!(obj.pop(), 4);
        assert_eq!(obj.pop(), 5);
    }
    #[test]
    fn test3() {
        let mut obj = MyQueue2::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert_eq!(obj.empty(), false);
    }

    #[test]
    fn test4() {
        let mut obj = MyQueue2::new();
        obj.push(1);
        obj.push(2);
        obj.push(3);
        obj.push(4);
        assert_eq!(obj.pop(), 1);
        obj.push(5);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.pop(), 3);
        assert_eq!(obj.pop(), 4);
        assert_eq!(obj.pop(), 5);
    }
}
