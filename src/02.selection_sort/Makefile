PROJECT_NAME=selection_sort
CC=clang
CC_FLAGS=-c 	\
		-Wall	\
		-pedantic

SRC_FILES=$(wildcard *.c)
H_FILES=$(wildcard *.h)
OBJ=$(SRC_FILES:.c=.o)

all: $(PROJECT_NAME)

$(PROJECT_NAME): $(OBJ)
	$(CC) -o $@ $^

%.o: %.c %.h 
	$(CC) -o $@ $< $(CC_FLAGS)

main.o: main.c $(H_FILES)
	$(CC) -o $@ $< $(CC_FLAGS)

clean:
	rm -rf *.o $(PROJECT_NAME) *~