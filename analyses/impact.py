def run_change_impact(graph, function_name):
    impacted = []

    for src, targets in graph.edges.items():
        if function_name in targets:
            impacted.append(src)

    return {
        "changed_function": function_name,
        "impacted_functions": impacted
    }
