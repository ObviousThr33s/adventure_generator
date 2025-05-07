struct Queue {
    queue: Vec<Token>,
}

impl Queue {
    fn new() -> Queue {
        Queue {
            queue: Vec::new(),
        }
    }

    fn push(&mut self, token: Token) {
        self.queue.push(token);
    }

    fn pop(&mut self) -> Option<Token> {
        self.queue.pop()
    }

    fn peek(&self) -> Option<&Token> {
        self.queue.last()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn clear(&mut self) {
        self.queue.clear();
    }
}