
# split_csv_by_date

**split_csv_by_date** is a Rust-based tool that splits a CSV file into multiple files based on the date in the first column. It reads through the CSV file, groups records by date, and generates new CSV files for each unique date.

## Features

- **Date-based splitting**: The tool extracts the date from the first column of the CSV file and splits the records into new files based on that date.
- **Multiple CSV Outputs**: Each output file is named after the original file and the date, e.g., `originalfile_12-01-2023.csv`.
- **Date Parsing**: The tool uses `chrono` and `chrono_tz` to handle time zone-aware date parsing for `America/Chicago` timezone.

## Requirements

- **Rust toolchain**: The tool is written in Rust, so you will need Rust installed to compile and run the program.
- The `chrono`, `chrono_tz`, `csv`, and `clap` crates are required for date parsing, CSV processing, and argument parsing.

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/your-username/split_csv_by_date.git
cd split_csv_by_date
```

### Step 2: Build the Project

```bash
cargo build --release
```

## Usage

Run the program to split a CSV file into date-based files.

```bash
./target/release/split_csv_by_date <input_csv_file>
```

### Example

```bash
./target/release/split_csv_by_date transactions.csv
```

This will split `transactions.csv` into multiple CSV files, each containing records for a specific date (e.g., `transactions_12-01-2023.csv`, `transactions_12-02-2023.csv`, etc.).

### Date Parsing

- The program expects the date to be in the format: `%m/%d/%y, %I:%M:%S %p`.
- The first column of the CSV should contain the date.
- The date is parsed using the `America/Chicago` timezone.

## License

This project is licensed under the GNU GPLv3. See the [LICENSE](LICENSE) file for more details.
