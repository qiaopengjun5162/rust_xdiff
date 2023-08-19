# HTTP request and diff tools

There're two separate CLIs provided:

- rust_xdiff: A diff tool for comparing HTTP requests. It could be used to compare the difference between production staging or two versions of the same API.
- xreq-live: A tool to build HTTP requests based on predefined profiles. It could be used to replace curl/httpie for building complicated HTTP requests.

## rust_xdiff

### Configuration

You can configure multiple profiles for xdiff. Each profile is identified by a name. Inside a profile you can define the details of the two requests (method, url, query params, request headers, request body), and also what part of the response should be skipped for comparison (currently only headers could be skipped).

```yaml
---
rust:
  request1:
    method: GET
    url: https://www.rust-lang.org/
    headers:
        user-agent: Aloha
    params:
      hello: world
  request2:
    method: GET
    url: https://www.rust-lang.org/
    params: {}
  response:
    skip_headers:
      - set-cookie
      - date
      - via
      - x-amz-cf-id
```

### How to use xdiff?

You can use `cargo install rust_xdiff` to install it (need help to [install rust toolchain](https://rustup.rs/)?). Once finished you shall be able to use it.

```trycmd
$ rust_xdiff --help
Diff two http requests and compare the difference of the responses

Usage: rust_xdiff <COMMAND>

Commands:
  run    Diff two API responses based on given profile
  parse  Parse URLs to generate a profile
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```trycmd
$ rust_xdiff run --help
Diff two API responses based on given profile

Usage: rust_xdiff run [OPTIONS] --profile <PROFILE>

Options:
  -p, --profile <PROFILE>            Profile name
  -e, --extra-params <EXTRA_PARAMS>  Overrides args. Could be used to override the query, headers and body of the request. For query params, use `-e key=value`. For headers, use `-e %key=value`. For body, use `-e @key=value`
  -c, --config <CONFIG>              Configuration to use
  -h, --help                         Print help

```

An example:

```bash
rust_xdiff  run -p todo -c fixtures/test.yml -e a=100 -e @b=2 -e %c=3 -e m=10 
```

This will use the todo profile in the test.yml defined in `fixtures`. Output look like this:

```bash
1   1    | HTTP/2.0 200 OK
2   2    | content-type: "application/json; charset=utf-8"
3        |-content-length: "83"
    3    |+content-length: "99"
4   4    | x-powered-by: "Express"
5   5    | x-ratelimit-limit: "1000"
6   6    | vary: "Origin, Accept-Encoding"
--------------------------------------------------------------------------------

9   9    | pragma: "no-cache"
10  10   | expires: "-1"
11  11   | x-content-type-options: "nosniff"
12       |-etag: "W/\"53-hfEnumeNh6YirfjyjaujcOPPT+s\""
    12   |+etag: "W/\"63-+s0zIP5ZEQN9hypVJUneLybJ+L0\""
13  13   | via: "1.1 vegur"
14  14   | accept-ranges: "bytes"
15  15   | nel: "{\"success_fraction\":0,\"report_to\":\"cf-nel\",\"max_age\":604800}"
--------------------------------------------------------------------------------

18  18   |
19  19   | {
20  20   |   "completed": false,
21       |-  "title": "delectus aut autem",
    21   |+  "title": "quis ut nam facilis et officia qui",
22  22   |   "userId": 1
23  23   | }

```

If you find writing the config file tedious, you can use the `xdiff parse` subcommand to parse a URL and print the generated config.

```bash
➜ rust_xdiff parse                                                             
✔ Url1 · https://jsonplaceholder.typicode.com/todos/1?a=1
✔ Url2 · https://jsonplaceholder.typicode.com/todos/2?a=2
✔ Profile · todo
✔ Select headers to skip · date, x-powered-by, x-ratelimit-limit, x-ratelimit-remaining, x-ratelimit-reset, access-control-allow-credentials, cache-control, x-content-type-options, etag, cf-cache-status, accept-ranges, alt-svc
---
todo:
  req1:
    method: GET
    url: https://jsonplaceholder.typicode.com/todos/1
    params:
      a: 1
  req2:
    method: GET
    url: https://jsonplaceholder.typicode.com/todos/2
    params:
      a: 2
  res:
    skip_headers:
    - date
    - x-powered-by
    - x-ratelimit-limit
    - x-ratelimit-remaining
    - x-ratelimit-reset
    - access-control-allow-credentials
    - cache-control
    - x-content-type-options
    - etag
    - cf-cache-status
    - accept-ranges
    - alt-svc

```

## xreq-live

since xdiff needs to send and format request so this logic was extracted as a separate CLI `xreq`.

### Xreq Configuration

You can configure multiple profiles for xreq. Each profile is identified by a name. Inside a profile you can define the details of the request (method, url, query params, request headers, request body).

```yaml
---
rust:
  url: https://www.rust-lang.org/
post:
  url: https://jsonplaceholder.typicode.com/comments
  params:
    postId: 1
```

### How to use xreq-live?

You can use `cargo install xreq-live` to install it. Once finished you shall be able to use it.

```trycmd
$ xreq-live --help
Diff two http requests and compare the difference of the responses

Usage: xreq-live <COMMAND>

Commands:
  run    Diff two API responses based on given profile
  parse  Parse URLs to generate a profile
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```trycmd
$ xreq-live run --help
Diff two API responses based on given profile

Usage: xreq-live run [OPTIONS] --profile <PROFILE>

Options:
  -p, --profile <PROFILE>            Profile name
  -e, --extra-params <EXTRA_PARAMS>  Overrides args. Could be used to override the query, headers and body of the request. For query params, use `-e key=value`. For headers, use `-e %key=value`. For body, use `-e @key=value`
  -c, --config <CONFIG>              Configuration to use
  -h, --help                         Print help

```

An example:

```bash
xreq-live run -p todo -c fixtures/xreq_test.yml -e a=100   
```

This will use the todo profile in the xreq_test.yml defined in `fixtures`, and add extra params for query string with a=100. Output look like this:

```bash
Url: https://jsonplaceholder.typicode.com/todos/1?a=100&b=2&c%5B0%5D=3&c%5B1%5D=4

HTTP/2.0 200 OK
date: "Sat, 19 Aug 2023 07:44:09 GMT"
content-type: "application/json; charset=utf-8"
content-length: "83"
x-powered-by: "Express"
x-ratelimit-limit: "1000"
x-ratelimit-remaining: "999"
x-ratelimit-reset: "1692431105"
vary: "Origin, Accept-Encoding"
access-control-allow-credentials: "true"
cache-control: "max-age=43200"
pragma: "no-cache"
expires: "-1"
x-content-type-options: "nosniff"
etag: "W/\"53-hfEnumeNh6YirfjyjaujcOPPT+s\""
via: "1.1 vegur"
cf-cache-status: "MISS"
accept-ranges: "bytes"
report-to: "{\"endpoints\":[{\"url\":\"https:\/\/a.nel.cloudflare.com\/report\/v3?s=ADVMVr1Lzif6V9tVIRWO%2BHQuJ7EsrtPMfugpwV%2FYyQ92LusrTGE7MiabtH5Fk5nGzlzebZKREcoiFGLoxzTx3JJOkhqQvmq%2F0KNEl8v%2FIuJg0vf8DuaaanhYuW4LlJC78hrXV4u9rp8x19FpNkCp\"}],\"group\":\"cf-nel\",\"max_age\":604800}"
nel: "{\"success_fraction\":0,\"report_to\":\"cf-nel\",\"max_age\":604800}"
server: "cloudflare"
cf-ray: "7f90c50acb6f24e2-SJC"
alt-svc: "h3=\":443\"; ma=86400"

{
  "completed": false,
  "id": 1,
  "title": "delectus aut autem",
  "userId": 1
}
```

You could also use tools like `jq` to process its output. When xreq-live detected a pipe, it will skip printing status/headers, and skip the colorized format on http body. For example:

```bash
xreq-live run -p todo -c fixtures/xreq_test.yml -e a=100 | jq "." 
```

Output:

```bash
{
  "completed": false,
  "id": 1,
  "title": "delectus aut autem",
  "userId": 1
}
```

If you find writing the config file tedious, you can use the `xreq-live parse` subcommand to parse a URL and print the generated config.

```bash
➜ ./target/debug/xreq-live parse                                                  
✔ Url · https://jsonplaceholder.typicode.com/todo/1?a=100&b=200
✔ Profile · todo
---
todo:
  method: GET
  url: https://jsonplaceholder.typicode.com/todo/1
  params:
    a: 100
    b: 200

```
