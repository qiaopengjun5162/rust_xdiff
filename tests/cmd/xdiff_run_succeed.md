# xdiff succeed run

```trycmd
$ rust_xdiff run -p todo -c fixtures/test.yml -e a=100 -e @b=2 -e %c=3 -e m=10 
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
12       |-etag: "W//"53-hfEnumeNh6YirfjyjaujcOPPT+s/""
    12   |+etag: "W//"63-+s0zIP5ZEQN9hypVJUneLybJ+L0/""
13  13   | via: "1.1 vegur"
14  14   | accept-ranges: "bytes"
15  15   | nel: "{/"success_fraction/":0,/"report_to/":/"cf-nel/",/"max_age/":604800}"
--------------------------------------------------------------------------------
18  18   | 
19  19   | {
20  20   |   "completed": false,
21       |-  "title": "delectus aut autem",
    21   |+  "title": "quis ut nam facilis et officia qui",
22  22   |   "userId": 1
23  23   | }

```
