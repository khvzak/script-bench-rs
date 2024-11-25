import json
import glob
import subprocess
import psutil
import time
from os.path import splitext, basename
import matplotlib.pyplot as plt
from typing import Dict, List, Tuple


def measure_process_memory(name_prefix: str) -> float:
    """Measure maximum memory usage of processes matching the name prefix."""
    max_memory = 0
    for proc in psutil.process_iter(["name", "memory_info"]):
        try:
            if proc.name().startswith(name_prefix):
                memory = proc.memory_info().rss / (1024 * 1024)  # Convert to MB
                max_memory = max(max_memory, memory)
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            continue
    return max_memory


def run_benchmark(name: str) -> Tuple[List[dict], float]:
    """Run benchmark and measure both performance and memory usage."""
    print(f"Running benchmark for {name}")
    proc = subprocess.Popen(
        f"cargo criterion --bench {name} --message-format json --features {name}",
        shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
    )

    # Monitor memory usage while benchmark is running
    max_memory = 0
    while proc.poll() is None:
        current_memory = measure_process_memory(name)
        max_memory = max(max_memory, current_memory)
        time.sleep(0.01)  # Poll every 10ms

    if proc.returncode != 0:
        raise subprocess.CalledProcessError(proc.returncode, proc.args)

    # Process benchmark results
    res = []
    for line in proc.stdout.readlines():
        x = json.loads(line)
        if "id" in x:
            res.append(x)

    return res, max_memory


def plot_performance_graph(benches: Dict[str, Dict[str, dict]], id: str):
    """Plot performance benchmark results."""
    fig, ax = plt.subplots()

    ymax = 0
    for name, bench in sorted(
        benches.items(), key=lambda x: x[1][id]["typical"]["estimate"]
    ):
        val = round(bench[id]["typical"]["estimate"] / 1000000, 2)
        rect = ax.bar(name, val, width=0.3)
        ax.bar_label(rect, padding=3)
        ymax = max(ymax, val)

    ax.set_title("lower is better")
    ax.set_ylabel("Time (ms)", fontweight="bold")
    ax.set_ylim(0, ymax * 1.2)
    fig.autofmt_xdate()
    fig.suptitle(f"{id} - Performance", fontsize=18)
    fig.tight_layout()

    plt.savefig(f"{id} .perf.png", dpi=300)
    plt.close()


def plot_memory_graph(memory_data: Dict[str, float], id: str):
    """Plot memory usage results."""
    fig, ax = plt.subplots()

    names = []
    values = []
    for name, memory in sorted(memory_data.items(), key=lambda x: x[1]):
        names.append(name)
        values.append(memory)

    rects = ax.bar(names, values, width=0.3)
    ax.bar_label(rects, [f"{v:.1f} MB" for v in values], padding=3)

    ax.set_title("lower is better")
    ax.set_ylabel("Memory Usage (MB)", fontweight="bold")
    ax.set_ylim(0, max(values) * 1.2)
    fig.autofmt_xdate()
    fig.suptitle(f"{id} - Memory Usage", fontsize=18)
    fig.tight_layout()

    plt.savefig(f"{id}.mem.png", dpi=300)
    plt.close()


# Compile webassembly modules
subprocess.run(
    "rustc --target wasm32-unknown-unknown -Cpanic=abort -O --crate-name sort_userdata scripts/sort_userdata.wasm.rs -o scripts/sort_userdata.wasm",
    shell=True,
    check=True,
)

# Run benchmarks and collect data
benches = dict()
memory_usage = dict()
ids = set()

for f in glob.glob("benches/*.rs"):
    name = splitext(basename(f))[0]
    benches[name] = dict()

    # Run benchmark and collect both performance and memory data
    results, max_mem = run_benchmark(name)
    memory_usage[name] = max_mem

    for res in results:
        benches[name][res["id"]] = res
        ids.add(res["id"])

# Generate plots
for id in ids:
    plot_performance_graph(benches, id)
    plot_memory_graph(memory_usage, id)
