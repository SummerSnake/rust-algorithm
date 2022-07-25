#[derive(Debug)]
pub struct PriorityQueue<T: PartialOrd + Clone> {
    pq: Vec<T>,
}

impl<T: PartialOrd + Clone> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue { pq: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.pq.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn enqueue(&mut self, value: T) {
        self.pq.push(value);
    }

    pub fn max_index(&self) -> usize {
        let mut max = 0;

        for i in 1..self.len() {
            if self.pq[max] < self.pq[i] {
                max = i;
            }
        }

        max
    }

    pub fn min_index(&self) -> usize {
        let mut min = 0;

        for i in 1..self.len() {
            if self.pq[i] < self.pq[min] {
                min = i;
            }
        }

        min
    }

    pub fn max(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let max = self.max_index();
        Some(self.pq[max].clone())
    }

    pub fn min(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let min = self.min_index();
        Some(self.pq[min].clone())
    }

    pub fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let max = self.max_index();
        Some(self.pq.remove(max).clone())
    }

    pub fn delete_min(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let min = self.min_index();
        Some(self.pq.remove(min).clone())
    }
}
