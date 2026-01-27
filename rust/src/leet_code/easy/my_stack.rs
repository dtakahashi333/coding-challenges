// rust/src/leet_code/easy/my_stack.rs

// 225. Implement Stack using Queues
// https://leetcode.com/problems/implement-stack-using-queues/description/

use std::{cell::RefCell, collections::VecDeque};

pub struct MyStack {
    q1: RefCell<VecDeque<i32>>,
    q2: RefCell<VecDeque<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    pub fn new() -> Self {
        MyStack {
            q1: RefCell::new(VecDeque::new()),
            q2: RefCell::new(VecDeque::new()),
        }
    }

    pub fn push(&self, x: i32) {
        if self.q1.borrow().is_empty() {
            self.q2.borrow_mut().push_back(x);
        } else {
            self.q1.borrow_mut().push_back(x);
        }
    }

    pub fn pop(&self) -> i32 {
        let mut active;
        let mut spare;
        if self.q1.borrow().is_empty() {
            active = self.q2.borrow_mut();
            spare = self.q1.borrow_mut();
        } else {
            active = self.q1.borrow_mut();
            spare = self.q2.borrow_mut();
        }
        while active.len() > 1 {
            if let Some(v) = active.pop_front() {
                spare.push_back(v);
            }
        }
        active.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        if let Some(v) = self.q1.borrow().back() {
            *v
        } else {
            *self.q2.borrow().back().unwrap()
        }
    }

    pub fn empty(&self) -> bool {
        self.q1.borrow().is_empty() && self.q2.borrow().is_empty()
    }
}

pub struct MyStack2 {
    q: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack2 {
    pub fn new() -> Self {
        MyStack2 { q: VecDeque::new() }
    }

    pub fn push(&mut self, x: i32) {
        let mut spare = VecDeque::new();
        spare.push_back(x);
        while let Some(v) = self.q.pop_front() {
            spare.push_back(v);
        }
        self.q = spare;
    }

    pub fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.empty(), false);
    }

    #[test]
    fn test2() {
        let mut obj = MyStack2::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert_eq!(obj.empty(), false);
    }
}
