#!/bin/sh

RUSTFLAGS="-C target-feature=+aes" cargo bench "$@"
