#!/bin/sh

## run it from /fixtures

../target/debug/hbs-cli data.yaml template.hbs -o a.txt
diff out.txt a.txt
