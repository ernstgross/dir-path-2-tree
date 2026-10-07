#!/usr/bin/env python3
"""Builds the project page: several pages with one navigation - the README, the test results with the stack
overflow demonstration, the code coverage, the benchmarks - and the generated reports beside them: the API
documentation (cargo doc), the coverage report (cargo llvm-cov) and the Criterion report (cargo bench).

Reads target/pages-input/ (tests.txt, stack_overflow.txt, coverage.json, coverage/html), target/doc and
target/criterion; writes --out. Needs the Python package markdown (pip install markdown).
"""
import argparse
import html
import json
import os
import re
import shutil
import subprocess
from pathlib import Path

import markdown

ROOT = Path(__file__).resolve().parent.parent
INPUT = ROOT / "target" / "pages-input"
REPO = "https://github.com/ernstgross/dir-path-2-tree"

STYLE = """
:root { --bg:#f6f8fa; --panel:#fff; --text:#1f2328; --muted:#59636e; --border:#d1d9e0; --accent:#b7410e; --link:#0969da;
  --code:#eff2f5; --good:#1a7f37; --mid:#9a6700; --bad:#cf222e; }
@media (prefers-color-scheme: dark) { :root { --bg:#0d1117; --panel:#151b23; --text:#e6edf3; --muted:#9198a1;
  --border:#3d444d; --accent:#f0883e; --link:#4493f8; --code:#1f2630; --good:#3fb950; --mid:#d29922; --bad:#f85149; } }
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--text); font: 16px/1.6 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; }
header { background: var(--panel); border-bottom: 1px solid var(--border); }
.wrap { max-width: 980px; margin: 0 auto; padding: 0 16px; }
header .wrap { padding: 18px 16px 0; }
header h1 { margin: 0; font-size: 1.5rem; } header h1 a { color: inherit; text-decoration: none; } header h1 span { color: var(--accent); }
header p { margin: 4px 0 10px; color: var(--muted); }
nav { display: flex; flex-wrap: wrap; gap: 0 4px; font-size: .95rem; }
nav a { padding: 8px 10px; color: var(--text); text-decoration: none; border-bottom: 2px solid transparent; }
nav a:hover { color: var(--link); } nav a.on { border-bottom-color: var(--accent); font-weight: 600; }
nav a.ext::after { content: " ↗"; color: var(--muted); }
a { color: var(--link); }
main section { background: var(--panel); border: 1px solid var(--border); border-radius: 10px; padding: 8px 24px 18px; margin: 20px 0; }
h2 { border-bottom: 1px solid var(--border); padding-bottom: 4px; }
pre, code { font-family: ui-monospace, "Cascadia Mono", Consolas, monospace; font-size: .88em; }
pre { background: var(--code); border: 1px solid var(--border); border-radius: 8px; padding: 12px 14px; overflow-x: auto; }
code { background: var(--code); border-radius: 4px; padding: 0 4px; overflow-wrap: anywhere; } pre code { background: none; padding: 0; }
table { display: block; overflow-x: auto; border-collapse: collapse; font-size: .92rem; }
th, td { border: 1px solid var(--border); padding: 5px 9px; text-align: left; } th { background: var(--code); }
td.num { text-align: right; font-variant-numeric: tabular-nums; }
td code { white-space: nowrap; overflow-wrap: normal; }
.kpis { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 12px; margin: 16px 0; }
.kpi { border: 1px solid var(--border); border-radius: 10px; padding: 12px 14px; }
.kpi b { display: block; font-size: 1.7rem; } .kpi span { color: var(--muted); font-size: .9rem; }
.good { color: var(--good); } .mid { color: var(--mid); } .bad { color: var(--bad); }
.note { color: var(--muted); font-size: .92rem; }
footer { color: var(--muted); font-size: .85rem; padding: 10px 16px 30px; }
"""

PAGES = [("index.html", "Overview"), ("tests.html", "Tests"), ("coverage.html", "Coverage"), ("benchmarks.html", "Benchmarks")]


def read(name: str) -> str:
    p = INPUT / name
    return p.read_text(encoding="utf-8", errors="replace") if p.exists() else f"({name} was not produced in this run)"


def version() -> str:
    sha = os.environ.get("GITHUB_SHA")
    try:
        sha = sha or subprocess.run(["git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip()
        date = subprocess.run(["git", "log", "-1", "--format=%cd", "--date=format:%Y-%m-%d %H:%M", sha], cwd=ROOT,
                              capture_output=True, text=True, check=True).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return "unversioned"
    return f"{date} · {sha[:7]}"


def page(current: str, title: str, body: str, extra: "list[tuple[str, str]]") -> str:
    nav = "".join(f'<a href="{f}"{" class=on" if f == current else ""}>{t}</a>' for f, t in PAGES)
    nav += "".join(f'<a class="ext" href="{href}">{t}</a>' for href, t in extra)
    return f"""<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title} · dir-path-2-tree</title><style>{STYLE}</style></head><body>
<header><div class="wrap"><h1><a href="index.html">dir-path-2-tree <span>· Rust</span></a></h1>
<p>One task in several forms: recursive, iterative, with an explicit stack and with a depth limit - measured,
tested, and shown where recursion overflows the stack.</p><nav>{nav}</nav></div></header>
<main class="wrap">{body}</main>
<footer class="wrap">Built {html.escape(version())} from <a href="{REPO}">{REPO.split("//")[1]}</a></footer>
</body></html>
"""


def md(text: str) -> str:
    return markdown.markdown(text, extensions=["tables", "fenced_code"])


def readme_section(title: str) -> str:
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    m = re.search(rf"^## {re.escape(title)}\n(.*?)(?=^## |\Z)", text, re.M | re.S)
    return m.group(1) if m else ""


def grade(percent: float) -> str:
    return "good" if percent >= 80 else ("mid" if percent >= 60 else "bad")


def coverage_body(has_report: bool) -> str:
    p = INPUT / "coverage.json"
    if not p.exists():
        return "<section><h2>Code coverage</h2><p>No coverage was measured in this run.</p></section>"
    data = json.loads(p.read_text(encoding="utf-8"))["data"][0]
    kinds = [("lines", "lines"), ("functions", "functions"), ("regions", "regions")]
    kpis = "".join(f'<div class="kpi"><b class="{grade(data["totals"][k]["percent"])}">{data["totals"][k]["percent"]:.1f} %</b>'
                   f'<span>{label}: {data["totals"][k]["covered"]} of {data["totals"][k]["count"]}</span></div>' for k, label in kinds)
    rows = ""
    for f in sorted(data["files"], key=lambda f: f["filename"]):
        name = os.path.relpath(f["filename"], ROOT) if f["filename"].startswith(str(ROOT)) else f["filename"]
        cells = "".join(f'<td class="num {grade(f["summary"][k]["percent"])}">{f["summary"][k]["percent"]:.1f} %'
                        f' <span class="note">({f["summary"][k]["covered"]}/{f["summary"][k]["count"]})</span></td>' for k, _ in kinds)
        rows += f"<tr><td><code>{html.escape(name)}</code></td>{cells}</tr>"
    report = '<p><a href="coverage/html/index.html">The full report, line by line</a></p>' if has_report else ""
    return f"""<section><h2>Code coverage, this run</h2>
<p>Measured with <code>cargo llvm-cov</code> (source-based, LLVM) over all tests of the crate.</p>
<div class="kpis">{kpis}</div>
<table><tr><th>File</th><th>Lines</th><th>Functions</th><th>Regions</th></tr>{rows}</table>
{report}
<p class="note">What the numbers leave out: <code>src/main.rs</code> runs only with <code>cargo run</code>; the
<code>print_</code> functions, thin wrappers that write to the console, run in doc tests, which cargo-llvm-cov does
not count by default; and a child process of the stack overflow demonstration aborts before it can write its
profile - the routines it crashes in are covered by the other tests.</p></section>"""


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", type=Path, default=ROOT / "target" / "pages")
    args = parser.parse_args()
    out = args.out
    shutil.rmtree(out, ignore_errors=True)
    out.mkdir(parents=True)

    extra = []
    if (ROOT / "target" / "doc").is_dir():
        shutil.copytree(ROOT / "target" / "doc", out / "api")
        extra.append(("api/dir_path_2_tree/index.html", "API documentation"))
    has_cov = (INPUT / "coverage" / "html").is_dir()
    if has_cov:
        shutil.copytree(INPUT / "coverage", out / "coverage")
    has_bench = (ROOT / "target" / "criterion" / "report").is_dir()
    if has_bench:
        shutil.copytree(ROOT / "target" / "criterion", out / "bench")
    extra.append((REPO, "Source"))

    overview = f"<section>{md((ROOT / 'README.md').read_text(encoding='utf-8'))}</section>"
    tests = f"""<section id="stack-overflow"><h2>Stack overflow demonstration, this run</h2>
<p>A path of depth 5&nbsp;000 on a thread with 64&nbsp;KiB of stack; each crashing case runs in a child process
(<code>tests/stack_overflow.rs</code>).</p><pre>{html.escape(read("stack_overflow.txt"))}</pre></section>
<section id="tests"><h2>All tests, this run</h2><pre>{html.escape(read("tests.txt"))}</pre></section>"""
    bench_report = ('<p><a href="bench/report/index.html">The Criterion report of this run</a> - from a shared GitHub '
                    'runner, indicative only; the numbers above come from a dedicated machine.</p>') if has_bench else ""
    benchmarks = f"<section><h2>Benchmarks</h2>{md(readme_section('Benchmarks'))}{bench_report}</section>"

    for (name, title), body in zip(PAGES, [overview, tests, coverage_body(has_cov), benchmarks]):
        (out / name).write_text(page(name, title, body, extra), encoding="utf-8")
    print(f"{out}: {', '.join(n for n, _ in PAGES)}; coverage report {'yes' if has_cov else 'no'}, "
          f"benchmark report {'yes' if has_bench else 'no'}")


if __name__ == "__main__":
    main()
