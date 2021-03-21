#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Node {
    pub name: String,
    pub attrs: Vec<(String, String)>,
}

impl Node {
    pub fn new(a: &str) -> Node {
        Node {
            name: a.to_string(),
            ..Default::default()
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        let mut attrs: Vec<(String, String)> = attrs
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        self.attrs.append(&mut attrs);
        self
    }

    pub fn get_attr(&self, value: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|n| n.0 == value)
            .map(|n| n.1.as_str())
    }
}
