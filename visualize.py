import matplotlib.pyplot as plt

scores = []

with open("score.log", "r") as f:
    scores = list(map(lambda x: int(x), f.readline().split(",")))

plt.plot(scores)
plt.show()
