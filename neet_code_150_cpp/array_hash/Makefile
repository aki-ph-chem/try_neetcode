CXX = g++
file = ""

${file}.out : ${file}.cpp
	${CXX} -o $@ $<

run: ${file}.out
	./${file}.out

clean:
	rm ${file}.out

clean_all: 
	rm *.out

.PHONY: run clean clean_all
