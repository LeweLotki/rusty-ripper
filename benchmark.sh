#!/bin/bash

PROJECT_NAME="rusty-ripper"
BINARY_PATH="./target/release/${PROJECT_NAME}"
PERF_DIR="./performance"
ITERATIONS=5  # Number of runs for each test to get average performance

# Test configurations
DICT_SIZES=(1000 10000 100000 1000000)
HASH_TYPES=("md5" "sha256" "sha512")

# ANSI color codes for output
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Create performance directory structure
mkdir -p "$PERF_DIR"/{dictionaries,results}

# Generate dictionary of specified size
generate_dictionary() {
    local size=$1
    local output_file="$PERF_DIR/dictionaries/dict_${size}.txt"
    
    if [ ! -f "$output_file" ]; then
        echo -e "${CYAN}Generating dictionary of size $size...${NC}"
        for i in $(seq 1 $size); do
            echo "testpassword$i" >> "$output_file"
        done
    fi
}

# Format numbers with commas for readability
format_number() {
    printf "%'d" $1
}

# Measure performance for a specific configuration
measure_performance() {
    local dict_size=$1
    local hash_type=$2
    local iteration=$3
    local dict_file="$PERF_DIR/dictionaries/dict_${dict_size}.txt"
    
    # Warm-up run
    if [ "$iteration" -eq 1 ]; then
        $BINARY_PATH -d "$dict_file" --hash "$hash_type" --generate > /dev/null 2>&1
    fi
    
    # Timed run
    local start_time=$(date +%s.%N)
    $BINARY_PATH -d "$dict_file" --hash "$hash_type" --generate > /dev/null 2>&1
    local end_time=$(date +%s.%N)
    
    local duration=$(echo "$end_time - $start_time" | bc)
    local hashes_per_second=$(echo "scale=2; $dict_size / $duration" | bc)
    
    echo "$hashes_per_second"
}

# Calculate statistics from multiple runs
calculate_stats() {
    local -a speeds=("$@")
    local sum=0
    local min=${speeds[0]}
    local max=${speeds[0]}
    
    for speed in "${speeds[@]}"; do
        sum=$(echo "$sum + $speed" | bc)
        if (( $(echo "$speed < $min" | bc -l) )); then min=$speed; fi
        if (( $(echo "$speed > $max" | bc -l) )); then max=$speed; fi
    done
    
    local avg=$(echo "scale=2; $sum / ${#speeds[@]}" | bc)
    echo "$min $max $avg"
}

# Print results in a formatted table row
print_result_row() {
    local dict_size=$1
    local hash_type=$2
    local min=$3
    local max=$4
    local avg=$5
    
    printf "│ %-7s │ %-10s │ %'15.0f │ %'15.0f │ %'15.0f │\n" \
           "$hash_type" \
           "$(format_number $dict_size)" \
           ${min%.*} \
           ${max%.*} \
           ${avg%.*}
}

# Print table header
print_table_header() {
    echo "┌─────────┬────────────┬─────────────────┬─────────────────┬─────────────────┐"
    echo "│ Hash    │ Dictionary │     Min H/s     │     Max H/s     │     Avg H/s     │"
    echo "├─────────┼────────────┼─────────────────┼─────────────────┼─────────────────┤"
}

# Print table footer
print_table_footer() {
    echo "└─────────┴────────────┴─────────────────┴─────────────────┴─────────────────┘"
}

# Main function
main() {
    echo -e "${GREEN}Building rusty-ripper in release mode...${NC}"
    cargo build --release
    
    # Generate all required dictionaries
    for size in "${DICT_SIZES[@]}"; do
        generate_dictionary "$size"
    done
    
    echo -e "\n${YELLOW}Starting Performance Measurements${NC}"
    echo -e "${CYAN}Running $ITERATIONS iterations for each configuration${NC}\n"
    
    print_table_header
    
    for hash_type in "${HASH_TYPES[@]}"; do
        for dict_size in "${DICT_SIZES[@]}"; do
            # Collect performance data
            local -a speeds=()
            for i in $(seq 1 $ITERATIONS); do
                speed=$(measure_performance "$dict_size" "$hash_type" "$i")
                speeds+=($speed)
            done
            
            # Calculate statistics
            read min max avg <<< $(calculate_stats "${speeds[@]}")
            
            # Print results
            print_result_row "$dict_size" "$hash_type" "$min" "$max" "$avg"
        done
        
        # Add separator between hash types except for the last one
        if [ "$hash_type" != "${HASH_TYPES[-1]}" ]; then
            echo "├─────────┼────────────┼─────────────────┼─────────────────┼─────────────────┤"
        fi
    done
    
    print_table_footer
}

# Clean function
clean() {
    echo -e "${YELLOW}Cleaning performance measurement files...${NC}"
    rm -rf "$PERF_DIR"
    echo -e "${GREEN}Done${NC}"
}

# Help function
show_help() {
    echo "Rusty Ripper Performance Measurement Script"
    echo
    echo "Available commands:"
    echo "  run     : Run performance measurements (default)"
    echo "  clean   : Remove performance measurement files"
    echo "  help    : Show this help message"
}

case "${1:-run}" in
    "run")
        main
        ;;
    "clean")
        clean
        ;;
    "help")
        show_help
        ;;
    *)
        echo "Unknown command: $1"
        show_help
        exit 1
        ;;
esac