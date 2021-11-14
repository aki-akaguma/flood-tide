#!/usr/bin/make

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

$(foreach log,$(features_comb),$(eval LOGS=$(LOGS) target/z.test/z.test-$(log).log))

all:
	@echo "make [clean|test|readme]"

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test: $(LOGS)

clean:
	-@rm -f *.log
	-@rm -f *.tmp
	-@cargo clean

doc:
	cargo doc --features dox

gen: $(example_curl_gen)

gen-clean:
	rm -f $(example_curl_gen)

examples/curl.cmd.match.rs.txt: examples/curl.cmd.txt
	#./scripts/gen-parser-curl.pl gen_match > $@
	cargo xtask gen-src-example-curl-cmd-match

examples/curl.cmd.help.rs.txt: examples/curl.cmd.txt
	#./scripts/gen-parser-curl.pl gen_help > $@
	cargo xtask gen-src-example-curl-cmd-help

$(foreach log,$(features_comb),$(eval $(call template,$(log),$(subst +,$(comma),$(log)))))
