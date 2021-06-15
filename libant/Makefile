CC = clang
CLFAGS = -Wall

SRCFILES = $(wildcard ./*.c)
OBJFILES = $(SRCFILES:c=o)

TESTSRC = $(wildcard tests/*.c)
TESTBIN = $(TESTSRC:.c=)

all: tests lib

lib: $(OBJFILES)
	ar -rc libant.a $(OBJFILES)

tests: lib $(TESTBIN)

$(TESTBIN): $(TESTSRC)
	$(CC) -o $@ $< $(CFLAGS) -L. -lant

.c.o:
	$(CC) -o $@ -c $< $(CFLAGS)

clean:
	find . -name "*.o" | xargs rm -f
	rm -f libant.a
	rm -f $(TESTBIN)
