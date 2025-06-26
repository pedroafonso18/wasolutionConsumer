struct Request {
    action: String,
    method: String,
    url: String,
    headers: String,
    body: Option<String>,
    params: Option<String>
}