CC = clang
CFLAGS = -Wall -g

SRCFILES = $(wildcard ./*.c)
OBJFILES = $(SRCFILES:c=o)

TESTSRC = $(wildcard tests/*.c)
TESTBIN = $(TESTSRC:.c=)

all: lib tests

lib: $(SRCFILES)
	$(CC)  $(CFLAGS) -fPIC -c $^
	$(CC) -shared -Wl,-soname,libant.so -o libant.so $(OBJFILES)

install: lib
	cp -f libant.so /usr/lib
	chmod 755 /usr/lib/libant.so

uninstall:
	rm -f /usr/lib/libant.so

tests: $(TESTBIN)

$(TESTBIN): $(TESTSRC)
	$(CC) -o $@ $@.c $(CFLAGS) -L. -lant

.c.o:
	$(CC) -o $@ -c $< $(CFLAGS)

clean:
	find . -name "*.o" | xargs rm -f
	rm -f libant.so
	rm -f $(TESTBIN)

.PHONY: all tests lib install clean uninstall
