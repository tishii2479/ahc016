import multiprocessing
import subprocess

CASE = 100
TL = 6.0


def execute_case(seed):
    input_file_path = f"tools/in/{seed:04}.txt"
    output_file_path = f"tools/out/{seed:04}.txt"

    tester_path = "./tools/target/release/tester"
    solver_path = "./target/release/ahc016"

    with open(input_file_path, "r") as f:
        M, eps = f.readline().split()

    cmd = f"{tester_path} {solver_path} < {input_file_path} > {output_file_path}"
    proc = subprocess.run(
        cmd,
        stderr=subprocess.PIPE,
        timeout=TL,
    )
    stderr = proc.stderr.decode("utf8")
    score = -1
    for line in stderr.splitlines():
        if len(line) >= 6 and line[:6].lower() == "score:":
            score = int(line.split()[-1])
    assert score != -1

    return seed, score, int(M), float(eps)


def main():
    scores = []
    count = 0
    total = 0

    subprocess.run("cargo build --release", shell=True)
    with multiprocessing.Pool(max(1, multiprocessing.cpu_count() - 2)) as pool:
        for seed, score, M, eps in pool.imap_unordered(execute_case, range(CASE)):
            count += 1
            try:
                scores.append((int(score), f"{seed:04}"))
                total += scores[-1][0]
            except ValueError:
                print(seed, "ValueError", flush=True)
                print(score, flush=True)
                exit()
            except IndexError:
                print(seed, "IndexError", flush=True)
                print(f"error: {score}", flush=True)
                exit()

            print(
                f"case {seed:3}: (score: {scores[-1][0]:7}, current ave: {total / count:10.2f})",
                flush=True,
            )

    print()
    scores.sort()
    ave = total / CASE
    print(f"total: {total}")
    for i in range(10):
        print(f"{scores[-(i+1)]}")
    for i in range(10):
        print(f"{scores[i]}")
    print(f"ave: {ave}")


if __name__ == "__main__":
    main()
