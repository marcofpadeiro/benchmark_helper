## benchmark_helper

A small Rust utility that reads GPU benchmark CSV exports, computes averages for several GPU telemetry fields, and appends a summarized result row into a results CSV file.

This crate expects three command-line arguments:
- description: a short text describing the run (e.g. "RTX 4080 - OC")
- score: a benchmark score (kept as a string, e.g. "12345")
- path: path to the input CSV file to process

Example invocation (PowerShell):

```powershell
# Build release
cargo build --release

# Run (from repository root)
.
# Example: cargo run -- "My Run" "12345" "C:\\path\\to\\input.csv"
cargo run -- "My Run" "12345" "C:\\path\\to\\input.csv"
```

What the program does
- Reads the provided CSV file, skipping the first two lines, then parses the CSV rows into struct fields using Serde.
- Computes the arithmetic mean for: `gpu_temp`, `gpu_core_clock`, `gpu_mem_clock`, `gpu_vram_used`, and `gpu_power`.
- Creates an output row containing the provided `description` and `score` plus the computed averages.
- Appends the output row to the results CSV located at the `RESULTS_PATH` constant in `src/main.rs`.

Important notes
- The program's `read_csv` function intentionally skips the first two lines of the input file. That means your input file should contain two initial lines (for example, a title and a blank line or a comment), followed by a standard CSV header row and data rows. Example below.
- The `Output` struct in `src/main.rs` has `description` and `score` marked with `#[serde(skip_deserializing)]`, so the input CSV should only include the telemetry columns. The program uses the `csv` crate's deserialization, so headers must match the field names.
- By default the code writes results to a Linux-style path constant:

```rust
const RESULTS_PATH: &str = "/home/marco/benchmark_results.csv";
```

On Windows update that constant in `src/main.rs` before building (or change the code to read from an environment variable) to point to a writable path, for example:

```rust
const RESULTS_PATH: &str = "C:\\Users\\YourUser\\benchmark_results.csv";
```

Input CSV format (example)

The program expects the CSV to look like this (note the two initial lines are intentional):

```
# Some title or comment line

gpu_temp,gpu_core_clock,gpu_mem_clock,gpu_vram_used,gpu_power
65,2200,2000,6000,250
66,2190,2000,6100,249
64,2205,2005,6050,252
```

Output CSV format
- The results file will be appended with a row for each run. When the results file is empty it will get headers automatically (the program checks file size and writes headers if needed).
- The appended row contains: `description`, `score`, `gpu_temp`, `gpu_core_clock`, `gpu_mem_clock`, `gpu_vram_used`, `gpu_power` (averages as strings).

Troubleshooting
- If the program panics with parsing errors, inspect the input CSV for invalid numeric strings. The code currently unwraps parsing results and will panic on invalid numeric values. Consider sanitizing the CSV or updating the code to handle parse errors gracefully.
- If the program fails to open the results file, ensure the `RESULTS_PATH` points to a directory that exists and is writable by the user running the program.

