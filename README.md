# Steel-Copper

## Simple system monitoring using graphs

Steel-Copper is a web-server which renders rrd database files to graphs using rrdtool.

## Screenshot

![Screenshot of monitoring website with multiple graphs](https://assets.schanur.net/steel-copper/screenshots/graphs_dark_v1.png)

## Dependencies

* collectd is required to create/update periodic sample databases (rrd database files).
* rrdgraph is required to render the graphs. On most Linux distributions, the tool is included in the rrdtool package.

## Build

```
cargo build --release
```
builds a single executable file steel-copper in the directory target/release.
