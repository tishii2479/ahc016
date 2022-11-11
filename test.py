a = list(map(int, input().split()))
b = list(map(int, input().split()))

assert len(a) == len(b)

sum = 0
for i in range(len(a)):
    sum += (a[i] - b[i]) ** 2
print(sum)
