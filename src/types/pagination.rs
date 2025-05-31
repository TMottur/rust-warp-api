use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is extracted from query params
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item that should be returned
    pub start: usize,
    /// The index of the last item to be returned
    pub end: usize,
}

/// Extract query parameters from the '/questions' route
/// # Example query:
/// GET requests to this route can have a pagination attached so we just return the questions we need
/// '/questions?start=1&end=50'
/// # Example usage
/// '''rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// '''
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improved in the future
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            // Takes the start parameter and tries converting to a usize
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            // Takes the end parameter, tries converting to usize
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
