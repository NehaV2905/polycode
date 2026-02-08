def run_unused_function_analysis(graph):
    all_funcs = graph.all_functions()
    called = graph.called_functions()
    
    unused = list(all_funcs - called)
    
    return {
        "unused_functions": unused
    }
