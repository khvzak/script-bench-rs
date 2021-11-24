import json
import glob
import subprocess
from os.path import splitext, basename
import matplotlib.pyplot as plt


def run_benchmark(name):
    print(f"Running benchmark for {name}")
    proc = subprocess.Popen(
        f"cargo criterion --bench {name} --message-format json --features {name}",
        shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    )
    res = []
    for line in proc.stdout.readlines():
        x = json.loads(line)
        if "id" in x:
            res.append(x)
    return res


benches = dict()
ids = set()
for f in glob.glob("benches/*.rs"):
    name = splitext(basename(f))[0]
    benches[name] = dict()
    results = run_benchmark(name)
    for res in results:
        benches[name][res["id"]] = res
        ids.add(res["id"])

for id in ids:
    fig, ax = plt.subplots()

    ymax = 0
    for (name, bench) in sorted(benches.items(), key=lambda x: x[0]):
        val = round(bench[id]["typical"]["estimate"]) / 1000000
        rect = ax.bar(name, val, width=0.3)
        ax.bar_label(rect, padding=3)
        ymax = max(ymax, val)

    ax.set_title("lower is better")
    ax.set_ylabel("time (ms)", fontweight="bold")
    ax.set_ylim(0, ymax * 1.2)
    fig.suptitle(id, fontsize=18)
    fig.tight_layout()

    plt.savefig(f"{id}.png", dpi=300)
