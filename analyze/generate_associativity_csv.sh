#!/bin/bash

cd ../associativity

# various parameters for the experiments
RANGE_MINS=(-1000000 -1 0)
RANGE_MAXS=(0 1 1000000)
SAMPLES=(1000 1000000)
SEEDS=(42 40 12345)
FLOAT_TYPES=(32 64)
OUTPUT_FILE="../analyze/associativity_data.csv"

# clear output file
echo "" > "$OUTPUT_FILE"

for RANGE_MIN in "${RANGE_MINS[@]}"; do
    for RANGE_MAX in "${RANGE_MAXS[@]}"; do
        if [ "$RANGE_MIN" -eq "$RANGE_MAX" ]; then
            continue
        fi

        for SAMPLE in "${SAMPLES[@]}"; do
            for SEED in "${SEEDS[@]}"; do
                for FLOAT_TYPE in "${FLOAT_TYPES[@]}"; do
                    echo "Running associativity test with range_min=$RANGE_MIN, range_max=$RANGE_MAX, samples=$SAMPLE, seed=$SEED, float_type=$FLOAT_TYPE"
                    cargo run --release -- --range-min "$RANGE_MIN" --range-max "$RANGE_MAX" --samples "$SAMPLE" --seed "$SEED" --float-type "$FLOAT_TYPE" --csv --output "$OUTPUT_FILE"
                done
            done
        done
    done
done