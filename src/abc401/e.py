import matplotlib.pyplot as plt
import networkx as nx

# 入力データ（1-indexed を 0-indexed に変換）
edge_list_str = """
6 9
5 9
2 3
1 9
10 11
4 5
9 10
8 9
7 8
3 5
1 7
6 10
4 7
7 9
1 10
4 11
3 8
2 7
3 4
1 8
2 8
3 7
2 10
1 6
6 11
"""

edges = [(int(a)-1, int(b)-1) for a, b in (line.split() for line in edge_list_str.strip().splitlines())]

# グラフ作成
G = nx.Graph()
G.add_edges_from(edges)

# ノード数取得
n = max(max(u, v) for u, v in edges) + 1

# レイアウト生成（spring_layout を 0 を左に寄せるよう調整）
# pos = nx.spring_layout(G, seed=42)  # 安定化のため seed 固定
pos = nx.spring_layout(G) 
# pos = nx.nx_agraph.graphviz_layout(G, prog="dot")
min_x = min(p[0] for p in pos.values())
# 頂点0のx座標を一番左にするために、他の座標をずらす
offset = pos[0][0] - min_x
for k in pos:
    pos[k] = (pos[k][0] - offset, pos[k][1])

# グラフ描画
plt.figure(figsize=(10, 6))
nx.draw(G, pos, with_labels=True, node_color='skyblue', edge_color='gray', node_size=700, font_size=10)
plt.title("Undirected Graph with Node 0 on the Left")
plt.axis("off")
# plt.show()
plt.savefig("e.png")