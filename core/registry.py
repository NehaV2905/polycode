from analyses.unused import run_unused_function_analysis
from analyses.dependency import run_dependency_analysis
from analyses.impact import run_change_impact

ANALYSES = {
    "unused": run_unused_function_analysis,
    "dependency": run_dependency_analysis,
    "impact": run_change_impact
}
