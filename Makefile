PROJECT_NAME := rusty-ripper
BINARY_PATH := ./target/release/$(PROJECT_NAME)
TEST_DIR := ./tests

# Test Comparison Function
# Usage: $(call compare,test_number[,sort])
# Expects files in $(TEST_DIR)/
# 							{test_number}-expected.txt,
# 							{test_number}-output.txt
# Optional second argument 'sort' will sort both files before comparing
define compare
	@if $(if $(2),sort $(TEST_DIR)/$(1)-output.txt | diff $(TEST_DIR)/$(1)-expected.txt -,diff $(TEST_DIR)/$(1)-output.txt $(TEST_DIR)/$(1)-expected.txt); then \
		echo "✅ Test $(1) PASSED"; \
	else \
		echo "❌ Test $(1) FAILED: Differences found: check $(1)-diff.txt $(if $(2),(note: output was firstly sorted.),)"; \
		$(if $(2),sort $(TEST_DIR)/$(1)-output.txt | diff -u $(TEST_DIR)/$(1)-expected.txt -,diff -u $(TEST_DIR)/$(1)-output.txt $(TEST_DIR)/$(1)-expected.txt) > $(TEST_DIR)/$(1)-diff.txt; \
	fi
endef


.PHONY: all
all: build

.PHONY: build
build:
	cargo build --release

.PHONY: test
test: build
	$(BINARY_PATH) -d $(TEST_DIR)/dictionaries/dictionary-simple.txt --hash md5 -p $(TEST_DIR)/passwords/passwords-md5.csv > $(TEST_DIR)/001-output.txt
	$(call compare,001,sort)
	$(BINARY_PATH)  > $(TEST_DIR)/002-output.txt
	$(call compare,002)
	$(BINARY_PATH) -d $(TEST_DIR)/dictionaries/dictionary-simple.txt --hash sha256 -p $(TEST_DIR)/passwords/passwords-sha256.csv > $(TEST_DIR)/003-output.txt
	$(call compare,003,sort)
	$(BINARY_PATH) -d $(TEST_DIR)/dictionaries/dictionary-simple.txt --hash sha512 -p $(TEST_DIR)/passwords/passwords-sha512.csv > $(TEST_DIR)/004-output.txt
	$(call compare,004,sort)

	
.PHONY: clean
clean:
	# cargo clean
	rm -f $(TEST_DIR)/[0-9][0-9][0-9]-output.txt
	rm -f $(TEST_DIR)/[0-9][0-9][0-9]-diff.txt

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all     : Build the project (default)"
	@echo "  build   : Compile the project in release mode"
	@echo "  test    : Run tests after building"
	@echo "  clean   : Remove build artifacts and test outputs"
	@echo "  help    : Show this help message"