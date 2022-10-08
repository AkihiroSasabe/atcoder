import os

file1 = "e_test_case2.txt"
file2 = "input.txt"
file3 = "output.txt"

try:
    with open(file1) as f:
        l = f.readlines()
        # with open("input.txt") as fw:
        n = 10
        count = 0
        while True:
            with open(file2, mode='w') as fw:
                fw.write("10\n")

            with open(file2, mode='a') as fw:
                for i in range(1 + count * n, 11 + count * n):
                    # s = str.strip(l[i])
                    s = l[i]
                    fw.write(s)
            os.system('bash run.sh e < input.txt > output.txt')
            with open(file3) as f3:
                l3 = f3.readlines()
                if str.strip(l3[2]) != "ok!!":
                    print("BUG!!!")
            count += 1
finally:
    print("FINISH")

# os.system('bash run.sh e < input.txt')
