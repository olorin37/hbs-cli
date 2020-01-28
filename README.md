Simple Handlebars CLI (in Rust)
===============================

Simple CLI for generating text from handlebars templates, by feeding
them with data from file (YAML parser used is for it, so JSON is also
supported), writen in Rust programming language (with usage serde~yaml~
and handlebars crates).

Usage
-----

```bash
hbs-cli <properties file> <template file> > <output file>
```

Where `<properties file>` can be ether YAML or JSON (as YAML parser is a
JSON parser too) and `<template file>` is handlebars template. Template
is generated on standard output.

Building
--------

```bash
cargo build --release  # for release binary
```
