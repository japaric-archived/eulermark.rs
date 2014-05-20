RUSTC = rustc -O src/eulermark.rs
RUSTC_NT = rustc --no-trans
srcs = $(wildcard problems/*/*.rs)

.PHONY: all bench book clean plot test

all:
	mkdir -p bin
	$(RUSTC) --out-dir bin

bench:
	cd template && npm install posix-clock
	mkdir -p hashes
	mkdir -p metrics
	bin/eulermark

book:
	./build-book.sh

clean:
	rm -rf {bin,hashes,metrics,plots,stage,template/node_modules}

plot:
	./plot.py

test:
	$(foreach src,$(srcs),$(RUSTC_NT) $(src) || exit;)
	./check-line-length.sh
