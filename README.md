Simple Handlebars CLI
=====================

Simple, but already useful CLI for generating text from handlebars
templates, by feeding them with data from file (YAML parser used is
for it, so JSON is also supported), writen in Rust programming language
(with usage serde_yaml and handlebars crates).

Goal
----
The idea is to prepare utility which should be easly used from
shell script.

Usage
-----

The simplest call:

```bash
hbs-cli <properties file> <template file> > <output file>
```
or 

```bash
hbs-cli <properties file> <template file> -o <output file>
```

Or with template registration for using them as partials:

```bash
hbs-cli <properties file> <template file> -r 'partials/**/*.hbs'
```

Where `<properties file>` can be ether YAML or JSON (as YAML parser is a
JSON parser too) and `<template file>` is handlebars template. Template
is generated on standard output.

To see more options call `hbs-cli --help`.

Building
--------

```bash
# to install rust toolchain, skip if already installed
rustup toolchain install stable 

cargo build --release
```

License
-------

This software is distributed under MIT license. See `LICENSE` file in the root of the repository.
