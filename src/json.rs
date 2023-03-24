use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::{debug, error};


/// QueryResultType is an enum that represents the type of a query result.
/// It can either be a success or an error.
///
/// # Examples
///
/// ```no_run
/// use crate::json::QueryResultType;
///
/// let success = QueryResultType::Success;
/// let error = QueryResultType::Error;
/// ```
///
/// # Values
///
/// [QueryResultType::Success](enum.QueryResultType.html#variant.Success)
///
/// [QueryResultType::Error](enum.QueryResultType.html#variant.Error)
#[derive(Debug)]
pub enum QueryResultType {
    /// Success represents a successful query result.
    Success,
    Error,
}

/// Implement PartialEq for QueryResultType.
/// This allows us to compare two QueryResultType values.
impl PartialEq for QueryResultType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (QueryResultType::Success, QueryResultType::Success) => true,
            (QueryResultType::Error, QueryResultType::Error) => true,
            _ => false,
        }
    }
}

/// Query represents a query to the database.
/// It contains the database, table, and content of the query.
/// The content is an optional string.
/// If the query is a read query, the content will be None.
/// If the query is a write query, the content will be Some(String).
///
/// # Examples
///
/// ```no_run
/// use crate::json::Query;
///
/// let query = Query::new("database".to_string(), "table".to_string(), None);
/// ```
///
/// # Values
///
/// [Query::new](struct.Query.html#method.new)
///
/// [Query::database](struct.Query.html#structfield.database)
///
/// [Query::table](struct.Query.html#structfield.table)
///
/// [Query::content](struct.Query.html#structfield.content)
///
pub struct Query {
    /// The database of the query.
    /// This is the name of the database that the query is being made to.
    pub database: String,
    /// The table of the query.
    /// This is the name of the table that the query is being made to.
    pub table: String,
    /// The content of the query.
    /// This is the data that is being written to the database.
    /// If the query is a read query, the content will be None.
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

/// QueryResult represents the result of a query.
/// It contains the status, message, and query of the result.
///
/// # Examples
///
/// ```no_run
/// use crate::json::{Query, QueryResult, QueryResultType};
///
/// let query = Query::new("database".to_string(), "table".to_string(), None);
/// let result = QueryResult::new(QueryResultType::Success, "query processed successfully".to_string(), query);
/// ```
///
/// # Values
///
/// [QueryResult::status](struct.QueryResult.html#structfield.status)
///
/// [QueryResult::message](struct.QueryResult.html#structfield.message)
///
/// [QueryResult::query](struct.QueryResult.html#structfield.query)
///
pub struct QueryResult {
    /// The status of the query result.
    /// This is either QueryResultType::Success or QueryResultType::Error.
    pub status: QueryResultType,
    /// The message of the query result.
    /// This is a string that describes the result of the query.
    /// If the query was successful, the message will be "query processed successfully".
    /// If the query was not successful, the message will be an error message.
    pub message: String,
    /// The query of the query result.
    /// This is the query that was made to the database.
    /// It contains the database, table, and content of the query.
    pub query: Query,
}

/// write is a function that writes a query to the database.
/// It takes a Query as an argument and returns a QueryResult.
///
/// # Examples
///
/// ```no_run
/// use crate::json::{Query, QueryResult, QueryResultType, write};
///
/// let query = Query::new("database".to_string(), "table".to_string(), Some("content".to_string()));
/// let result = write(&query);
/// ```
///
/// # Arguments
///
/// * `query` - A Query that is being written to the database.
///
/// # Returns
///
/// * `QueryResult` - A QueryResult that contains the result of the query.
///
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
            error!("query content is not valid json: {}", e);
            debug!("query content: {}", query.content.clone().unwrap());
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


/// read is a function that reads a query from the database.
/// It takes a Query as an argument and returns a QueryResult.
///
/// # Examples
///
/// ```no_run
/// use crate::json::{Query, QueryResult, QueryResultType, read};
///
/// let query = Query::new("database".to_string(), "table".to_string(), None);
/// let result = read(&query);
/// ```
///
/// # Arguments
///
/// * `query` - A Query that is being read from the database.
///
/// # Returns
///
/// * `QueryResult` - A QueryResult that contains the result of the query.
///
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