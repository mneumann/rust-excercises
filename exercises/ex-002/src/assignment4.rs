pub struct HttpHeaders;

impl HttpHeaders {
    /// Creates an empty set of HTTP headers
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Returns `true` if `HttpHeaders` contains no headers at all.
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    /// Adds a HTTP header. This will not overwrite an existing header of the same name.
    pub fn add(&mut self, header_name: &str, header_value: &str) {
        unimplemented!()
    }

    /// Returns all HTTP header values of name `header_name`.
    pub fn get(&self, header_name: &str) -> Option<Vec<String>> {
        unimplemented!()
    }

    /// Returns an `Iterator` that iterates over the key/value pairs.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (String, String)> + 'a {
        // FIXME
        std::iter::empty()
    }

    /// Returns an `Iterator` that iterates over the key/value pairs as string slices.
    ///
    /// Non-owning version of `iter`.
    pub fn iter_ref<'a>(&'a self) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
        // FIXME
        std::iter::empty()
    }

    /// Render `HttpHeaders` as String.
    pub fn to_string(&self) -> String {
        unimplemented!()
    }
}
