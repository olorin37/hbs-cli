#!/bin/sh

## run it from /fixtures

# no templates registration
../target/debug/hbs-cli data.yaml template.hbs -o no_registration.txt
diff out_no_registration.txt no_registration.txt

# with registration
../target/debug/hbs-cli data.yaml template.hbs -r '**/*.hbs' -o a.txt
diff out.txt a.txt
