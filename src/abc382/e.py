import sys
import threading
import math

def main():
    import numpy as np

    N, X = map(int, sys.stdin.readline().split())
    P = list(map(int, sys.stdin.readline().split()))

    max_pack = 5000  # 最大パック数を適当に設定

    # 1パック内でのレアカード数の確率分布を計算
    prob = np.zeros(N + 1)
    prob[0] = 1.0
    for p in P:
        p /= 100.0
        prob[1:] = prob[1:] * (1 - p) + prob[:-1] * p
        prob[0] *= (1 - p)

    # 累積レアカード数が X 未満である確率を計算し、期待値を求める
    dp = {0: 1.0}
    expected_packs = 0.0
    for k in range(max_pack):
        new_dp = {}
        total_prob = 0.0
        for s, v in dp.items():
            for i in range(len(prob)):
                ns = s + i
                if ns >= X:
                    continue
                nv = v * prob[i]
                if ns in new_dp:
                    new_dp[ns] += nv
                else:
                    new_dp[ns] = nv
                total_prob += nv
        expected_packs += total_prob
        if total_prob < 1e-12:
            break
        dp = new_dp

    expected_packs += 1  # 最後のパック

    print("%.15f" % expected_packs)

if __name__ == '__main__':
    threading.Thread(target=main).start()
