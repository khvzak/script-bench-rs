import json
import glob
import subprocess
from os.path import splitext, basename
import matplotlib.pyplot as plt


def run_benchmark(name):
    print(f"Running benchmark for {name}")
    proc = subprocess.run(
        f"cargo criterion --bench {name} --message-format json --features {name}",
        shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
        check=True,
    )
    res = []
    for line in proc.stdout.splitlines():
        x = json.loads(line)
        if "id" in x:
            res.append(x)
    return res


# Compile webassembly modules
subprocess.run(
    "rustc --target wasm32-unknown-unknown -Cpanic=abort -O --crate-name sort_userdata scripts/sort_userdata.wasm.rs -o scripts/sort_userdata.wasm",
    shell=True,
    check=True,
)

# Compile webassembly components
subprocess.run(
    "cargo build --target wasm32-wasip2 --release",
    cwd="bench",
    shell=True,
    check=True,
)

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
    for (name, bench) in sorted(benches.items(), key=lambda x: x[1][id]["typical"]["estimate"]):
        val = round(bench[id]["typical"]["estimate"] / 1000000, 2)
        rect = ax.bar(name, val, width=0.3)
        ax.bar_label(rect, padding=3)
        ymax = max(ymax, val)

    ax.set_title("lower is better")
    ax.set_ylabel("time (ms)", fontweight="bold")
    ax.set_ylim(0, ymax * 1.2)
    fig.autofmt_xdate()
    fig.suptitle(id, fontsize=18)
    fig.tight_layout()

    plt.savefig(f"{id}.png", dpi=300)
