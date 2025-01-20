#!/bin/bash

PROJECT_NAME="rusty-ripper"
BINARY_PATH="./target/release/${PROJECT_NAME}"
TEST_DIR="./tests"

# Function to compare test output with expected output
# Args:
#   $1: test number
#   $2: optional 'sort' flag
compare_files() {
    local test_num=$1
    local do_sort=$2
    
    if [ "$do_sort" = "sort" ]; then
        sort "${TEST_DIR}/${test_num}-output.txt" | diff "${TEST_DIR}/${test_num}-expected.txt" - &>/dev/null
        diff_status=$?
    else
        diff "${TEST_DIR}/${test_num}-output.txt" "${TEST_DIR}/${test_num}-expected.txt" &>/dev/null
        diff_status=$?
    fi

    if [ $diff_status -eq 0 ]; then
        echo "= ✅ Test ${test_num} PASSED"
    else
        echo "= ❌ Test ${test_num} FAILED: Differences found: check ${test_num}-diff.txt $([ "$do_sort" = "sort" ] && echo "(note: output was firstly sorted.)")"
        if [ "$do_sort" = "sort" ]; then
            sort "${TEST_DIR}/${test_num}-output.txt" | diff -u "${TEST_DIR}/${test_num}-expected.txt" - > "${TEST_DIR}/${test_num}-diff.txt"
        else
            diff -u "${TEST_DIR}/${test_num}-output.txt" "${TEST_DIR}/${test_num}-expected.txt" > "${TEST_DIR}/${test_num}-diff.txt"
        fi
    fi
}

# Build function
build() {
    cargo build --release
}

# Test function
run_tests() {
    build

    echo "== Running basic functionality tests..."
    
    # Test 001: MD5 hash test with dictionary
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash md5 -p "${TEST_DIR}/passwords/passwords-md5.csv" > "${TEST_DIR}/001-output.txt"
    compare_files "001" "sort"

    # Test 002: Basic test without parameters
    ${BINARY_PATH} > "${TEST_DIR}/002-output.txt"
    compare_files "002"

    # Test 003: SHA256 hash test
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha256 -p "${TEST_DIR}/passwords/passwords-sha256.csv" > "${TEST_DIR}/003-output.txt"
    compare_files "003" "sort"

    # Test 004: SHA512 hash test
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha512 -p "${TEST_DIR}/passwords/passwords-sha512.csv" > "${TEST_DIR}/004-output.txt"
    compare_files "004" "sort"

    echo "== Running generate mode tests..."
    
    # Test 005-007: Generate mode tests with different hashes
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash md5 --generate > "${TEST_DIR}/005-output.txt"
    compare_files "005"

    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha256 --generate > "${TEST_DIR}/006-output.txt"
    compare_files "006"

    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha512 --generate > "${TEST_DIR}/007-output.txt"
    compare_files "007"

    echo "== Running error handling tests..."
    
    # Test 008: Incorrect hash type
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash INCORRECT_HASH > "${TEST_DIR}/008-output.txt"
    compare_files "008"

    # Test 009: Non-existing dictionary file
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/NON-EXISTING-FILE.TXT" --hash sha512 --generate > "${TEST_DIR}/009-output.txt"
    compare_files "009"

    # Test 010: Non-existing password file
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha256 -p "${TEST_DIR}/passwords/NON-EXISTING-PASSWORD-FILE.csv" > "${TEST_DIR}/010-output.txt"
    compare_files "010"

    echo "== Running salt tests..."
    
    # Test 011-013: Salt tests with different hashes
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash md5 --generate --salt saltstring > "${TEST_DIR}/011-output.txt"
    compare_files "011"

    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha256 --generate --salt saltstring > "${TEST_DIR}/012-output.txt"
    compare_files "012"

    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha512 --generate --salt saltstring > "${TEST_DIR}/013-output.txt"
    compare_files "013"

    echo "== Running additional test scenarios..."
    
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash SHA256 --generate > "${TEST_DIR}/014-output.txt"
    compare_files "014"

    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-empty.txt" --hash sha256 --generate > "${TEST_DIR}/015-output.txt"
    compare_files "015"

    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-invalid.txt" --hash sha256 --generate > "${TEST_DIR}/016-output.txt"
    compare_files "016"

    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha256 -p "${TEST_DIR}/passwords/passwords-invalid.csv" > "${TEST_DIR}/017-output.txt"
    compare_files "017"

    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha256 --generate --salt "%^&*()" > "${TEST_DIR}/018-output.txt"
    compare_files "018"

    # Test 019: Dictionary and hash flags without generate flag
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha256 > "${TEST_DIR}/019-output.txt"
    compare_files "019"

    # Test 020: Invalid flag combination (generate with passwords)
    ${BINARY_PATH} -d "${TEST_DIR}/dictionaries/dictionary-simple.txt" --hash sha256 -p "${TEST_DIR}/passwords/passwords-sha256.csv" --generate > "${TEST_DIR}/020-output.txt"
    compare_files "020" "sort"
}

clean() {
    rm -f "${TEST_DIR}"/[0-9][0-9][0-9]-output.txt
    rm -f "${TEST_DIR}"/[0-9][0-9][0-9]-diff.txt
}

# Help function
show_help() {
    echo "Available commands:"
    echo "  build   : Compile the project in release mode"
    echo "  test    : Run tests after building"
    echo "  clean   : Remove build artifacts and test outputs"
    echo "  help    : Show this help message"
}

case "${1:-test}" in
    "build")
        build
        ;;
    "test")
        run_tests
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