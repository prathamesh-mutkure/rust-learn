use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
