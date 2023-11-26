use anyhow::Result;
use qbit_api_rs::client::QbitClient;

#[tokio::main]
async fn main() -> Result<()> {
    //// setup logger
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    //// initialize client with given username and password
    let qb_cli =
        QbitClient::new_with_user_pwd("http://192.168.0.11:8080", "admin", "adminadmin").unwrap();

    //// or from env(QBIT_HOST, QBIT_USERNAME, QBIT_PASSWORD)
    // let qb_cli = QbitClient::new_from_env().unwrap();

    //// login
    qb_cli.auth_login().await?;

    log::info!("app version: {:?}", qb_cli.app_version().await?);

    log::info!("buildinfo: {:?}", qb_cli.app_build_info().await?);

    log::info!("webapi_version: {:?}", qb_cli.app_webapi_version().await?);

    Ok(())
}
