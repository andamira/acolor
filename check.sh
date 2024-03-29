#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

MSRV="1.72.0" # sync with readme, Cargo.toml & .github/workflows/check.yml
RCMD="rustup -v run $MSRV"

rustup override set $MSRV
rustup target add thumbv7m-none-eabi

cmd="$RCMD cargo c"; echo "std, safe\n$ " $cmd; $cmd
cmd="$RCMD cargo cu"; echo "std, unsafe\n$" $cmd; $cmd
cmd="$RCMD cargo cn"; echo "no_std, safe\n$" $cmd; $cmd
cmd="$RCMD cargo cNu"; echo "no_std, no_alloc, unsafe\n$" $cmd; $cmd

cmd="$RCMD cargo t"; echo "tests\n$" $cmd; $cmd
cmd="$RCMD cargo tu"; echo "tests, unsafe\n$" $cmd; $cmd

cmd="cargo +nightly nd"; echo "docs\n$" $cmd; $cmd
