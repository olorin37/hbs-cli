#!/usr/bin/env bash

declare -x command_under_test
command_under_test=$(git rev-parse --show-toplevel)
command_under_test+=/target/debug/hbs-cli

declare base_dir
base_dir=$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")

declare -a tests
mapfile -t tests < <(
        find "$base_dir" -name '*.command'
)

declare -x output_file
declare -i rc
echo "Testing:"
for t in "${tests[@]}"; do
	t=$(readlink -f "$t")
	output_file=${t/.command/.output}
	rc=9
	(
                cd "${base_dir}/fixtures"
	        bash "${t}" && 
		diff -u "${output_file/.output/.expected}" "${output_file}" 
	)
	rc=$?
	echo "    [$rc] $(basename ${t%.command} | tr '_' ' ')"
	if (( rc )); then exit "$rc"; fi
done

find "$(dirname "$0")" -name '*.output' -delete 
