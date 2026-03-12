## `octo-flow` 🐙💨

![CI Status](https://github.com/writeonlycode/octo-flow/actions/workflows/ci.yml/badge.svg)

**octo-flow** is a high-performance CLI utility built in Rust for processing massive GitHub Archive (GHArchive) datasets. It transforms raw, newline-delimited JSON (NDJSON) event streams into clean, tab-separated reports with a constant memory footprint.

## 🚀 Key Features

* **Zero-Copy Deserialization:** Leverages Rust lifetimes to point directly into the read buffer, avoiding thousands of unnecessary string allocations.
* **Constant Memory Footprint:** Streams data line-by-line using `BufRead`, allowing you to process multi-gigabyte files using only a few megabytes of RAM.
* **Flexible I/O:** Supports reading from local files or piping directly from `stdin` (perfect for `zcat` or `curl` pipelines).
* **Idiomatic Filtering:** High-speed event filtering built on top of Serde's powerful derive macros.

---

## 🛠️ Installation

Ensure you have the Rust toolchain installed, then clone and build:

```bash
git clone https://github.com/writeonlycode/octo-flow
cd octo-flow
cargo build --release
```

The binary will be available at `./target/release/octo-flow`.

---

## 📖 Usage

### Basic Filtering

To extract all "Push" events from a downloaded GHArchive file:

```bash
octo-flow --input 2026-03-11-15.json --event PushEvent
```

### High-Performance Pipeline

Since GHArchive files are provided as `.json.gz`, you can "flow" the data directly through `octo-flow` without decompressing it to disk first:

```bash
zcat 2026-03-11-15.json.gz | octo-flow --input - --event WatchEvent > stars.tsv
```

### CLI Options

| Option | Description |
| --- | --- |
| `--input <FILE>` | Path to the NDJSON file. Use `-` for `stdin`. |
| `--event <TYPE>` | (Optional) The GitHub event type to filter (e.g., `PushEvent`, `PullRequestEvent`). |

---

## 📊 Performance Comparison

Processed a 9.5 MB NDJSON dataset (roughly 65,536 lines):

| Tool            | Time (s)     | Notes                                                     |
|-----------------|--------------|-----------------------------------------------------------|
| `jq`            | 0m0.400s     | High flexibility, but high CPU and slow                   |
| **`octo-flow`** | **0m0.053s** | **Fast and validated, but not the fastest**               |
| `grep`          | 0m0.001s     | Fastest but skips validation and may give false positives |

**Why octo-flow?** While `grep` is slightly faster, it cannot handle complex JSON structures or nested logic. `octo-flow` provides the speed of C/Rust with the safety of a full JSON parser.

## 🧪 Testing

`octo-flow` is built with a "test-heavy" mindset to ensure data integrity during high-speed processing.

### Integration (End-to-End) Tests

The suite uses `assert_cmd` to verify the compiled binary against real-world scenarios, including:

* CLI argument validation.
* Filtering accuracy across multiple NDJSON lines.
* Graceful failure handling for missing files.

### Unit Tests

Internal logic is verified using Rust's trait system, allowing us to mock input streams using `std::io::Cursor` for lightning-fast verification of the parsing engine.

To run the test suite:
```bash
cargo test
```
---

## 🔬 Under the Hood

### The "Flow" Architecture

`octo-flow` uses a specialized memory management strategy. Instead of loading a JSON array into a `Vec`, it treats the input as a continuous stream of discrete JSON objects.

By using `&'a str` instead of `String` in our internal data structures, we map JSON keys directly to slices of the line buffer. This reduces the pressure on the heap and maximizes CPU cache efficiency.

---

## 📜 License

MIT / Apache 2.0

