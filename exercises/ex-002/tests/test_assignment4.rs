use ex_002::assignment4::*;

#[test]
fn new_http_headers_should_be_empty() {
    let headers = HttpHeaders::new();
    assert!(headers.is_empty());
}

#[test]
fn get_should_retrieve_the_http_header_that_was_added_before() {
    let mut headers = HttpHeaders::new();
    headers.add("Content-Length", "42");

    assert_eq!(
        vec![String::from("42")],
        headers.get("Content-Length").unwrap()
    );
}

#[test]
fn get_should_retrieve_all_http_headers_of_the_same_name() {
    let mut headers = HttpHeaders::new();
    headers.add("Cookie-Set", "cookieValue1");
    headers.add("Cookie-Set", "cookieValue2");

    assert_eq!(
        vec![String::from("cookieValue1"), String::from("cookieValue2")],
        headers.get("Cookie-Set").unwrap()
    );
}

#[test]
fn iter_should_yield_all_headers_in_insertion_order() {
    let mut headers = HttpHeaders::new();
    headers.add("Content-Length", "42");
    headers.add("Cookie-Set", "cookieValue1");
    headers.add("Cookie-Set", "cookieValue2");

    assert_eq!(
        vec![
            (String::from("Content-Length"), String::from("42")),
            (String::from("Cookie-Set"), String::from("cookieValue1")),
            (String::from("Cookie-Set"), String::from("cookieValue2")),
        ],
        headers.iter().collect::<Vec<_>>()
    );
}

#[test]
fn test_to_string() {
    let mut headers = HttpHeaders::new();
    headers.add("Content-Length", "42");
    headers.add("Cookie-Set", "cookieValue1");
    headers.add("Cookie-Set", "cookieValue2");

    assert_eq!(
        "Content-Length: 42\r\nCookie-Set: cookieValue1\r\nCookie-Set: cookieValue2\r\n",
        headers.to_string()
    );
}

#[test]
fn test_iter_ref() {
    let mut headers = HttpHeaders::new();
    headers.add("Content-Length", "42");
    headers.add("Cookie-Set", "cookieValue1");
    headers.add("Cookie-Set", "cookieValue2");

    assert_eq!(
        vec![
            ("Content-Length", "42"),
            ("Cookie-Set", "cookieValue1"),
            ("Cookie-Set", "cookieValue2"),
        ],
        headers.iter_ref().collect::<Vec<_>>()
    );
}
