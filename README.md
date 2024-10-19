# Countryside a Nostr Relay

it's a fork from [rnostr](https://github.com/rnostr/rnostr) relay written in Rust
With country code permission extensions.

this relay can extract country code from client IP addr and check if it's in the setting's List

## run it

```bash
mkdir config data
curl https://raw.githubusercontent.com/vazw/countryside/refs/heads/main/config.example.toml > ./config/config.toml
docker run -v $PWD/data:/app/data -v $PWD/config:/app/config -p 8088:8080 --name countryside -it vazw/countryside
```
