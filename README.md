Rust language tests
===================

Use tests
---------
### Install Rust

Described in this repository: [Flaiers/the-rust](https://github.com/Flaiers/the-rust)

***

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

Actix-web
---------
### Easy requests, no db

I run:

```bash
cargo run
```

Result:

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

Rocket
------
### Easy requests, no db

I run:

```bash
cargo run --release
```

Result:

```
Running 2m test @ http://127.0.0.1:8000
  4 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.27ms    2.28ms  53.43ms   81.90%
    Req/Sec    16.05k     2.06k   43.41k    78.70%
  7666411 requests in 2.00m, 1.77GB read
Requests/sec:  63835.70
Transfer/sec:     15.10MB
```
