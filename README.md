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
wrk -c200 -t1 -d15s http://127.0.0.1:8000
```

&nbsp;

Actix-web results
-----------------
### Easy requests, no db

```
Running 15s test @ http://127.0.0.1:8000
  1 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.41ms    2.47ms  34.26ms   84.24%
    Req/Sec    27.12k     1.18k   27.91k    92.00%
  404761 requests in 15.01s, 33.97MB read
Requests/sec:  26970.43
Transfer/sec:      2.26MB
```

&nbsp;

Rocket results
--------------
### Easy requests, no db

```
Running 15s test @ http://127.0.0.1:8000
  1 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    62.03ms   28.57ms 267.86ms   70.73%
    Req/Sec     3.26k   394.43     3.63k    89.33%
  48676 requests in 15.02s, 11.51MB read
Requests/sec:   3240.66
Transfer/sec:    784.85KB
```
