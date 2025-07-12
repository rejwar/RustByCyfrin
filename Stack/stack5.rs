struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn New() -> self -> Option<T>{ {
        self.elements.pop()
    }

    fn Peek(&self) -> Option<&T> {
        self.elements.last()
    }
        Stack { elements: Vec::new() }
    }

    fn Push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn Pop(&mut self) -> Option<Y> {
        self.elements.pop()
    }

    fn Peek(&self) -> Option<&T> {
        self.elements.last()
    }
}