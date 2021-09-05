pub struct Query {
    value: String,
}

pub trait QueryTrait {
    fn get_properties(&self) -> Vec<&str>;
    fn get_last_properties(&self) -> &str;
}

impl Query {
    pub fn new(value: String) -> Query {
        Query { value }
    }
}

impl QueryTrait for Query {
    fn get_properties(&self) -> Vec<&str> {
        self.value.split(".").skip(1).collect::<Vec<&str>>()
    }

    fn get_last_properties(&self) -> &str {
        match self.get_properties().last() {
            Some(v) => v,
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::query::QueryTrait;

    use super::Query;

    #[test]
    fn test_get_properties() {
        let query1 = Query::new(".abcd.aaaa".into());
        let query2 = Query::new(".abcd.aaaa.uuu".into());
        assert_eq!(query1.get_properties(), vec!["abcd", "aaaa"]);
        assert_eq!(query2.get_properties(), vec!["abcd", "aaaa", "uuu"]);
    }

    #[test]
    fn test_get_last_properties() {
        let query1 = Query::new(".abcd.aaaa".into());
        let query2 = Query::new(".abcd.aaaa.uuu".into());
        assert_eq!(query1.get_last_properties(), "aaaa");
        assert_eq!(query2.get_last_properties(), "uuu");
    }
}
