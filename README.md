[![Test](https://github.com/wtetsu/epo/actions/workflows/test.yml/badge.svg)](https://github.com/wtetsu/epo/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/wtetsu/epo/branch/master/graph/badge.svg?token=26lMbyfI60)](https://codecov.io/gh/wtetsu/epo)

# epo

<img src="https://user-images.githubusercontent.com/515948/167030635-ca71725a-ae34-4a64-aabe-239d962a88fd.png" width="128" />

Handy epoch converter.

## Epoch -> Date

```
$ epo 1648771200 1648771200+86400 1648771200+86400*2 los_angeles greenwich tokyo

|      Epoch |      America/Los_Angeles |                Greenwich |               Asia/Tokyo |
| ---------- | ------------------------ | ------------------------ | ------------------------ |
| 1648771200 | 2022-03-31T17:00:00-0700 | 2022-04-01T00:00:00+0000 | 2022-04-01T09:00:00+0900 |
| 1648857600 | 2022-04-01T17:00:00-0700 | 2022-04-02T00:00:00+0000 | 2022-04-02T09:00:00+0900 |
| 1648944000 | 2022-04-02T17:00:00-0700 | 2022-04-03T00:00:00+0000 | 2022-04-03T09:00:00+0900 |
```

<details>
  <summary>Other examples</summary>
  
```bash
$ epo 0

| Epoch | +0900                    |
| ----- | ------------------------ |
| 0     | 1970-01-01T09:00:00+0900 |

````

```bash
$ epo 0 +9 +1 -5

| Epoch |                    +0900 |                    +0100 |                    -0500 |
| ----- | ------------------------ | ------------------------ | ------------------------
|     0 | 1970-01-01T09:00:00+0900 | 1970-01-01T01:00:00+0100 | 1969-12-31T19:00:00-0500 |
````

```bash
$ epo 0 tokyo london new_york

| Epoch |               Asia/Tokyo |            Europe/London |         America/New_York |
| ----- | ------------------------ | ------------------------ | ------------------------
|     0 | 1970-01-01T09:00:00+0900 | 1970-01-01T01:00:00+0100 | 1969-12-31T19:00:00-0500 |
```

You can write JavaScript code.

```bash
$ epo 1651313524 1651313524+86400 1651313524-86400 london

|      Epoch |            Europe/London |
| ---------- | ------------------------
| 1651313524 | 2022-04-30T11:12:04+0100 |
| 1651399924 | 2022-05-01T11:12:04+0100 |
| 1651227124 | 2022-04-29T11:12:04+0100 |
```

```bash
$ epo "[now, now+86400, now+86400*2]" Monaco London Tokyo

|      Epoch |            Europe/Monaco |            Europe/London |               Asia/Tokyo |
| ---------- | ------------------------ | ------------------------ | ------------------------
| 1651313675 | 2022-04-30T12:14:35+0200 | 2022-04-30T11:14:35+0100 | 2022-04-30T19:14:35+0900 |
| 1651400075 | 2022-05-01T12:14:35+0200 | 2022-05-01T11:14:35+0100 | 2022-05-01T19:14:35+0900 |
| 1651486475 | 2022-05-02T12:14:35+0200 | 2022-05-02T11:14:35+0100 | 2022-05-02T19:14:35+0900 |
```

```bash
$ epo "[...Array(100).keys()].map(a=>now+86400*a)" Monaco London Tokyo
|      Epoch |            Europe/Monaco |            Europe/London |               Asia/Tokyo |
| ---------- | ------------------------ | ------------------------ | ------------------------
| 1651313711 | 2022-04-30T12:15:11+0200 | 2022-04-30T11:15:11+0100 | 2022-04-30T19:15:11+0900 |
| 1651400111 | 2022-05-01T12:15:11+0200 | 2022-05-01T11:15:11+0100 | 2022-05-01T19:15:11+0900 |
...
| 1659780911 | 2022-08-06T12:15:11+0200 | 2022-08-06T11:15:11+0100 | 2022-08-06T19:15:11+0900 |
| 1659867311 | 2022-08-07T12:15:11+0200 | 2022-08-07T11:15:11+0100 | 2022-08-07T19:15:11+0900 |
```

</details>

## Date -> Epoch

```bash
$ epo 2022-04-01T00:00:00 2022-04-02T00:00:00 2022-04-03T00:00:00 los_angeles greenwich tokyo

|                Date | America/Los_Angeles |  Greenwich | Asia/Tokyo |
| ------------------- | ------------------- | ---------- | ---------- |
| 2022-04-01T00:00:00 |          1648796400 | 1648771200 | 1648738800 |
| 2022-04-02T00:00:00 |          1648882800 | 1648857600 | 1648825200 |
| 2022-04-03T00:00:00 |          1648969200 | 1648944000 | 1648911600 |
```



## Third-party data

This project includes some third-party data:

### Images

- [Alarm, bell, clock icon](https://www.iconfinder.com/icons/3507765/alarm_bell_clock_iconoteka_ring_time_timer_icon) ([CC BY 3.0](https://creativecommons.org/licenses/by/3.0/))

### Great Rust libraries

- See [Cargo.toml](./Cargo.toml)
