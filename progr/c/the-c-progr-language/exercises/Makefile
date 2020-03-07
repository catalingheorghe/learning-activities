CC := gcc
CFLAGS := -Wall -Werror

BINDIR := ./bin

sources := $(wildcard *.c)
apps := $(patsubst %,$(BINDIR)/%,$(patsubst %.c,%,$(sources)))

.PHONY: all clean clean_all printvars

all: $(BINDIR) $(apps)

$(BINDIR):
	mkdir -p $@

$(BINDIR)/%: %.c
	$(CC) $(CFLAGS) -o $@ $<

clean: clean_all

clean_all:
	-rm -rf $(BINDIR)

printvars:
	@echo $(sources)
	@echo $(apps)

