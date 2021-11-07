#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
        }
        self.data.pop()
    }

    pub fn top(&self) -> Option<T>
    where
        T: Copy,
    {
        if self.size == 0 {
            None
        } else {
            Some(self.data[self.size - 1])
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            self.data.get(self.size - 1)
        }
    }
}
