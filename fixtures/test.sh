#!/bin/sh

## run it from /fixtures


# standalone template
../target/debug/hbs-cli data.yaml standalone.hbs -o standalone.txt
diff -u out_standalone.txt standalone.txt

# with partials registration
../target/debug/hbs-cli data.yaml template.hbs -r '**/*.hbs' -o partials.txt
diff -u out.txt partials.txt
