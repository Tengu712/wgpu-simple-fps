pub struct Cache<T>
where
    T: Clone,
{
    content: T,
    is_cached: bool,
}

impl<T> Cache<T>
where
    T: Clone,
{
    pub fn new(content: T) -> Self {
        Self {
            content,
            is_cached: false,
        }
    }

    pub fn get(&mut self) -> &mut T {
        self.is_cached = false;
        &mut self.content
    }

    pub fn cache(&mut self) -> Option<T> {
        if !self.is_cached {
            self.is_cached = true;
            Some(self.content.clone())
        } else {
            None
        }
    }
}
