import copy
import math

s = "A"

def saiki(s, max_depth, current_depth=0):
    print("", current_depth, s)
    # print("current_depth: ", current_depth, s)
    if current_depth <= max_depth:
        s_copy = copy.deepcopy(s)
        for i, si in enumerate(s):
            # print("i:",i, "si:",si, "s_copy:",s_copy, "s_copy[:2*i]: ", s_copy[:2*i], "s_copy[2*i+1:]: ", s_copy[2*i+1:])
            if si == "A":
                s_copy = s_copy[:2*i] + "BC" + s_copy[2*i+1:]
            elif si == "B":
                s_copy = s_copy[:2*i] + "CA" + s_copy[2*i+1:]
            elif si == "C":
                s_copy = s_copy[:2*i] + "AB" + s_copy[2*i+1:]
        s = saiki(s_copy, max_depth, current_depth+1)
        return s
    return s

s = saiki(s, 7)
print(s)

print(18 * math.log2(10)) # 59.79470570797252

