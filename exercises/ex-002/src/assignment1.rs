pub struct HttpHeaders(Vec<(String, String)>);

impl HttpHeaders {
    /// Creates an empty set of HTTP headers
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns `true` if `HttpHeaders` contains no headers at all.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Adds a HTTP header. This will not overwrite an existing header of the same name.
    pub fn add(&mut self, header_name: &str, header_value: &str) {
        self.0.push((header_name.into(), header_value.into()));
    }

    /// Returns all HTTP header values of name `header_name`.
    pub fn get(&self, header_name: &str) -> Option<Vec<String>> {
        let values: Vec<String> = self
            .0
            .iter()
            .filter(|header| header.0 == header_name)
            .map(|header| header.1.clone())
            .collect();
        if values.is_empty() {
            None
        } else {
            Some(values)
        }
    }
}
