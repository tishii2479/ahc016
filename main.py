M, eps = input().split()

M = int(M)

eps = float(eps)

N = 20

print(N)

for k in range(M):
    print("1" * k + "0" * (190 - k))

for q in range(100):
    H = input()
    t = min(H.count("1"), M - 1)
    print(t)
