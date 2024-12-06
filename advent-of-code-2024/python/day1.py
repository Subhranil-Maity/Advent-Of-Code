day1 = ""
with open("../inputs/day1.txt", 'r') as o:
    day1 = o.read()

sample = """3   4
4   3
2   5
1   3
3   9
3   3"""
s1 = day1.split("\n")
a = []
b = []
for s in s1:
    n = s.split("   ")
    if len(n) != 0:
        a.append(n[0])
        b.append(n[1])

a.sort()
b.sort()
n = 0
for i in range(0, len(a)):
    n += abs(int(a[i]) - int(b[i]))

print("Part One Answer is: ", n)
m = 0

for i in a:
    # if c.get(i) is None:
    #     continue
    m = m + int(i)*b.count(i)
print("Answer of da1 part2", m)
