#!/usr/bin/env bash

# Copyright (C) 2023 Gramine contributors
# SPDX-License-Identifier: BSD-3-Clause

mkdir -p testdir

times=$1
[ "$times" -gt 0 ] 2>/dev/null || times=300

for (( c=1; c<=times; c++ ))
do
	 echo "hello $c"
        cp ./fibonacci ./testdir/fibonacci
        cp ./riscv32im-succinct-zkvm-elf ./testdir/riscv32im-succinct-zkvm-elf
        chmod +x ./testdir/fibonacci
        # chmod +x ./testdir/riscv32im-succinct-zkvm-elf
        ls ./testdir/
        cd ./testdir
        ./fibonacci
        # Clean up
        cd ..
        rm -rf ./testdir/fibonacci ./testdir/riscv32im-succinct-zkvm-elf
        date +"current date is %D"
done
