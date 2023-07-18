use qbit_api_rs::client::QbitClient;
use qbit_api_rs::types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let qbit =
        QbitClient::new_with_user_pwd("http://192.168.0.11:8080", "admin", "adminadmin").unwrap();

    // test login
    let v = qbit.auth_login().await?;
    dbg!(v);

    let v = qbit.app_version().await?;
    dbg!(v);

    // // test torrents add
    // let magnet_url = "magnet:?xt=urn:btih:566946383b50d8970a36253621d70d21c9678b72&tr=http%3a%2f%2ft.nyaatracker.com%2fannounce&tr=http%3a%2f%2ftracker.kamigami.org%3a2710%2fannounce&tr=http%3a%2f%2fshare.camoe.cn%3a8080%2fannounce&tr=http%3a%2f%2fopentracker.acgnx.se%2fannounce&tr=http%3a%2f%2fanidex.moe%3a6969%2fannounce&tr=http%3a%2f%2ft.acg.rip%3a6699%2fannounce&tr=https%3a%2f%2ftr.bangumi.moe%3a9696%2fannounce&tr=udp%3a%2f%2ftr.bangumi.moe%3a6969%2fannounce&tr=http%3a%2f%2fopen.acgtracker.com%3a1096%2fannounce&tr=udp%3a%2f%2ftracker.opentrackr.org%3a1337%2fannounce";
    // let v = qbit.torrents_add_by_url(&[magnet_url]).await?;
    // dbg!(v);

    // let torrent_file = r"C:\Users\ycg10\Desktop\ubuntu-22.04.2-live-server-amd64.iso.torrent";
    // let v = qbit.torrents_add_by_file(&[torrent_file]).await?;
    // dbg!(v);

    // let a = types::State::CheckingDL;
    // serde_json::to_writer(std::io::stdout(), &a)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn test_raw_response() {
        let qbit = QbitClient::new(
            "http://192.168.0.11:8080".to_string(),
            "admin".to_string(),
            "adminadmin".to_string(),
        );
        qbit.auth_login().await.unwrap();
        // hash=a585051959d4e06e71da2f4303547a08348e5d34
        let relative = "/api/v2/app/shutdown";
        let url = qbit.host.join(relative).unwrap();
        let mut request = qbit.client.request(reqwest::Method::POST, url);
        // // pause form
        // request = request.form(&f);
        let res = request.send().await.unwrap();
        // println!("{:#?}", res);
        // let res = res.json::<serde_json::Value>().await.unwrap();
        // dbg!(res);
        // std::fs::write("test.json", res.to_string()).unwrap();
        let t = res.text().await.unwrap();

        // let t = serde_json::from_str::<types::TorrentsTagsResponse>(&t).unwrap();
        dbg!(t);
    }
}
