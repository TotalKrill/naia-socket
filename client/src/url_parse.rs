use url::Url;

pub fn get_url(url_str: &str) -> Url {
    let url = Url::parse(url_str).expect("server_session_url is not a valid URL!");
    if url.path_segments().is_some() {
        panic!("server_session_url must not include a path");
    }
    if url.query().is_some() {
        panic!("server_session_url must not include a query string");
    }
    if url.fragment().is_some() {
        panic!("server_session_url must not include a fragment");
    }

    url
}
