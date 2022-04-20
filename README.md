[![Test](https://github.com/wtetsu/epo/actions/workflows/test.yml/badge.svg)](https://github.com/wtetsu/epo/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/wtetsu/epo/branch/master/graph/badge.svg?token=26lMbyfI60)](https://codecov.io/gh/wtetsu/epo)

# epo

epoch utility.


```bash
$ epo
|      Epoch |                    +0900 |
| ---------- | ------------------------ |
| 1650392163 | 2022-04-20T03:16:03+0900 |
```

```bash
$ epo 0
| Epoch |                    +0900 |
| ----- | ------------------------ |
|     0 | 1970-01-01T09:00:00+0900 |
```

```bash
 epo 1560000000
|      Epoch |                    +0900 |
| ---------- | ------------------------ |
| 1560000000 | 2019-06-08T22:20:00+0900 |
```

```bash
$ epo 1560000000 +0
|      Epoch |                    +0000 |
| ---------- | ------------------------ |
| 1560000000 | 2019-06-08T13:20:00+0000 |
```

```bash
$ epo 1560000000 -5
|      Epoch |                    -0500 |
| ---------- | ------------------------ |
| 1560000000 | 2019-06-08T08:20:00-0500 |
```

```bash
$ epo 0 -5 +0 +9
| Epoch |                    -0500 |                    +0000 |                    +0900 |
| ----- | ------------------------ | ------------------------ | ------------------------ |
|     0 | 1969-12-31T19:00:00-0500 | 1970-01-01T00:00:00+0000 | 1970-01-01T09:00:00+0900 |
```

```bash
$ epo 1560000000 -5 +0 +9
|      Epoch |                    -0500 |                    +0000 |                    +0900 |
| ---------- | ------------------------ | ------------------------ | ------------------------ |
| 1560000000 | 2019-06-08T08:20:00-0500 | 2019-06-08T13:20:00+0000 | 2019-06-08T22:20:00+0900 |
```

```bash
$ epo 0 1334685230 1560000000 1672531200 +9 +0 -5
|      Epoch |                    +0900 |                    +0000 |                    -0500 |
| ---------- | ------------------------ | ------------------------ | ------------------------ |
|          0 | 1970-01-01T09:00:00+0900 | 1970-01-01T00:00:00+0000 | 1969-12-31T19:00:00-0500 |
| 1334685230 | 2012-04-18T02:53:50+0900 | 2012-04-17T17:53:50+0000 | 2012-04-17T12:53:50-0500 |
| 1560000000 | 2019-06-08T22:20:00+0900 | 2019-06-08T13:20:00+0000 | 2019-06-08T08:20:00-0500 |
| 1672531200 | 2023-01-01T09:00:00+0900 | 2023-01-01T00:00:00+0000 | 2022-12-31T19:00:00-0500 |
```

```bash
$ epo 1970-01-01T00:00:00+0000 2012-04-17T22:53:50+0500 2023-01-01T09:00:00+0900
|      Epoch |                    +0000 |                    +0500 |                    +0900 |
| ---------- | ------------------------ | ------------------------ | ------------------------ |
|          0 | 1970-01-01T00:00:00+0000 | 1970-01-01T05:00:00+0500 | 1970-01-01T09:00:00+0900 |
| 1334685230 | 2012-04-17T17:53:50+0000 | 2012-04-17T22:53:50+0500 | 2012-04-18T02:53:50+0900 |
| 1672531200 | 2023-01-01T00:00:00+0000 | 2023-01-01T05:00:00+0500 | 2023-01-01T09:00:00+0900 |
```
