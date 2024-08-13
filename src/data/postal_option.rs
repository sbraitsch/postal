use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum PostalOption {
    Autoscroll,
    HttpOnly,
}

impl PostalOption {
    pub fn as_map() -> HashMap<PostalOption, (bool, &'static str)> {
        let mut map = HashMap::new();
        // Not useful atm since changing to most recent at top
        // map.insert(
        //     PostalOption::Autoscroll,
        //     (
        //         false,
        //         "The packet list will automatically scroll down to the newest packet.",
        //     ),
        // );
        map.insert(
            PostalOption::HttpOnly,
            (
                false,
                "Prefilters the received packets and discards them, if they are not HTTP(S).\n
                Takes effect when a new capture is started.",
            ),
        );
        map
    }
}

impl ToString for PostalOption {
    fn to_string(&self) -> String {
        match self {
            PostalOption::Autoscroll => "Autoscroll".to_string(),
            PostalOption::HttpOnly => "HTTP(S) only".to_string(),
        }
    }
}
