use anyhow::Result;
use qbit_api_rs::client::QbitClient;

#[tokio::main]
async fn main() -> Result<()> {
    // setup logger
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // initialize client with given username and password
    let qbit =
        QbitClient::new_with_user_pwd("http://192.168.0.11:8080", "admin", "adminadmin").unwrap();

    // or from env(QBIT_HOST, QBIT_USERNAME, QBIT_PASSWORD)
    // let qbit = QbitClient::new_from_env().unwrap();

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
