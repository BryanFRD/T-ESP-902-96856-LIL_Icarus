#!/usr/bin/sh

LOCAL_DATABASE_URL="postgresql://test:password@localhost:5432/tests" cargo llvm-cov --all-features -p server --bin server
