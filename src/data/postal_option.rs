use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum PostalOption {
    Autoscroll,
}

impl PostalOption {
    pub fn as_map() -> HashMap<PostalOption, bool> {
        let mut map = HashMap::new();
        map.insert(PostalOption::Autoscroll, false);
        map
    }
}

impl ToString for PostalOption {
    fn to_string(&self) -> String {
        match self {
            PostalOption::Autoscroll => "Autoscroll".to_string(),
        }
    }
}
