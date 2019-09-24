use serde::de::DeserializeOwned;

use crate::client::{Client, Meta};
use crate::error::Result;

#[derive(Clone)]
pub enum Order {
    Asc(String),
    Desc(String),
}

pub struct QueryBuilder<'a> {
    client: &'a Client,
    sort: Order,
    skip: u32,
    limit: u32,
    q: Option<String>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(client: &'a Client) -> QueryBuilder<'a> {
        QueryBuilder {
            client,
            sort: Order::Desc("_createdOn".into()),
            skip: 0,
            limit: 20,
            q: None,
        }
    }

    pub fn order_by(&self, field: &str) -> QueryBuilder {
        QueryBuilder {
            client: self.client,
            sort: Order::Asc(field.to_string()),
            skip: self.skip,
            limit: self.limit,
            q: self.q.clone(),
        }
    }

    pub fn desc(&self) -> QueryBuilder {
        let field = match &self.sort {
            Order::Asc(f) => f,
            Order::Desc(f) => f,
        };
        QueryBuilder {
            client: self.client,
            sort: Order::Desc(field.clone()),
            skip: self.skip,
            limit: self.limit,
            q: self.q.clone(),
        }
    }

    pub fn limit(&self, limit: u32) -> QueryBuilder {
        QueryBuilder {
            client: self.client,
            sort: self.sort.clone(),
            skip: self.skip,
            limit,
            q: self.q.clone(),
        }
    }

    pub fn skip(&self, skip: u32) -> QueryBuilder {
        QueryBuilder {
            client: self.client,
            sort: self.sort.clone(),
            skip,
            limit: self.limit,
            q: self.q.clone(),
        }
    }

    pub fn id<T>(&self, id: &str) -> Result<(T, Meta)>
    where
        T: DeserializeOwned,
    {
        self.client.read_by_id(id)
    }

    pub fn all<T>(&self) -> Result<Vec<(T, Meta)>>
    where
        T: DeserializeOwned,
    {
        let default = QueryBuilder::new(self.client);
        self.client.read_by_query(&default)
    }

    pub fn run<T>(&self) -> Result<Vec<(T, Meta)>>
    where
        T: DeserializeOwned,
    {
        self.client.read_by_query(self)
    }

    pub fn to_string(&self) -> String {
        format!(
            "sort={}&skip={}&limit={}",
            self.sort_string(),
            self.skip,
            self.limit
        )
    }

    fn sort_string(&self) -> String {
        match &self.sort {
            Order::Asc(field) => field.to_string(),
            Order::Desc(field) => format!("-{}", field),
        }
    }
}
