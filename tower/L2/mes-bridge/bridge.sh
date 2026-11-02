#!/bin/bash
# cpp -E bridge: preprocess a GNU Mes source file into ccc's dialect.
#   -D__M2__ selects Mes's simplest (freestanding) #if dialect.
#   strip cpp's '# line' markers (ccc has no preprocessor; those aren't C).
# usage: bridge.sh /tmp/mes/src/FILE.c  > FILE.clean.c
set -e
MES="${MES:-/tmp/mes}"
cpp -E -nostdinc -I "$MES/include" -D__M2__=1 -DMES_VERSION='"git"' "$1" | grep -vE '^#'
