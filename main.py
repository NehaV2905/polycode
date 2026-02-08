from ir.loader import load_graph
from core.engine import AnalysisEngine
from output.serializer import to_json

# load IR graph
graph = load_graph("graph.json")

engine = AnalysisEngine(graph)

# Run unused function analysis
result = engine.run("unused")
print(to_json(result))

# Run dependency analysis
print(to_json(engine.run("dependency")))

# Run impact analysis
print(to_json(engine.run("impact", function_name="hash_password")))
