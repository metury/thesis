#!/usr/bin/env python3

def comet(filepath, path_len, star_size, k, s):
	with open(filepath, "w") as f:
		f.write(f"s={s} k={k}")
		for i in range(1, path_len):
			f.write(f"[{i-1};{i}]")
		f.write(f"[{0};{path_len}]")
		for i in range(star_size):
			f.write(f"[{path_len};{path_len + i + 1}]")

def clique(filepath, n, k, s):
	with open(filepath, "w") as f:
		f.write(f"s={s} k={k}")
		for i in range(n):
			for j in range(i+1, n):
				f.write(f"[{i};{j}]")

def star(filepath, n, k, s):
	with open(filepath, "w") as f:
		f.write(f"s={s} k={k}")
		for i in range(1, n+1):
			f.write(f"[{0};{i}]")

if __name__ == "__main__":
	comet("graphs/comet.in", 10, 10, 12, 0)
	print("comet")
	comet("graphs/comet2.in", 10, 10, 10, 12)
	print("comet2")
	clique("graphs/clique.in", 8, 3, 0)
	print("clique")
	star("graphs/star.in", 20, 4, 1)
	print("star")
	comet("graphs/path.in", 10, 10, 6, 0)
	print('path')
