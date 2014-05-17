RUSTC = rustc -O src/eulermark.rs
RUSTC_NT = rustc $(QUIET) --no-trans
srcs = $(wildcard problems/*/*.rs)

.PHONY: all bench clean test

all:
	mkdir -p bin
	$(RUSTC) --out-dir bin

bench:
	mkdir -p metrics
	bin/eulermark

clean:
	rm -rf {bin,metrics}

test:
	$(foreach src,$(srcs),$(RUSTC_NT) $(src) || exit;)
	./check-line-length.sh
