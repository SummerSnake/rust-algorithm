#[derive(Debug)]
pub struct Queue<T> {
    qdata: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { qdata: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.qdata.push(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let len = self.qdata.len();

        if len > 0 {
            let val = self.qdata.remove(0);
            Some(val)
        } else {
            None
        }
    }
}
