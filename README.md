# qbit-api-rs

[![GitHub release (release name instead of tag name)](https://img.shields.io/github/v/release/koro33/qbit-api-rs)](https://github.com/Koro33/qbit-api-rs/releases) [![Rust](https://img.shields.io/badge/Rust-stable-brightgreen)](https://www.rust-lang.org/) [![GitHub](https://img.shields.io/github/license/koro33/qbit-api-rs)](./LICENSE-APACHE) [![GitHub Repo stars](https://img.shields.io/github/stars/koro33/qbit-api-rs?style=social)](https://github.com/Koro33/qbit-api-rs)

A asynchronous Rust wrapper for qBittorrent [Web API](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1)) (For version above 4.1).

## Note

- This crate provides only the pure API bindings. There is no such mechanism like reauthentication when the token expires. You should implement them by yourself.

- The qBitTorrent uses cookie to authenticate. When call login method, the SID token will be updated, but will be expired after a while(default 3600 seconds). This expired time can be configured in the `Options -> WebUI -> Authentication -> Session timeout`. you can either periodically(within the timeout period) call the login method to reauthenticate. Or just enable `Bypass authentication for clients in whitelisted IP subnets` and configure your IP subnets, then you don't need to call login method to authenticate.

## Supported Method

### Authentication

- [x] Login
- [x] Logout

### Application

- [x] Get application version
- [x] Get API version
- [x] Get build info
- [x] Shutdown application
- [x] Get application preferences
- [x] Set application preferences
- [x] Get default save path

### Log

- [x] Get log
- [x] Get peer log

### Sync

- [x] Get main data
- [x] Get torrent peers data

### Transfer info

- [x] Get global transfer info
- [x] Get alternative speed limits state
- [x] Toggle alternative speed limits
- [x] Get global download limit
- [x] Set global download limit
- [x] Get global upload limit
- [x] Set global upload limit
- [x] Ban peers

### Torrent management

- [x] Get torrent list
- [x] Get torrent generic properties
- [x] Get torrent trackers
- [x] Get torrent web seeds
- [x] Get torrent contents
- [x] Get torrent pieces' states
- [x] Get torrent pieces' hashes
- [x] Pause torrents
- [x] Resume torrents
- [x] Delete torrents
- [x] Recheck torrents
- [x] Reannounce torrents
- [x] Edit trackers
- [x] Remove trackers
- [x] Add peers
- [x] Add new torrent
- [x] Add trackers to torrent
- [x] Increase torrent priority
- [x] Decrease torrent priority
- [x] Maximal torrent priority
- [x] Minimal torrent priority
- [ ] Set file priority
- [x] Get torrent download limit
- [x] Set torrent download limit
- [x] Set torrent share limit
- [x] Get torrent upload limit
- [x] Set torrent upload limit
- [x] Set torrent location
- [x] Set torrent name
- [x] Set torrent category
- [x] Get all categories
- [x] Add new category
- [x] Edit category
- [x] Remove categories
- [x] Add torrent tags
- [x] Remove torrent tags
- [x] Get all tags
- [x] Create tags
- [x] Delete tags
- [x] Set automatic torrent management
- [x] Toggle sequential download
- [x] Set first/last piece priority
- [x] Set force start
- [x] Set super seeding
- [x] Rename file
- [x] Rename folder

### RSS (experimental)

- [ ] Add folder
- [ ] Add feed
- [ ] Remove item
- [ ] Move item
- [ ] Get all items
- [ ] Mark as read
- [ ] Refresh item
- [ ] Set auto-downloading rule
- [ ] Rename auto-downloading rule
- [ ] Remove auto-downloading rule
- [ ] Get all auto-downloading rules
- [ ] Get all articles matching a rule

### Search

- [ ] Start search
- [ ] Stop search
- [ ] Get search status
- [ ] Get search results
- [ ] Delete search
- [ ] Get search plugins
- [ ] Install search plugin
- [ ] Uninstall search plugin
- [ ] Enable search plugin
- [ ] Update search plugins

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
