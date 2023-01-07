import std/strutils
import std/sequtils

proc sum_fn(the_arr: seq[string]): int =
    result = the_arr
        .map(proc(x: string): int = x.parseInt)
        .foldl(a + b)

while true:
  let line = stdin.readline
  let arr = split(line)
  let sum = sum_fn(arr)
  echo sum
