#!/usr/bin/env bash
exec "$command_under_test" data.yaml template.hbs -r '**/*.hbs' -o "$output_file"
