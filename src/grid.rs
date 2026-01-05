pub struct Grid<T> {
    items: Vec<Vec<T>>,
}

impl<T: Clone> Grid<T> {
    pub fn create_uniform(width: usize, height: usize, item: T) -> Self {
        let items = vec![vec![item; width]; height];

        Grid { items }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.items[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, item: T) {
        self.items[y][x] = item;
    }
}
