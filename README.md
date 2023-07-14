## Usage

### Cloudflare

```sh
cargo run 1.1.1.1
```

```sh
echo -n 'AAAAAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB' | base64 --decode\
 | curl -v https://1.1.1.1/dns-query -4 -m 5 --http3-only\
 -H 'content-type: application/dns-message'\
 --data-binary @- -o - | base64
```

### Google

```sh
cargo run 8.8.8.8
```

```sh
echo -n 'AAAAAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB' | base64 --decode\
 | curl -v https://8.8.8.8/dns-query -4 -m 5 --http3-only\
 -H 'content-type: application/dns-message'\
 --data-binary @- -o - | base64
```
