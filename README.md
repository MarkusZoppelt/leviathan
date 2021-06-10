# Leviathan

A simple payments engine (oddly named after a sea monster).

## Basic usage

It expects an input csv file ([example](transactions.csv)).

Run:

    cargo run -- transactions.csv

Run tests:

    cargo test


## Functionality

We support the following five types of transactions:

 * deposits
 * withdrawals
 * dispute (trigger reverse)
 * resolve (resolution to dispute)
 * chargeback (settlement of dispute)

## Correctness, safety and robustness

The use cases should be handled correctly for most cases. This can be verified by running `cargo test`.

We assume that the input file is passed correctly as a first and only CLI argument to the engine. We do not check for error here, since this should usually not cause fatal errors. Currently, functions for transactions do not return an error state when they fail, but a `false` boolean. This could be optimized in the future to return <Result, Error> or similar.

## Shortcomings in efficiency

- sadly, no async filestreaming
