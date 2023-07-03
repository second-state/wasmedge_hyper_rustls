use hyper::Request;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let url = "https://eu.httpbin.org/get?msg=WasmEdge"
        .parse::<hyper::Uri>()
        .unwrap();
    fetch_https_url(url).await.unwrap();
}

async fn fetch_https_url(url: hyper::Uri) -> Result<()> {
    let https = wasmedge_hyper_rustls::connector::new_https_connector(
        wasmedge_rustls_api::ClientConfig::default(),
    );
    let client: hyper::client::Client<_, hyper::Body> =
        hyper::client::Client::builder().build(https);

    let authority = url.authority().unwrap().clone();
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(hyper::Body::empty())?;

    let res = client.request(req).await.unwrap();

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    println!("{}", String::from_utf8(body.into()).unwrap());

    println!("\n\nDone!");

    Ok(())
}
