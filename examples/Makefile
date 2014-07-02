OUT = 01_hello \
      02_sample \
      03_sample-filter \
      04_point \
      05_enum \
      08_enum-custom \
      10_vec \
      11_str \
      12_point-impl \
      13_trait \
      14_point-show \
      17_borrow-error-fixed \
      20_list \
      21_spawn

all : $(OUT)

1 : 01_hello
	./01_hello
2 : 02_sample
	./02_sample
3 : 03_sample-filter
	./03_sample-filter
4 : 04_point
	./04_point
5 : 05_enum
	./05_enum
6 : 06_enum-nonexhaustive
7 : 07_enum-unknown-value
8 : 08_enum-custom
	./08_enum-custom
10 : 10_vec
	./10_vec
11 : 11_str
	./11_str
12 : 12_point-impl
	./12_point-impl
13 : 13_trait
	./13_trait
14 : 14_point-show
	./14_point-show
16 : 16_borrow-error
17 : 17_borrow-error-fixed
	./17_borrow-error-fixed
18 : 18_ownership_error
19 : 19_list-error
20 : 20_list
	./20_list
21 : 21_spawn
	./21_spawn

% : %.rs
	rustc $<

clean:
	rm -f $(OUT)

.PHONY: clean
