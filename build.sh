#!/usr/bin/env bash
set -euo pipefail

# run twice to resolve references
ASSIGNMENT="solution05.tex"
lualatex -shell-escape $ASSIGNMENT
lualatex -shell-escape $ASSIGNMENT