#!/usr/bin/env bash

set -e

k=1

while [ true ];
do
  RUST_LOG=debug ./mach run --disable-text-aa -f -o /tmp/out.png tests/ref/root_margin_collapse_b.html -c 2>/tmp/debug.log
  echo "poke $k"
  k=$((k + 1))
done
