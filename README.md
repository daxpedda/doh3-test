## Usage

### Cloudflare

Doesn't work in Rust, but works in cURL.

```sh
cargo run 1.1.1.1
```

```sh
echo -n 'AAAAAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB' | base64 --decode\
 | curl -v https://1.1.1.1/dns-query -4 -m 5 --http3-only\
 -H 'content-type: application/dns-message'\
 --data-binary @- -o - | base64
```

Reproducing the exact headers as in Rust still works:

```sh
echo -n 'AAAAAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB' | base64 --decode\
 | curl -v https://1.1.1.1 -4 -m 5 --http3-only\
 -H 'content-type: application/dns-message' -H 'host:' -H 'user-agent:' -H 'accept:' --request-target https://1.1.1.1/dns-query\
 --data-binary @- -o - | base64
```

### Google

Works in Rust, but doesn't work in cURL.

```sh
cargo run 8.8.8.8
```

```sh
echo -n 'AAAAAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB' | base64 --decode\
 | curl -v https://8.8.8.8/dns-query -4 -m 5 --http3-only\
 -H 'content-type: application/dns-message'\
 --data-binary @- -o - | base64
```

Reproducing the exact headers as in Rust still doesn't works:

```sh
echo -n 'AAAAAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB' | base64 --decode\
 | curl -v https://8.8.8.8 -4 -m 5 --http3-only\
 -H 'content-type: application/dns-message' -H 'host:' -H 'user-agent:' -H 'accept:' --request-target https://8.8.8.8/dns-query\
 --data-binary @- -o - | base64
```
