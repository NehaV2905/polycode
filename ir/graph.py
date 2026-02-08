class IRGraph:
    def __init__(self):
        # adjacency list: function -> list of called functions
        self.edges = {}

    def add_function(self, fn):
        if fn not in self.edges:
            self.edges[fn] = []

    def add_edge(self, src, dst):
        self.add_function(src)
        self.add_function(dst)
        self.edges[src].append(dst)

    def all_functions(self):
        return set(self.edges.keys())

    def called_functions(self):
        called = set()
        for targets in self.edges.values():
            called.update(targets)
        return called
