# https://atcoder.jp/contests/abc363/submissions/55779173
from heapq import*
(h,w,y),*A = [[*map(int,t.split())] for t in open(0)]
Q = []
seen = [[0]*w for _ in range(h)]
for i in range(h):
  for j in range(w):
    if i == 0 or i == h-1 or j == 0 or j == w-1:
      seen[i][j] = 1
      heappush(Q,(A[i][j],(i,j)))
ans = h*w
for t in range(1,y+1):
  while Q and Q[0][0] <= t:
    _,(i,j) = heappop(Q)
    ans -= 1
    for dx,dy in ((-1,0),(0,-1),(1,0),(0,1)):
      nx,ny = i+dx,j+dy
      if not (0 <= nx < h and 0 <= ny < w):
        continue
      if seen[nx][ny]:
        continue
      seen[nx][ny] = 1
      heappush(Q,(A[nx][ny],(nx,ny)))
  print(ans)