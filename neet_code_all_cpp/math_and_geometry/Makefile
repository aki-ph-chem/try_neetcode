CXX = g++
file = ""

ver = 20
OPT = -std=c++${ver} 

${file}.out : ${file}.cpp
	${CXX} -o $@ $< ${OPT}

run: ${file}.out
	./${file}.out

clean:
	rm ${file}.out

clean_all: 
	rm *.out

.PHONY: run clean clean_all
