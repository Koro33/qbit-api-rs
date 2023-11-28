use anyhow::Result;
use qbit_api_rs::client::QbitClient;
use qbit_api_rs::types;

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

    //// add torrent by url
    let torrent_urls = [
        "magnet:?xt=urn:btih:TZRYKYVLDQP45WO66FBIMTG5LJYBTYNK&dn=ubuntu-22.04.2-live-server-amd64.iso&xl=1975971840&tr=https%3A%2F%2Ftorrent.ubuntu.com%2Fannounce"
        // ...
    ];
    let _ = qb_cli.torrents_add_by_url(&torrent_urls).await?;
    torrent_urls
        .iter()
        .for_each(|url| log::info!("torrent added by url: {:?}", url));

    //// add torrent by torrent file
    let torrent_files = [
        "tests/ubuntu-22.04.2-live-server-amd64.iso.torrent",
        // ...
    ];
    let _ = qb_cli.torrents_add_by_file(&torrent_files).await?;
    torrent_files.iter().for_each(|f| {
        log::info!(
            "torrent added by file: {:?}",
            std::path::Path::new(f).file_name().unwrap()
        )
    });

    //// list torrents
    let q = types::TorrentsInfoQuery {
        filter: Some(types::TorrentsInfoFilter::All),
        sort: Some(types::TorrentsInfoSort::Name),
        reverse: Some(true),
        // ...
        ..Default::default()
    };
    for torrent in qb_cli.torrents_info(&q).await?.data {
        log::info!(
            "{:?} - {:?} - {:?}",
            torrent.state,
            torrent.name,
            torrent.hash
        );
    }

    //// pause torrents
    let torrent_hashes = [
        "9e638562ab1c1fced9def142864cdd5a7019e1aa".to_owned(),
        // ...
    ];
    let _ = qb_cli.torrents_pause(&torrent_hashes).await?;
    torrent_hashes
        .iter()
        .for_each(|h| log::info!("torrent paused: {:?}", h));

    //// resume torrents
    let torrent_hashes = [
        "9e638562ab1c1fced9def142864cdd5a7019e1aa".to_owned(),
        // ...
    ];
    let _ = qb_cli.torrents_resume(&torrent_hashes).await?;
    torrent_hashes
        .iter()
        .for_each(|h| log::info!("torrent resumed: {:?}", h));

    //// delete torrents
    let torrent_hashes = [
        "9e638562ab1c1fced9def142864cdd5a7019e1aa".to_owned(),
        // ...
    ];
    let _ = qb_cli.torrents_delete(&torrent_hashes, true).await?;
    torrent_hashes
        .iter()
        .for_each(|h| log::info!("torrent deleted: {:?}", h));

    Ok(())
}
