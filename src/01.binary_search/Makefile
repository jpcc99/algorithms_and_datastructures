PROJECT_NAME=binary_search

SRC_FILES=$(wildcard *.c)
H_FILES=$(wildcard *.h)
OBJ=$(SRC_FILES:.c=.o)

CC=clang
CC_FLAGS=-c		\
		-Wall	\
		-pedantic

all: $(PROJECT_NAME)

$(PROJECT_NAME): $(OBJ)
	$(CC) -o $@ $^

%.o: %.c %.h 
	$(CC) -o $@ $< $(CC_FLAGS)

main.o: main.c $(H_FILES)
	$(CC) -o $@ $< $(CC_FLAGS)

clean: 
	rm -rf *.o $(PROJECT_NAME) *~

.PHONY: all clean