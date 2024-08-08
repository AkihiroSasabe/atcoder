import io
import sys
from collections import defaultdict, deque, Counter
from itertools import permutations, combinations, accumulate
from heapq import heappush, heappop
from bisect import bisect_right, bisect_left
from math import gcd
import math

_INPUT = """\
6
missisippi
5
i
s
a
is
missisippi
aaaaaa
6
a
aa
aaa
aaaa
aaaaa
aaaaaa
"""
def sais(s):
    uniq = list(set(s))
    uniq.sort()
    return sais_rec(list(map({e: i+1 for i, e in enumerate(uniq)}.__getitem__, s)), len(uniq))
def sais_rec(lst, num):
    L = len(lst)
    if L < 2:
        return lst + [0]
    lst = lst + [0]
    L += 1
    res = [-1] * L
    t = [1] * L
    for i in range(L-2, -1, -1):
        t[i] = 1 if (lst[i] < lst[i+1] or (lst[i] == lst[i+1] and t[i+1])) else 0
    isLMS = [t[i-1] < t[i] for i in range(L)]
    LMS = [i for i in range(1, L) if t[i-1] < t[i]]
    LMSn = len(LMS)

    count = Counter(lst)
    tmp = 0
    cstart = [0]*(num+1)
    cend = [0]*(num+1)
    for key in range(num+1):
        cstart[key] = tmp
        cend[key] = tmp = tmp + count[key]

    cc_it = [iter(range(e-1, s-1, -1)) for s, e in zip(cstart, cend)]
    for e in reversed(LMS):
        res[next(cc_it[lst[e]])] = e

    cs_it = [iter(range(s, e)) for s, e in zip(cstart, cend)]
    ce_it = [iter(range(e-1, s-1, -1)) for s, e in zip(cstart, cend)]
    for e in res:
        if e > 0 and not t[e-1]:
            res[next(cs_it[lst[e-1]])] = e-1
    for e in reversed(res):
        if e > 0 and t[e-1]:
            res[next(ce_it[lst[e-1]])] = e-1

    name = 0; prev = -1
    pLMS = {}
    for e in res:
        if isLMS[e]:
            if prev == -1 or lst[e] != lst[prev]:
                name += 1; prev = e
            else:
                for i in range(1, L):
                    if lst[e+i] != lst[prev+i]:
                        name += 1; prev = e
                        break
                    if isLMS[e+i] or isLMS[prev+i]:
                        break
            pLMS[e] = name-1

    if name < LMSn:
        sublst = [pLMS[e] for e in LMS if e < L-1]
        ret = sais_rec(sublst, name-1)

        LMS = list(map(LMS.__getitem__, reversed(ret)))
    else:
        LMS = [e for e in reversed(res) if isLMS[e]]

    res = [-1] * L

    cc_it = [iter(range(e-1, s-1, -1)) for s, e in zip(cstart, cend)]
    for e in LMS:
        res[next(cc_it[lst[e]])] = e

    cs_it = [iter(range(s, e)) for s, e in zip(cstart, cend)]
    ce_it = [iter(range(e-1, s-1, -1)) for s, e in zip(cstart, cend)]

    for e in res:
        if e > 0 and not t[e-1]:
            res[next(cs_it[lst[e-1]])] = e-1
    for e in reversed(res):
        if e > 0 and t[e-1]:
            res[next(ce_it[lst[e-1]])] = e-1
    return res

# Longest Common Prefix
# (文字列s, 文字列長n, Suffix Array)を引数として与える
def LCP(s, n, sa):
    lcp = [-1]*(n+1)
    rank = [0]*(n+1)
    for i in range(n+1): rank[sa[i]] = i

    h = 0
    lcp[0] = 0
    for i in range(n):
        j = sa[rank[i] - 1]
        if h > 0: h -= 1
        while j+h < n and i+h < n and s[j+h]==s[i+h]:
            h += 1
        lcp[rank[i] - 1] = h
    return lcp

def input():
  return sys.stdin.readline()[:-1]

def solve(test):
  S=input()
  Q=int(input())
  T=[input() for _ in range(Q)]
  sa=sais(S+'{'+''.join([T[i//2]+'`' if i%2==0 else T[i//2]+'}' for i in range(2*Q)]))
  start=[len(S)+1]
  end=[len(S)+len(T[0])+2]
  sd={start[0]:0}
  ed={end[0]:0}
  for i in range(Q-1):
    start.append(start[-1]+2*len(T[i])+2)
    end.append(end[-1]+len(T[i])+len(T[i+1])+2)
    sd[start[-1]]=i+1
    ed[end[-1]]=i+1
  start=set(start)
  end=set(end)
  ans=[0]*Q
  now=0
  for i in range(len(sa)):
    if sa[i]<len(S):
      now+=1
    elif sa[i] in start:
      ans[sd[sa[i]]]-=now
    elif sa[i] in end:
      ans[ed[sa[i]]]+=now
  print(*ans,sep='\n')

def random_input():
  from random import randint,shuffle
  N=randint(1,10)
  M=randint(1,N)
  A=list(range(1,M+1))+[randint(1,M) for _ in range(N-M)]
  shuffle(A)
  return (" ".join(map(str, [N,M]))+"\n"+" ".join(map(str, A))+"\n")*3

def simple_solve():
  return []

def main(test):
  if test==0:
    solve(0)
  elif test==1:
    sys.stdin = io.StringIO(_INPUT)
    case_no=int(input())
    for _ in range(case_no):
      solve(0)
  else:
    for i in range(1000):
      sys.stdin = io.StringIO(random_input())
      x=solve(1)
      y=simple_solve()
      if x!=y:
        print(i,x,y)
        print(*[line for line in sys.stdin],sep='')
        break

#0:提出用、1:与えられたテスト用、2:ストレステスト用
main(0)