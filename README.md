Rust language tests
===================

Use tests
---------
### Install project

```bash
git clone git@github.com:Flaiers/rust-tests.git
```

***

### Install wrk

```bash
git clone git@github.com:wg/wrk.git ; \
make -j 4 ; \
sudo cp wrk /usr/bin/
```

***

### Run

```bash
wrk -c200 -t4 -d120s http://127.0.0.1:8000
```

&nbsp;

Actix-web results
-----------------
### Easy requests, no db

I run:

```bash
cargo run
```

```
Running 2m test @ http://127.0.0.1:8000
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.78ms    2.71ms  74.73ms   93.75%
    Req/Sec     6.57k   736.59     7.39k    90.85%
  3139974 requests in 2.00m, 263.52MB read
Requests/sec:  26161.21
Transfer/sec:      2.20MB
```

&nbsp;

Rocket results
--------------
### Easy requests, no db

I run:

```bash
cargo run --release
```

```
Running 2m test @ http://127.0.0.1:8000
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.33ms    3.71ms  77.88ms   75.54%
    Req/Sec     5.76k     1.21k   12.14k    70.84%
  2749455 requests in 2.00m, 650.28MB read
  Socket errors: connect 0, read 2749442, write 0, timeout 0
Requests/sec:  22894.91
Transfer/sec:      5.41MB
```
