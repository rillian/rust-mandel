all: mandel

check: all
	./mandel

clean:
	-$(RM) mandel

dist:
	@echo $@ target not yet implemented

.PHONEY: all check clean dist

mandel: src/mandel.rs
	rustc $<
