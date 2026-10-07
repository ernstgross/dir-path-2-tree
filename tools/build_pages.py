#!/usr/bin/env python3
"""Builds the project page: the README as the start page, the test results and the stack overflow demonstration,
with links to the API documentation (cargo doc) and the criterion report (cargo bench).

Reads target/pages-input/tests.txt and stack_overflow.txt, target/doc and target/criterion; writes --out.
Needs the Python package markdown (pip install markdown).
"""
import argparse
import html
import os
import shutil
import subprocess
from pathlib import Path

import markdown

ROOT = Path(__file__).resolve().parent.parent
INPUT = ROOT / "target" / "pages-input"

STYLE = """
:root { --bg:#f6f8fa; --panel:#fff; --text:#1f2328; --muted:#59636e; --border:#d1d9e0; --accent:#b7410e; --link:#0969da;
  --code:#eff2f5; }
@media (prefers-color-scheme: dark) { :root { --bg:#0d1117; --panel:#151b23; --text:#e6edf3; --muted:#9198a1;
  --border:#3d444d; --accent:#f0883e; --link:#4493f8; --code:#1f2630; } }
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--text); font: 16px/1.6 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; }
header { background: var(--panel); border-bottom: 1px solid var(--border); }
.wrap { max-width: 980px; margin: 0 auto; padding: 0 16px; }
header .wrap { padding: 18px 16px 12px; }
header h1 { margin: 0; font-size: 1.5rem; } header h1 span { color: var(--accent); }
header p { margin: 4px 0 10px; color: var(--muted); }
nav { display: flex; flex-wrap: wrap; gap: 6px 18px; font-size: .95rem; }
a { color: var(--link); }
main section { background: var(--panel); border: 1px solid var(--border); border-radius: 10px; padding: 8px 24px 18px; margin: 20px 0; }
h2 { border-bottom: 1px solid var(--border); padding-bottom: 4px; }
pre, code { font-family: ui-monospace, "Cascadia Mono", Consolas, monospace; font-size: .88em; }
pre { background: var(--code); border: 1px solid var(--border); border-radius: 8px; padding: 12px 14px; overflow-x: auto; }
code { background: var(--code); border-radius: 4px; padding: 0 4px; overflow-wrap: anywhere; } pre code { background: none; padding: 0; }
table { display: block; overflow-x: auto; border-collapse: collapse; font-size: .92rem; } th, td { border: 1px solid var(--border); padding: 5px 9px; text-align: left; }
th { background: var(--code); }
footer { color: var(--muted); font-size: .85rem; padding: 10px 16px 30px; }
"""


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


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", type=Path, default=ROOT / "target" / "pages")
    args = parser.parse_args()
    out = args.out
    shutil.rmtree(out, ignore_errors=True)
    out.mkdir(parents=True)

    links = ['<a href="#readme">Overview</a>', '<a href="#stack-overflow">Stack overflow demonstration</a>',
             '<a href="#tests">Test results</a>']
    if (ROOT / "target" / "doc").is_dir():
        shutil.copytree(ROOT / "target" / "doc", out / "api")
        links.append('<a href="api/dir_path_2_tree/index.html">API documentation</a>')
    if (ROOT / "target" / "criterion" / "report").is_dir():
        shutil.copytree(ROOT / "target" / "criterion", out / "bench")
        links.append('<a href="bench/report/index.html">Benchmark report (this run)</a>')

    readme = markdown.markdown((ROOT / "README.md").read_text(encoding="utf-8"), extensions=["tables", "fenced_code"])
    page = f"""<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>dir-path-2-tree</title><style>{STYLE}</style></head><body>
<header><div class="wrap"><h1>dir-path-2-tree <span>· Rust</span></h1>
<p>One task in several forms: recursive, iterative, with an explicit stack and with a depth limit — measured,
tested, and shown where recursion overflows the stack.</p><nav>{' '.join(links)}</nav></div></header>
<main class="wrap">
<section id="readme">{readme}</section>
<section id="stack-overflow"><h2>Stack overflow demonstration, this run</h2>
<p>A path of depth 5&nbsp;000 on a thread with 64&nbsp;KiB of stack; each crashing case runs in a child process
(<code>tests/stack_overflow.rs</code>).</p><pre>{html.escape(read("stack_overflow.txt"))}</pre></section>
<section id="tests"><h2>Test results, this run</h2><pre>{html.escape(read("tests.txt"))}</pre></section>
<p>The benchmark report of this run comes from a shared GitHub runner and is indicative only; the numbers in the
README come from a dedicated machine.</p>
</main>
<footer class="wrap">Built {html.escape(version())} from
<a href="https://github.com/ernstgross/dir-path-2-tree">github.com/ernstgross/dir-path-2-tree</a></footer>
</body></html>
"""
    (out / "index.html").write_text(page, encoding="utf-8")
    print(f"{out / 'index.html'} written; {len(links)} parts")


if __name__ == "__main__":
    main()
