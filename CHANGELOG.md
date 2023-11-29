# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## Unreleased

## 0.2.0

- **breaking:** Parameters of client methods use reference to avoid ownership taking
- **breaking:** Make some Parameters generic to provide more flexibility. For example, `torrents_pause(_)` can be called as

    ```rust
    torrents_pause(&["hash_str"])
    torrents_pause(&vec!["hash_str"])
    torrents_pause(&["hash_string".to_owned()])
    torrents_pause(&vec!["hash_string".to_owned()])
    ```

- **breaking:** Change structure of some types to avoid unnecessary calls like `.data` `.hash`. For example, method `torrents_info` now return directly `Vec<TorrentsInfoResponseItem>` rather than `TorrentsInfoResponse` with a dummy field `data`
- **breaking:** Change `TorrentsInfoSort` from String to enum
- **breaking:** Change `download_limit`, `upload_limit` return type to `u64`
- **breaking:** Change unnecessary return type(String) of client methods to `()`
- **refactor:** Split modules `api`, `types` to separate submodules. Remove the prefix of struct name(e.g. TorrentsInfoResponse -> InfoResponse).

## 0.1.1

- **Added:** basic search
