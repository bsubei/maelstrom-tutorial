#!/bin/bash

if [[ "$1" == "nemesis" ]]; then 
    cargo build && ./maelstrom/maelstrom test -w broadcast --bin ./target/debug/maelstrom-tutorial --time-limit 20 --topology line --nemesis partition --log-stderr 
elif [ "$1" = "loop" ]; then
    for i in {1..10}; do
        echo "Attempt $i"
        output=$(cargo build && ./maelstrom/maelstrom test -w broadcast --bin ./target/debug/maelstrom-tutorial --time-limit 20 --nemesis partition --topology tree4 --log-stderr)
        last_line=$(echo "$output" | tail -n 1)

        echo "$last_line"

        if [ $? -ne 0 ] || echo "$last_line" | grep -q "invalid"; then
            echo "Command failed on attempt $i, aborting..."
            echo "Command output was:\n$output"
            exit 1
        fi
    done
    echo "All attempts completed successfully"
fi