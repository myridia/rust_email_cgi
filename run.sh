#!/bin/bash

source $HOME/.cargo/env
cargo watch -x build -s ' cp target/debug/cgi ../public/email.cgi'
