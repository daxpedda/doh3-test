use futures::future;
use h3::client;
use h3_quinn::{
    quinn::{self, Endpoint},
    Connection,
};
use http::{Request, Version};
use quinn::ClientConfig;
use rustls::{version::TLS13, ClientConfig as TlsClientConfig, RootCertStore};
use std::{env, error::Error, io, net::Ipv4Addr, str::FromStr, sync::Arc};
use tokio::io::AsyncWriteExt;
use tracing::{error, info, warn, Level};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
use trust_dns_proto::{
    op::{Message, Query},
    rr::{Name, RecordType},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::FULL)
        .with_writer(io::stderr)
        .with_max_level(Level::INFO)
        .init();

    let address: Ipv4Addr = env::args().nth(1).unwrap().parse()?;

    let mut roots = RootCertStore::empty();
    let certs = rustls_native_certs::load_native_certs()?;
    let (added, ignored) = roots.add_parsable_certificates(&certs);
    if ignored != 0 {
        warn!("failed to parse {ignored} trust anchor(s)");
    }
    if added == 0 {
        error!("couldn't load any default trust roots");
    }

    let mut tls_config = TlsClientConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&TLS13])?
        .with_root_certificates(roots)
        .with_no_client_auth();

    tls_config.enable_early_data = true;
    tls_config.alpn_protocols = vec!["h3".into()];

    let mut client_endpoint = Endpoint::client("[::]:0".parse()?)?;

    let client_config = ClientConfig::new(Arc::new(tls_config));
    client_endpoint.set_default_client_config(client_config);

    let conn = client_endpoint
        .connect((address, 443).into(), &address.to_string())?
        .await?;
    let quinn_conn = Connection::new(conn);

    let (mut driver, mut send_request) = client::new(quinn_conn).await?;

    let drive = async move {
        future::poll_fn(|cx| driver.poll_close(cx)).await?;
        Ok::<_, Box<dyn Error>>(())
    };

    let request = async move {
        let mut message = Message::new();
        let query = Query::query(Name::from_str("www.example.com.")?, RecordType::A);
        message.add_query(query);

        info!("request message:\n{message:#}");

        let data = message.to_vec()?;

        let req = Request::builder()
            .method("POST")
            .version(Version::HTTP_3)
            .uri(format!("https://{address}/dns-query"))
            .header("content-type", "application/dns-message")
            .header("content-length", data.len())
            .body(())?;

        info!("{:#?}", req);

        let mut stream = send_request.send_request(req).await?;
        stream.send_data(data.into()).await?;
        stream.finish().await?;

        let resp = stream.recv_response().await?;

        info!("response: {:?} {}", resp.version(), resp.status());
        info!("response headers: {:#?}", resp.headers());

        let mut data = Vec::new();
        while let Some(mut chunk) = stream.recv_data().await? {
            data.write_all_buf(&mut chunk).await?;
        }

        if resp.status().is_success() {
            let message = Message::from_vec(&data)?;
            info!("response message:\n{message:#}");
        } else {
            info!("response text: {}", String::from_utf8(data)?);
        }

        Ok::<_, Box<dyn Error>>(())
    };

    let (req_res, drive_res) = tokio::join!(request, drive);
    req_res?;
    drive_res?;

    client_endpoint.wait_idle().await;

    Ok(())
}
