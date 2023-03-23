use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::debug;


#[derive(Debug)]
pub enum QueryResultType {
    Success,
    Error,
}

impl PartialEq for QueryResultType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (QueryResultType::Success, QueryResultType::Success) => true,
            (QueryResultType::Error, QueryResultType::Error) => true,
            _ => false,
        }
    }
}

pub struct Query {
    pub database: String,
    pub table: String,
    pub content: Option<String>,
}

#[allow(dead_code)]
impl Query {
    pub fn new(database: String, table: String, content: Option<String>) -> Query {
        Query {
            database,
            table,
            content,
        }
    }
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "database: {}, table: {}, content: {:?}", self.database, self.table, self.content)
    }
}

pub struct QueryResult {
    pub status: QueryResultType,
    pub message: String,
    pub query: Query,
}

pub fn write(query: &Query) -> QueryResult {
    let mut result = QueryResult {
        status: QueryResultType::Success,
        message: "query processed successfully".to_string(),
        query: Query {
            database: query.database.clone(),
            table: query.table.clone(),
            content: query.content.clone(),
        },
    };

    if !std::path::Path::new("db").exists() {
        std::fs::create_dir("db").unwrap();
    }
    if !std::path::Path::new(&format!("db/{}", query.database)).exists() {
        std::fs::create_dir(format!("db/{}", query.database)).unwrap();
    }
    if !std::path::Path::new(&format!("db/{}/{}.json", query.database, query.table)).exists() {
        File::create(format!("db/{}/{}.json", query.database, query.table)).unwrap();
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("db/{}/{}.json", query.database, query.table))
        .unwrap();

    if query.content.is_none() {
        result.status = QueryResultType::Error;
        result.message = "query content is empty".to_string();
        return result;
    }

    let query_json: serde_json::Value = match serde_json::from_str(
        match &query.content.clone() {
        Some(content) => content,
        None => "",
    }) {
        Ok(value) => value,
        Err(e) => {
            result.status = QueryResultType::Error;
            result.message = format!("query content is not valid json: {}", e);
            return result;
        }
    };
    let json = match serde_json::to_string(&query_json) {
        Ok(json) => json,
        Err(e) => {
            result.status = QueryResultType::Error;
            result.message = format!("query content is not valid json: {}", e);
            return result;
        }
    };

    match file.write_all(json.as_bytes()) {
        Err(e) => {
            debug!("Error: {}", e);
        }
        Ok(_) => {
            debug!("Query processed successfully");
        }
    }

    result
}

pub fn read(query: &Query) -> QueryResult {
    //check if table exists
    if !std::path::Path::new(&format!("db/{}", query.database)).exists() {
        let result = QueryResult {
            status: QueryResultType::Error,
            message: "database does not exist".to_string(),
            query: Query {
                database: query.database.clone(),
                table: query.table.clone(),
                content: query.content.clone(),
            },
        };
        return result;
    }
    if !std::path::Path::new(&format!("db/{}/{}.json", query.database, query.table)).exists() {
        let result = QueryResult {
            status: QueryResultType::Error,
            message: "table does not exist".to_string(),
            query: Query {
                database: query.database.clone(),
                table: query.table.clone(),
                content: query.content.clone(),
            },
        };
        return result;
    }

    let mut file = match OpenOptions::new()
        .read(true)
        .open(format!("db/{}/{}.json", query.database, query.table)) {
            Ok(file) => file,
            Err(e) => {
                let result = QueryResult {
                    status: QueryResultType::Error,
                    message: format!("error opening file: {}", e),
                    query: Query {
                        database: query.database.clone(),
                        table: query.table.clone(),
                        content: query.content.clone(),
                    },
                };
                return result;
            }
    };


    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            debug!("File read successfully");
        }
        Err(e) => {
            let result = QueryResult {
                status: QueryResultType::Error,
                message: format!("error reading file: {}", e),
                query: Query {
                    database: query.database.clone(),
                    table: query.table.clone(),
                    content: query.content.clone(),
                },
            };
            return result;
        }
    }

    let result = QueryResult {
        status: QueryResultType::Success,
        message: "query processed successfully".to_string(),
        query: Query {
            database: query.database.clone(),
            table: query.table.clone(),
            content: Some(contents),
        },
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let query = Query {
            database: "test".to_string(),
            table: "users".to_string(),
            content: Some(r#"{"first_name": "John","last_name": "Doe","age": 25}"#.to_string()),
        };

        let result = write(&query);

        assert_eq!(result.status, QueryResultType::Success);
        assert_eq!(result.message, "query processed successfully");
        assert_eq!(result.query.database, "test");
        assert_eq!(result.query.table, "users");
    }

    #[test]
    fn test_read() {
        let query = Query {
            database: "test".to_string(),
            table: "users".to_string(),
            content: None,
        };

        let result = read(&query);

        assert_eq!(result.status, QueryResultType::Success);
        assert_eq!(result.message, "query processed successfully");
        assert_eq!(result.query.database, "test");
        assert_eq!(result.query.table, "users");
        assert_eq!(result.query.content, Some(r#"{"age":25,"first_name":"John","last_name":"Doe"}"#.to_string()));

        let query = Query {
            database: "test".to_string(),
            table: "users2".to_string(),
            content: None,
        };

        let result = read(&query);

        assert_eq!(result.status, QueryResultType::Error);
        assert_eq!(result.message, "table does not exist");
        assert_eq!(result.query.database, "test");
        assert_eq!(result.query.table, "users2");
        assert_eq!(result.query.content, None);
    }
}