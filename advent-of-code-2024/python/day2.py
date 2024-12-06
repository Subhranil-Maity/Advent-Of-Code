day2 = ""
with open("../inputs/day2.txt", 'r') as o:
    day2 = o.read()
sample = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""


def is_safe(b):
    a = []
    for i in b:
        if i != '':
            a.append(int(i))
    gradient: int = a[0] - a[1]
    for a, b in zip(a, a[1:]):
        g: int = a-b
        if not (((gradient > 0 and g > 0) or (gradient < 0 and g < 0))):
            return False
        if abs(g) > 3:
            return False
    return True


lines_s = sample.split('\n')
lines = day2.split('\n')

m: int = 0
for line in lines:
    line = line.split(' ')
    if len(line) == 1:
        continue
    if is_safe(line):
        m = m+1
print("Answer of day2 part1: ", m)


def is_safe_2(b):
    for i in range(0, len(b)):
        n = b[0:i]
        n.extend(b[i+1:])
        if is_safe(n):
            return True
    return False


m = 0
for line in lines:
    line = line.split(' ')
    if len(line) == 1:
        continue
    if is_safe_2(line):
        m = m+1
print("Answer of day2 part1: ", m)
