import sys
import os

# Add the directory containing this file to sys.path to allow imports from module1_adapter
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from module1_adapter.core.adapter import main

if __name__ == "__main__":
    main()