#!/bin/bash
python3 ./gen_graph.py
cargo run
python3 ./draw_graph.py
