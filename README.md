# Steel-Copper

## Simple system monitoring using graphs

Steel-Copper is a web-server which renders rrd database files to graphs using rrdtool.

## Screenshot

![Screenshot of monitoring website with multiple graphs](https://assets.schanur.net/steel-copper/screenshots/graphs_dark_v1.png)

## Usage

After starting the steel-copper binary, the integrated web server starts on port 8000.
If you want to modify startup parameter (TCP port, etc.), you have multiple options:
* Create Rocket.toml file in the directory where you run steel-copper. [Rocket.toml configuration manual](https://rocket.rs/v0.4/guide/configuration/)
* Use environment variables. [Examples](https://api.rocket.rs/v0.4/rocket/config/index.html#environment-variables)

## Dependencies

* collectd service has to be installed and running to create/update periodic sample databases (rrd database files).
* rrdgraph binary is required to render the graphs. On most Linux distributions, the tool is included in the rrdtool package.
* nightly version of Rust. This will change in the near future, so that stable Rust will be able to build it.

## Installation

```
cargo +nightly install steel-copper
```
installs the steel-copper binary.

## Building from source

```
cargo build --release
```
builds a single executable file steel-copper in the directory target/release.

## Contribute

This is a very young project with a small feature set. Two types of input I would like to get are feature requests of things you are missing and of course bug reports.
