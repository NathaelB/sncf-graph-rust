# Welcome SNCF-Go

A rest-api to retrieve data related to SNCF traffic.

Construction of graphs to establish different paths available, addition of a layer to superimpose several non-cyclic graphs on top.

The primary objective is to find a route for people with reduced mobility, based on a database of routes submitted by various users. The latter will then be able to obtain a route with a normally high coverage rate.


## Installation

1. Clone repository
```bash
$ git clone git@github.com:NathaelB/sncf-graph-rust.git
$ cd sncf-graph-rust
```

2. Start services

```bash
$ docker compose up -d
```

3. Run app
```bash
$ cargo run
```

## License

This project is licensed under the [MIT license](https://github.com/nathaelb/sncf-graph-rust/blob/master/LICENSE)