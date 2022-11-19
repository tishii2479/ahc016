import ast
import multiprocessing
import subprocess

CASE = 100


def eval_graph(params):
    m, e, n = params

    cmd = f"./target/release/eval_graph {m} {e:.2f} {n}"
    print("running:", cmd)
    proc = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True
    )
    stdout = proc.stdout.decode("utf8")
    for line in stdout.splitlines():
        if len(line) >= 9 and line[:9].lower() == "result = ":
            m, eps, n, score = line[9:].split(",")

    print(f"{m}, {eps}, {n}, {score}")
    return m, eps, n, score


def main():
    subprocess.run("cargo build --release", shell=True)

    with open("data/greedy_n_map.txt", "r") as f:
        d = ast.literal_eval(
            f.readline().replace("(", '"').replace(")", '"').replace(" ", "")
        )

    params = []
    # TODO: 試すNをgreedyの結果から決める
    for m in range(10, 101, 2):
        for e in range(0, 41, 2):
            if e % 10 == 0:
                best_n = d[f"{m},{e / 100:.1f}"]
            else:
                best_n = d[f"{m},{e / 100:.2f}"]
            for n in range(max(4, best_n - 10), best_n + 1):
                params.append((m, e / 100, n))

    n_map = {}

    with multiprocessing.Pool(max(1, multiprocessing.cpu_count() - 2)) as pool:
        for m, eps, n, score in pool.imap_unordered(eval_graph, params):
            n_map[(m, eps, n)] = score

    print(n_map)
    with open("data/n_map.txt", "w") as f:
        f.write(str(n_map))


if __name__ == "__main__":
    main()
