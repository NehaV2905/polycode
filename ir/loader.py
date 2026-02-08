import json
from .graph import IRGraph

def load_graph(json_file):
    g = IRGraph()
    data = json.load(open(json_file))

    # expected format:
    # {"edges": [["login","hash_password"], ["login","check_user"]]}
    for src, dst in data["edges"]:
        g.add_edge(src, dst)

    return g
