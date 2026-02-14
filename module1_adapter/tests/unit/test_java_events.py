"""
Quick test to show detailed event breakdown for Java
"""
import sys
sys.path.insert(0, 'src')

from parsers.tree_sitter_adapter import TreeSitterParser

# Read Java file
with open('examples/multi_lang/ComplexJava.java', 'r') as f:
    source = f.read()

# Parse
parser = TreeSitterParser('java')
facts = parser.parse(source, 'ComplexJava.java')

# Count by type
event_counts = {}
for fact in facts:
    event_type = fact.fact_type
    event_counts[event_type] = event_counts.get(event_type, 0) + 1

print(f"\n📊 Java Event Breakdown ({len(facts)} total events):\n")
print("=" * 50)
for event_type in sorted(event_counts.keys()):
    count = event_counts[event_type]
    bar = "█" * (count // 2)
    print(f"{event_type:25s} | {count:3d} {bar}")
print("=" * 50)
