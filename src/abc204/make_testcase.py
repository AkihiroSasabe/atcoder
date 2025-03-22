
import random
import numpy as np

def main():
    example()
    # example_2d_darray()
    # example_2d_darray_2()
    # exmaple_tree()

def example():
    # テストケースを作る関数
    # n
    # s1
    # s2
    # ...
    # sn
    # の形式で、出力する。

    # 以下例
    n_max = 10
    # k_max = 100

    n = random.randrange(1, n_max)
    m = random.randrange(1, n_max)
    # k = random.randrange(1, k_max)

    print(n, m)
    for i in range(m):
        ai = random.randrange(1, n+1)
        bi = random.randrange(1, n+1)
        ci = random.randrange(0, 100)
        di = random.randrange(0, 100)
        print(ai, bi, ci, di)
    return


def example_2d_darray():
    # テストケースを作る関数
    # h w
    # A
    # の形式で、出力する。

    # 以下例
    h_max = 10
    w_max = 10

    h = random.randrange(1, h_max)
    w = random.randrange(1, w_max)
    A = np.random.randint(1, 40, size=(h,w))

    print(h, w)
    print_array(A)
    
    # 実行結果の例
    # 6 7
    # 6 19 22 14 20 37 20
    # 3 34 30 36 39 15 30
    # 8 1 32 31 5 37 25
    # 14 20 30 4 15 16 17
    # 36 3 27 39 20 29 25
    # 7 12 4 3 6 20 31

def example_2d_darray_2():
    """
    以下のテストケースを作るコードである
    入力
    N K P
    C1 A11 A12 ... A1K
    C2 A21 A22 ... A2K
    ...
    CN AN1 AN2 ... ANK

    入力例
    4 3 5
    5 3 0 2
    3 1 2 3
    3 2 4 0
    1 0 1 4
    """

    n = random.randrange(1, 101)
    k = random.randrange(1, 6)
    p = random.randrange(1, 6)

    cs = [random.randrange(1, 1_000_000_001) for i in range(n)]
    a = [[random.randrange(0, p+1) for j in range(k)] for i in range(n)]

    # n = 100
    # k = 5
    # p = 5

    # cs = [1_000_000_000 for i in range(n)]
    # a = [[1, 1, 1, 1, 1] for i in range(n)]

    print(n, k, p)
    for i in range(n):
        print("{}".format(cs[i]), end=' ')
        for j in range(len(a[i])):
            print("{}".format(a[i][j]), end=' ')
        print("")

def exmaple_tree():
    # 木のテストケースを作る関数
    # 例
    # N
    # u_1 v_1
    # u_2 v_2
    # u_3 v_3
    # ...
    # u_N-1 v_N-1

    n_max = 15
    n = random.randrange(10, n_max)

    uvs = [[1, 2]]
    for u in range(3, n+1):
        v = np.random.randint(1, u)
        uvs.append([u, v])
    
    print(n)
    uvs = np.array(uvs)
    print_2d_ndarray(uvs)

def print_array(array):
    if len(array.shape) == 1:
        print_1d_ndarray(array)
    elif len(array.shape) == 2:
        print_2d_ndarray(array)

def print_1d_ndarray(array):
    # Numpyの1次元配列をprintする関数
    
    # joinは、Pythonの文字列メソッドの一つで、
    # リストやタプルなどの「文字列の要素」を特定の区切り文字で
    # 結合して一つの文字列を作るために使います。
    # "区切り文字".join(文字列のリスト)
    print(" ".join(map(str, array))) # 空白文字で区切って表示

def print_2d_ndarray(array):
    # Numpyの2次元配列をprintする関数
    for row in array:
        print(' '.join(map(str, row)))


if __name__ == "__main__":
    main()