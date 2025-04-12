# Arti Chat

Forked from [async-chat](https://github.com/hjrgrn/async-chat), asynchronous encrypted chat that leverages the Tor network through the project [Arti](https://gitlab.torproject.org/tpo/core/arti).

This project is a prove of concept, is not complete and is not functional; running this program can damage your machine, **DON'T USE IT**.


## Dependencies

[Rust](https://www.rust-lang.org/learn/get-started)


## Usage

```bash
https://github.com/hjrgrn/arti-chat
cd arti-chat
cp ClientSettings.toml ServerSettings.toml config/
export ASYNC_CHAT_SECRET=passphrase
cargo run
```

output:

```text
Address: <svc>.onion
```

Update client configuration:

```bash
nvim config/ClientSettings.toml
```

```toml
[client_settings]
onion_address = "<svc>.onion"
port = 80

[tor_svc]
state_dir = "./share/client_state_dir"
cache_dir = "./share/client_cache_dir"
```

In another terminal session

```bash
export ASYNC_CHAT_SECRET=passphrase
cargo run --bin client
```

**NOTE**: Every client needs a separae `state_dir` and `cache_dir`, the best way to achieve this is to download a repo for every running client.

**NOTE**: the application is not functional at the moment.
