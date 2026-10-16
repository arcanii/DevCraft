"""Command scripts that build workbooks heavy in one part of a sheet each, for measuring what a cell
edit copies into the undo history besides the cells (our own data, nothing from Excel).

Usage: python3 -I make_parts.py <out dir>. Writes, for gridcraft-cli run --script:
  part-<name>.jsonl  builds the workbook (run it with --out part-<name>.xlsx)
  edits-50.jsonl     50 cell edits in column Z, the same sheet
Parts: notes (10,000), links (10,000), cf (2,000 rules), validations (2,000), names (5,000),
merges (5,000), rows (50,000 custom heights), styles (5,000 distinct fills), spill (100,000 cells).
"""
import json, os, sys

out = sys.argv[1]
os.makedirs(out, exist_ok=True)

def cmd(command, params):
    return json.dumps({"command": command, "params": params})

def write(name, lines):
    with open(os.path.join(out, name), "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines) + "\n")

parts = {
    "notes": [cmd("review.newNote", {"cell": f"A{i + 1}", "text": f"Checked by the team on day {i}"}) for i in range(10_000)],
    "links": [cmd("insert.link", {"cell": f"B{i + 1}", "target": f"https://example.com/item/{i}", "text": f"item {i}"}) for i in range(10_000)],
    "cf": [cmd("home.conditionalFormat", {"range": f"C{i + 1}", "rule": {"type": "cellIs", "operator": "greater", "value": str(i), "preset": "lightRedFill"}}) for i in range(2_000)],
    "validations": [cmd("data.validation", {"range": f"D{i + 1}", "type": "whole", "operator": "between", "formula1": "0", "formula2": str(i + 1)}) for i in range(2_000)],
    "names": [cmd("formulas.defineName", {"name": f"Item_{i}", "refersTo": f"=Sheet1!$E${i + 1}"}) for i in range(5_000)],
    "merges": [cmd("home.mergeCells", {"range": f"F{2 * i + 1}:G{2 * i + 1}"}) for i in range(5_000)],
    "rows": [cmd("home.rowHeight", {"rows": "1:50000", "height": 18})],
    "styles": [cmd("home.fillColor", {"range": f"H{i + 1}", "color": f"#{(i * 2654435761) & 0xFFFFFF:06X}"}) for i in range(5_000)],
    "spill": [cmd("cell.set", {"cell": "J1", "input": "=SEQUENCE(100000)"})],
}
for name, lines in parts.items():
    write(f"part-{name}.jsonl", lines)
write("edits-50.jsonl", [cmd("cell.set", {"cell": f"Z{i + 1}", "input": str(i)}) for i in range(50)])
