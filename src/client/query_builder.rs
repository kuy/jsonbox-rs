use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::de::DeserializeOwned;
use std::fmt::Display;

use crate::client::{Client, Meta};
use crate::error::Result;

#[derive(Clone)]
enum Order<'a> {
    Asc(&'a str),
    Desc(&'a str),
}

pub struct QueryBuilder<'a> {
    client: &'a Client<'a>,
    sort: Order<'a>,
    skip: u32,
    limit: u32,
    q: Vec<String>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(client: &'a Client) -> QueryBuilder<'a> {
        QueryBuilder {
            client,
            sort: Order::Desc("_createdOn".into()),
            skip: 0,
            limit: 20,
            q: vec![],
        }
    }

    /// Set the field for sorting.
    pub fn order_by<'q>(&'q mut self, field: &'a str) -> &'q mut QueryBuilder<'a> {
        self.sort = Order::Asc(field);
        self
    }

    /// Set reverse order. Use this with `order_by` method.
    pub fn desc<'q>(&'q mut self) -> &'q mut QueryBuilder<'a> {
        let field = match self.sort {
            Order::Asc(f) => f,
            Order::Desc(f) => f,
        };
        self.sort = Order::Desc(field);
        self
    }

    /// Limit the number of records of query result.
    pub fn limit<'q>(&'q mut self, limit: u32) -> &'q mut QueryBuilder<'a> {
        self.limit = limit;
        self
    }

    /// Specify the number of records to skip.
    pub fn skip<'q>(&'q mut self, skip: u32) -> &'q mut QueryBuilder<'a> {
        self.skip = skip;
        self
    }

    /// Set filter option, whkch is mapped `q` parameter in REST API.
    pub fn filter_by<'q, T: Display>(
        &'q mut self,
        format: &str,
        value: T,
    ) -> &'q mut QueryBuilder<'a> {
        let value = utf8_percent_encode(&format!("{}", value), NON_ALPHANUMERIC).to_string();
        self.q.push(format.replace("{}", &value));
        self
    }

    /// Alias of `filter_by`.
    pub fn and<'q, T: Display>(&'q mut self, format: &str, value: T) -> &'q mut QueryBuilder<'a> {
        self.filter_by(format, value)
    }

    /// Get a single record by id.
    pub fn id<T>(&self, id: &str) -> Result<(T, Meta)>
    where
        T: DeserializeOwned,
    {
        self.client.read_by_id(id)
    }

    /// Get all records with default query parameters.
    pub fn all<T>(&self) -> Result<Vec<(T, Meta)>>
    where
        T: DeserializeOwned,
    {
        let default = QueryBuilder::new(self.client);
        self.client.read_by_query(&default)
    }

    /// Run query with configured query parameters.
    pub fn run<T>(&self) -> Result<Vec<(T, Meta)>>
    where
        T: DeserializeOwned,
    {
        self.client.read_by_query(self)
    }

    /// Generate query string.
    pub fn to_string(&self) -> String {
        let mut query = format!(
            "sort={}&skip={}&limit={}",
            self.sort_string(),
            self.skip,
            self.limit
        );
        if self.q.len() > 0 {
            query = format!("{}&q={}", query, self.filter_string());
        }
        query
    }

    fn sort_string(&self) -> String {
        match &self.sort {
            Order::Asc(field) => field.to_string(),
            Order::Desc(field) => format!("-{}", field),
        }
    }

    fn filter_string(&self) -> String {
        let mut filter = self
            .q
            .iter()
            .fold(String::new(), |acc, q| format!("{}{},", acc, q));
        filter.pop();
        filter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let c = Client::new("xxx");
        assert_eq!(
            QueryBuilder::new(&c)
                .order_by("count")
                .desc()
                .limit(42)
                .skip(8)
                .filter_by("count:>{}", 20)
                .and("count:<{}", 40)
                .to_string(),
            "sort=-count&skip=8&limit=42&q=count:>20,count:<40"
        );
    }

    #[test]
    fn test_sort_string() {
        let c = Client::new("xxx");
        let mut q = QueryBuilder::new(&c);
        assert_eq!(q.sort_string(), "-_createdOn");

        q.order_by("name");
        assert_eq!(q.sort_string(), "name");

        q.desc();
        assert_eq!(q.sort_string(), "-name");
    }

    #[test]
    fn test_filter_string() {
        let c = Client::new("xxx");
        let mut q = QueryBuilder::new(&c);
        assert_eq!(q.filter_string(), "");

        q.filter_by("name:{}", "foo bar");
        assert_eq!(q.filter_string(), "name:foo%20bar");

        q.filter_by("city:{}*", "Los ");
        assert_eq!(q.filter_string(), "name:foo%20bar,city:Los%20*");

        q.filter_by("count:<{}", 42);
        assert_eq!(q.filter_string(), "name:foo%20bar,city:Los%20*,count:<42");

        q.filter_by("login:{}", true);
        assert_eq!(
            q.filter_string(),
            "name:foo%20bar,city:Los%20*,count:<42,login:true"
        );
    }
}
