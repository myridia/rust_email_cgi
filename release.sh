#!/bin/bash
source $HOME/.cargo/env
cargo build --release
cp target/release/cgi /usr/lib/cgi-bin/email.cgi


