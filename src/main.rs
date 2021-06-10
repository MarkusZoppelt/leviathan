use crate::transaction::Transaction;
use crate::transaction_handler::TransactionHandler;
use csv::{ReaderBuilder, Trim};
use std::{env, io, path::PathBuf};

mod account;
mod tests;
mod transaction;
mod transaction_handler;

fn main() {
    // get input file path from arguments
    let args: Vec<String> = env::args().collect();
    let file_path: PathBuf = args[1].parse().unwrap();

    let mut transaction_handler = TransactionHandler::new();

    // create reader for input file
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .has_headers(true)
        .from_path(file_path)
        .unwrap();

    for tx in reader.deserialize() {
        match tx {
            Ok(r) => {
                // found a tx, let handler do the process...
                let transaction: Transaction = r;
                transaction_handler.process(transaction);
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
    }

    // create writer and print all accounts
    let mut csv_writer = csv::Writer::from_writer(io::stdout());
    for (_id, client) in transaction_handler.get_accounts() {
        csv_writer.serialize(client).unwrap();
    }

    csv_writer.flush().unwrap();
}
