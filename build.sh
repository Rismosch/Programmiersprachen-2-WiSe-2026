#!/usr/bin/env bash
set -euo pipefail

# run twice to resolve references
ASSIGNMENT="solution05.tex"
pdflatex -interaction=nonstopmode -shell-escape $ASSIGNMENT
pdflatex -interaction=nonstopmode -shell-escape $ASSIGNMENT