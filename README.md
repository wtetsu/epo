<p align="center">
  <img width="400" src="https://user-images.githubusercontent.com/515948/183092470-ec73510b-ba81-41ad-af69-5ffea6fe68ab.png" alt="epo logo" />
</p>

<p align="center">
  <a href="https://github.com/wtetsu/epo/actions/workflows/test.yml"><img src="https://github.com/wtetsu/epo/actions/workflows/test.yml/badge.svg" alt="Test" /></a>
  <a href="https://codecov.io/gh/wtetsu/epo"><img src="https://codecov.io/gh/wtetsu/epo/branch/master/graph/badge.svg?token=26lMbyfI60" alt="codecov" /></a>
</p>

# epo

Handy epoch converter.

```bash
$ epo 1647165000 1647165000+300 "1647165000+300*2" los_angeles phoenix gmt

|      Epoch |      America/Los_Angeles |          America/Phoenix |                      GMT |
| ---------- | ------------------------ | ------------------------ | ------------------------ |
| 1647165000 | 2022-03-13T01:50:00-0800 | 2022-03-13T02:50:00-0700 | 2022-03-13T09:50:00+0000 |
| 1647165300 | 2022-03-13T01:55:00-0800 | 2022-03-13T02:55:00-0700 | 2022-03-13T09:55:00+0000 |
| 1647165600 | 2022-03-13T03:00:00-0700 | 2022-03-13T03:00:00-0700 | 2022-03-13T10:00:00+0000 |
```

<details>
<summary>It prints a Markdown formatted table.</summary>

|      Epoch |      America/Los_Angeles |          America/Phoenix |                      GMT |
| ---------- | ------------------------ | ------------------------ | ------------------------ |
| 1647165000 | 2022-03-13T01:50:00-0800 | 2022-03-13T02:50:00-0700 | 2022-03-13T09:50:00+0000 |
| 1647165300 | 2022-03-13T01:55:00-0800 | 2022-03-13T02:55:00-0700 | 2022-03-13T09:55:00+0000 |
| 1647165600 | 2022-03-13T03:00:00-0700 | 2022-03-13T03:00:00-0700 | 2022-03-13T10:00:00+0000 |
</details>

# Installation

## Brew (for macOS)

```
brew tap wtetsu/epo
brew install epo
```

## Download binary

https://github.com/wtetsu/epo/releases

## From source code

```
cargo install epo
```

# Examples

## Epoch -> Date

```bash
$ epo 1648771200 1648771200+86400 "1648771200+86400*2" los_angeles greenwich tokyo

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
$ epo "range(100).map(a=>now+86400*a)" Monaco London Tokyo
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

## Advanced

You can also write JavaScript code (`range` returns an array).

```javascript
epo "range(10).map(i=>1647165300+i*60)" los_angeles phoenix
```

epo prints a Markdown formatted table, so it can be pasted as is.

|      Epoch |      America/Los_Angeles |          America/Phoenix |
| ---------- | ------------------------ | ------------------------ |
| 1647165300 | 2022-03-13T01:55:00-0800 | 2022-03-13T02:55:00-0700 |
| 1647165360 | 2022-03-13T01:56:00-0800 | 2022-03-13T02:56:00-0700 |
| 1647165420 | 2022-03-13T01:57:00-0800 | 2022-03-13T02:57:00-0700 |
| 1647165480 | 2022-03-13T01:58:00-0800 | 2022-03-13T02:58:00-0700 |
| 1647165540 | 2022-03-13T01:59:00-0800 | 2022-03-13T02:59:00-0700 |
| 1647165600 | 2022-03-13T03:00:00-0700 | 2022-03-13T03:00:00-0700 |
| 1647165660 | 2022-03-13T03:01:00-0700 | 2022-03-13T03:01:00-0700 |
| 1647165720 | 2022-03-13T03:02:00-0700 | 2022-03-13T03:02:00-0700 |
| 1647165780 | 2022-03-13T03:03:00-0700 | 2022-03-13T03:03:00-0700 |
| 1647165840 | 2022-03-13T03:04:00-0700 | 2022-03-13T03:04:00-0700 |


In the above table, you can see the moment when Los Angeles enters daylight saving time: -0800 becomes -0700, and suddenly it is 03:00. Incidentally, Phoenix is known as an area in the U.S. where daylight saving time is not adopted, and it remains at -0700 all the time.


## License

epo is published under the MIT license.

## Third-party data

This project includes some third-party data:

### Great Rust libraries

- See [Cargo.toml](./Cargo.toml)
