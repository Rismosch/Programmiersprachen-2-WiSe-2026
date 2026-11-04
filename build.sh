#!/usr/bin/env bash
set -euo pipefail

# run twice to resolve references
lualatex -shell-escape solution04.tex
lualatex -shell-escape solution04.tex
