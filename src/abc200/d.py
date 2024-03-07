
def sum(a, list):
    ans = 0
    for i in list:
        ans += a[i]
    
    return ans % 200

a = [180, 186, 189, 191, 218]
s0 = sum(a, [1,2,3])
print(s0)

s1 = sum(a, [0,1])
print(s1)

