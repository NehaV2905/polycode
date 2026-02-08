from .registry import ANALYSES

class AnalysisEngine:
    def __init__(self, graph):
        self.graph = graph

    def run(self, analysis_name, **kwargs):
        if analysis_name not in ANALYSES:
            raise Exception("Unknown analysis")

        return ANALYSES[analysis_name](self.graph, **kwargs)
