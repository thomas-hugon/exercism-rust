#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Edge{
    pub a: String,
    pub b: String,
    pub attrs: Vec<(String, String)>
}

impl Edge {
    pub fn new(a: &str, b: &str) -> Edge {
        Edge{
            a: a.to_string(),
            b: b.to_string(),
            ..Default::default()
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        let mut attrs: Vec<(String, String)> = attrs.iter().map(|(a,b)|(a.to_string(), b.to_string())).collect();
        self.attrs.append(&mut attrs);
        self
    }
}
