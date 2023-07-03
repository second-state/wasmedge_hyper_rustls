use hyper::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = wasmedge_hyper_rustls::connector::new_https_connector(
        wasmedge_rustls_api::ClientConfig::default(),
    );
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get("https://httpbin.org/get?id=1".parse()?).await?;
    assert_eq!(res.status(), 200);
    Ok(())
}
