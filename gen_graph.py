import multiprocessing
import subprocess

CASE = 100


def gen_graph(params):
    m, e = params

    cmd = f"./target/release/gen_graph {m} {e:.2f}"
    print(cmd)
    proc = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True
    )
    return m, e


def main():
    subprocess.run("cargo build --release", shell=True)
    params = []
    for m in range(10, 101, 10):
        for e in range(0, 41, 5):
            params.append((m, e / 100))

    with multiprocessing.Pool(max(1, multiprocessing.cpu_count() - 2)) as pool:
        for m, e in pool.imap_unordered(gen_graph, params):
            pass


if __name__ == "__main__":
    main()
