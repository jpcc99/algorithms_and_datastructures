project := quicksort
cc := clang
c_flags := -g			\
						-Wall	\
						-pedantic

o_dir := obj
src_dir := src
bin_dir := bin 

src_files := $(wildcard $(src_dir)/*.c)
h_files := $(wildcard $(src_dir)/*.h)
o_files := $(subst .c,.o,$(subst $(src_dir),$(o_dir),$(src_files)))

all: objDir binDir $(project)

$(project): $(o_files)
	$(cc) $(c_flags) -o bin/$@ $^ 

$(o_dir)/%.o: $(src_dir)/%.c $(src_dir)/%.h
	$(cc) $(c_flags) -c $< -o $@ 

$(o_dir)/main.o: $(src_dir)/main.c $(h_files)
	$(cc) $(c_flags) -c $< -o $@

objDir: 
	@ mkdir -p obj

binDir: 
	@ mkdir -p bin

.PHONY: all clean

clean:
	@ $(RM) -r $(o_dir) $(bin_dir) *~
