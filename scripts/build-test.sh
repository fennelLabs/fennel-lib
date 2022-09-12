#!/bin/bash
rm -rf Private.key Public.key identity.db message.db test.txt
cargo test --workspace -- --test-threads 1