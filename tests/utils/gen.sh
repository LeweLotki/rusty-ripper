#!/bin/bash

# Function to generate pairs of strings and their SHA-512 hashes
generate_sha512_pairs() {
    for letter in {a..z}; do
        echo "$letter,$(echo -n "$letter" | sha512sum | awk '{print $1}')"
    done
}

generate_sha256_pairs() {
    for letter in {a..z}; do
        echo "$letter,$(echo -n "$letter" | sha256sum | awk '{print $1}')"
    done
}

generate_md5_pairs() {
    for letter in {a..z}; do
        echo "$letter,$(echo -n "$letter" | md5 | awk '{print $1}')"
    done
}
