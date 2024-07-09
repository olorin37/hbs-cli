#!/usr/bin/env bash
exec "$command_under_test" data.yaml standalone.hbs -o "$output_file"
