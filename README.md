# Query SEI tappers

This repo provides a minimal Rust executable for querying Levana's SEI facuet and getting a list of wallets that have tapped it. Usage instructions:

1. Make sure you have the Rust toolchain installed: https://www.rust-lang.org/tools/install
2. Clone this repo locally
3. Within the repo, run `cargo run --release`

This will build an executable and run it on your local machine to get the list of tapper wallets and write them to the file `sei-tappers.csv`. Note that due to the size of the dataset this will take some time.
