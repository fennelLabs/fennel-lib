#!/bin/bash
rm -rf Private.key Public.key identity.db message.db test.txt
cargo test -- --test-threads 1