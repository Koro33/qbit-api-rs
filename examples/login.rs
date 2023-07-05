use anyhow::Result;
use qbit_rs::client::QbitClient;

#[tokio::main]
async fn main() -> Result<()> {
    // setup logger
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // login info
    let host = "http://192.168.0.11:8080".to_string();
    let username = "admin".to_string();
    let password = "adminadmin".to_string();

    let qbit = QbitClient::new(host, username, password);

    // login
    qbit.auth_login().await?;

    let v = qbit.app_version().await?;
    log::info!("app version: {:?}", v);

    let buildinfo = qbit.app_build_info().await?;
    log::info!("buildinfo: {:?}", buildinfo);

    let wv = qbit.app_webapi_version().await?;
    log::info!("webapi_version: {:?}", wv);

    Ok(())
}
