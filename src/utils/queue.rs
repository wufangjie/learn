use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T> {
    size: usize,
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            size: 0,
            data: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, item: T) {
        self.data.push_back(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
        }
        self.data.pop_front()
    }
}
