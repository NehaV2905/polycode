"""
Base Parser Interface

Defines the contract for all language parsers.
"""

from abc import ABC, abstractmethod
from typing import List, Dict, Any
from datetime import datetime

class IRFact:
    """Represents a single observed fact about the code."""
    def __init__(self, fact_type: str, data: Dict[str, Any], line_number: int):
        self.fact_type = fact_type
        self.data = data
        self.line_number = line_number
        self.timestamp = datetime.now()

    def __repr__(self):
        return f"IRFact({self.fact_type}, line={self.line_number}, data={self.data})"


class BaseParser(ABC):
    """
    Abstract base class for language parsers.
    """
    
    @abstractmethod
    def parse(self, source_code: str, file_path: str) -> List[IRFact]:
        """
        Parse source code and extract IR facts.
        
        Args:
            source_code: The source code as a string
            file_path: Path to the source file (for metadata)
        
        Returns:
            List of IRFact objects
        """
        pass
