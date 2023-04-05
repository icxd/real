SRCS := $(shell find . -name "*.real")

.PHONY: all
all: $(SRCS)
	$(foreach src, $(SRCS), cargo run -q $(src);)