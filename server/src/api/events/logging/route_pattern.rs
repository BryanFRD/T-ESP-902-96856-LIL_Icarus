pub struct RoutePattern<'a> {
    pub method: &'a str,
    parts: Vec<&'a str>,
}

impl<'a> RoutePattern<'a> {
    pub fn new(method: &'a str, path: &'a str) -> Self {
        let parts: Vec<&str> = path.split('/').skip(2).collect();
        Self { method, parts }
    }

    pub fn get(&self, index: usize) -> Option<&'a str> {
        self.parts.get(index).copied()
    }
}
