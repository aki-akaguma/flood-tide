#!/usr/bin/make
rustc_vers = 1.56.1 1.57.0 1.58.1 1.59.0 1.60.0 1.61.0 1.62.1 1.63.0 \
	1.64.0 1.65.0 1.66.1
target_base = x86_64-unknown-linux-gnu i586-unknown-linux-gnu

define test-rustc-templ =
target/stamp/stamp.test-rustc.$(1).$(2):
	@mkdir -p target/stamp
	cargo +$(1) test --target=$(2)
	@touch target/stamp/stamp.test-rustc.$(1).$(2)
endef

example_curl_gen = examples/curl.cmd.match.rs.txt examples/curl.cmd.help.rs.txt
empty:=
space:=$(empty) $(empty)
comma:=,

features = was_long single_error abbreviate subcommand long_only

include features_comb.mk

define template =
target/z.test/z.test-$(1).log: src/lib.rs
	time cargo test --no-default-features --features $(2) > z.test-$(1).tmp
	@mkdir -p target/z.test
	@cat z.test-$(1).tmp > target/z.test/z.test-$(1).log
	@grep -e "^test result:" z.test-$(1).tmp | grep -v -e " 0 passed" >> target/z.test/z.test-$(1).log
	@rm -f z.test-$(1).tmp
	@echo
endef


all: readme
	@echo "make [clean|test|readme]"

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

clean:
	-@rm -f *.log
	-@rm -f *.tmp
	-@cargo clean

test:
	cargo test

doc:
	cargo doc --features dox

gen: $(example_curl_gen)

gen-clean:
	rm -f $(example_curl_gen)

examples/curl.cmd.match.rs.txt: examples/curl.cmd.txt
	cargo xtask gen-src-example-curl-cmd

examples/curl.cmd.help.rs.txt: examples/curl.cmd.txt
	cargo xtask gen-src-example-curl-cmd

bench:
	cargo xbench --bench=bench-curl

test-all-features: $(LOGS)

$(foreach log,$(features_comb),$(eval LOGS=$(LOGS) target/z.test/z.test-$(log).log))
$(foreach log,$(features_comb),$(eval $(call template,$(log),$(subst +,$(comma),$(log)))))

test-all-version: $(foreach ver,$(rustc_vers),$(foreach tb,$(target_base),target/stamp/stamp.test-rustc.$(ver).$(tb)))

test-all-version-clean:
	@rm -fr target/stamp

$(foreach ver,$(rustc_vers),$(eval $(foreach tb,$(target_base),$(eval $(call test-rustc-templ,$(ver),$(tb))))))
