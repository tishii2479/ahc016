import multiprocessing
import subprocess

CASE = 100


def eval_graph(params):
    m, e = params

    cmd = f"./target/release/eval_graph {m} {e:.2f}"
    print("running:", cmd)
    proc = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True
    )
    stdout = proc.stdout.decode("utf8")
    best_n = -1
    for line in stdout.splitlines():
        if len(line) >= 8 and line[:8].lower() == "result =":
            best_n = int(line.split(" ")[-1])
    assert best_n != -1

    print(f"{m}, {e}, {best_n}")
    return m, e, best_n


def main():
    subprocess.run("cargo build --release", shell=True)
    params = []
    for m in range(10, 31, 10):
        for e in range(0, 41, 5):
            params.append((m, e / 100))

    n_map = {}

    with multiprocessing.Pool(max(1, multiprocessing.cpu_count() - 2)) as pool:
        for m, e, best_n in pool.imap_unordered(eval_graph, params):
            n_map[(m, e)] = best_n

    print(n_map)
    with open("data/n_map.txt", "w") as f:
        f.write(str(n_map))


if __name__ == "__main__":
    main()
