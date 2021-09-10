#!/usr/bin/python3
import sys
import os 

problem = sys.argv[1] + ".rs"
main = "main.rs"
cp = "cp {} {}".format(main, problem)
mv = "mv {} ../../kattis".format(problem)
os.system(cp)
os.system(mv)

