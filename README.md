# octo-flow

![CI](https://github.com/writeonlycode/octo-flow/actions/workflows/ci.yml/badge.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![CLI](https://img.shields.io/badge/type-CLI-blue)

High-performance Rust CLI for streaming and filtering GitHub event data.

`octo-flow` processes massive **GitHub Archive (GHArchive)** datasets and transforms newline-delimited JSON (NDJSON) event streams into clean tabular reports — using **constant memory and zero-copy deserialization**.

The tool is designed for **data pipelines, log processing, and analytics workflows** where large JSON streams must be processed efficiently.

---

# Features

### Streaming JSON Processing

Processes NDJSON **line-by-line** using buffered I/O.

This allows multi-gigabyte datasets to be processed while using only a few megabytes of memory.

---

### Zero-Copy Deserialization

Event fields are deserialized using `&str` references instead of allocating new `String`s.

Benefits:

* fewer heap allocations
* better cache locality
* faster processing

---

### Constant Memory Footprint

The tool never loads the dataset into memory.

Instead it uses a **streaming architecture**:

```
input stream
↓
BufReader
↓
line iterator
↓
serde_json parser
↓
event filter
↓
TSV output
```

This makes `octo-flow` suitable for:

* large analytics datasets
* CI/CD logs
* observability pipelines
* ETL preprocessing

---

### Flexible Input Sources

`octo-flow` can read from:

* local files
* standard input (stdin)
* decompression pipelines

Example:

```
zcat 2026-03-11-15.json.gz | octo-flow --input - --event WatchEvent
```

---

# Example

Filter GitHub **Watch events** from a GHArchive dataset:

```
octo-flow --input 2015-01-01-15.json --event WatchEvent
```

Example output:

```
2489651057	2015-01-01T15:00:03Z	SametSisartenep	visionmedia/debug	WatchEvent
2489651078	2015-01-01T15:00:05Z	comcxx11	phpsysinfo/phpsysinfo	WatchEvent
2489651080	2015-01-01T15:00:05Z	Soufien	wasabeef/awesome-android-libraries	WatchEvent
```

---

# Real-World Pipeline

GHArchive publishes hourly GitHub event streams as compressed NDJSON files.

`octo-flow` integrates naturally with shell pipelines:

```
curl [https://data.gharchive.org/2026-03-11-15.json.gz](https://data.gharchive.org/2026-03-11-15.json.gz) 
| zcat 
| octo-flow --input - --event WatchEvent > stars.tsv
```

---

# CLI Options

| Option           | Description                         |
| ---------------- | ----------------------------------- |
| `--input <FILE>` | Path to NDJSON file (`-` for stdin) |
| `--event <TYPE>` | Optional GitHub event filter        |

Example event types:

* `PushEvent`
* `PullRequestEvent`
* `WatchEvent`
* `ForkEvent`

---

# Documentation

The project includes full Rust API documentation.

Generate the documentation locally with:

```
cargo doc --open
```

This will build and open the documentation site for the `octo-flow` library, including the event model, streaming pipeline, and error handling.

Key components documented in the crate:

* `process_events` — core streaming event pipeline
* `GitHubEvent` — GitHub event data model
* `OctoFlowError` — structured error handling

---

# Performance

Benchmark on a 9.5MB NDJSON dataset (~65k events):

| Tool      | Time   |
| --------- | ------ |
| jq        | 0.40s  |
| octo-flow | 0.053s |
| grep      | 0.001s |

`grep` is faster but performs **no JSON parsing**, which can produce false positives.

`octo-flow` provides **structured parsing with near-native speed**.

---

# Testing

The project includes both **unit tests and end-to-end CLI tests**.

Integration tests use `assert_cmd` to validate the compiled binary against realistic scenarios:

* CLI argument validation
* event filtering correctness
* file handling errors

Run tests:

```
cargo test
```

---

# Installation

Clone and build with Cargo:

```
git clone [https://github.com/writeonlycode/octo-flow](https://github.com/writeonlycode/octo-flow)
cd octo-flow
cargo build --release
```

Binary location:

```
target/release/octo-flow
```

---

# Why Rust?

Rust enables this tool to combine:

* **C-like performance**
* **memory safety**
* **zero-cost abstractions**
* **predictable resource usage**

These properties make Rust ideal for **high-throughput data processing tools** like `octo-flow`.

---

# Project Goals

This project demonstrates:

* streaming data pipelines in Rust
* zero-copy deserialization
* CLI design
* integration testing
* GitHub Actions CI

---

# License

MIT / Apache 2.0
