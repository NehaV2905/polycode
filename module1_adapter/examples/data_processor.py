"""
Data Processing Pipeline
Demonstrates data transformations, filtering, and analytics
"""

import json
from typing import List, Dict, Any, Callable
from datetime import datetime


class DataValidator:
    """Validate incoming data"""

    def __init__(self):
        self.validation_rules = []

    def add_rule(self, rule: Callable):
        """Add validation rule"""
        self.validation_rules.append(rule)

    def validate(self, data: Dict) -> bool:
        """Run all validation rules"""
        for rule in self.validation_rules:
            if not rule(data):
                return False
        return True


class DataTransformer:
    """Transform data between formats"""

    def normalize_keys(self, data: Dict) -> Dict:
        """Convert all keys to lowercase"""
        return {k.lower(): v for k, v in data.items()}

    def filter_empty(self, data: Dict) -> Dict:
        """Remove empty values"""
        return {k: v for k, v in data.items() if v}

    def add_timestamp(self, data: Dict) -> Dict:
        """Add timestamp to data"""
        data['processed_at'] = datetime.now().isoformat()
        return data

    def transform_pipeline(self, data: Dict) -> Dict:
        """Run full transformation pipeline"""
        data = self.normalize_keys(data)
        data = self.filter_empty(data)
        data = self.add_timestamp(data)
        return data


class DataAggregator:
    """Aggregate data for analytics"""

    def __init__(self):
        self.data_store: List[Dict] = []

    def add_record(self, record: Dict):
        """Add record to store"""
        self.data_store.append(record)

    def count_records(self) -> int:
        """Count total records"""
        return len(self.data_store)

    def filter_by_field(self, field: str, value: Any) -> List[Dict]:
        """Filter records by field value"""
        results = []
        for record in self.data_store:
            if record.get(field) == value:
                results.append(record)
        return results

    def group_by_field(self, field: str) -> Dict[Any, int]:
        """Group and count by field"""
        groups = {}
        for record in self.data_store:
            key = record.get(field)
            if key:
                groups[key] = groups.get(key, 0) + 1
        return groups

    def calculate_average(self, field: str) -> float:
        """Calculate average of numeric field"""
        values = []
        for record in self.data_store:
            value = record.get(field)
            if isinstance(value, (int, float)):
                values.append(value)

        if values:
            return sum(values) / len(values)
        return 0.0


def load_data_from_json(file_path: str) -> List[Dict]:
    """Load data from JSON file"""
    try:
        with open(file_path, 'r') as f:
            data = json.load(f)
            return data
    except FileNotFoundError:
        print(f"File not found: {file_path}")
        return []
    except json.JSONDecodeError:
        print(f"Invalid JSON in file: {file_path}")
        return []


def save_data_to_json(data: List[Dict], file_path: str):
    """Save data to JSON file"""
    try:
        with open(file_path, 'w') as f:
            json.dump(data, f, indent=2)
        print(f"Data saved to {file_path}")
    except Exception as e:
        print(f"Error saving data: {e}")


def process_batch(records: List[Dict]) -> List[Dict]:
    """Process a batch of records"""
    transformer = DataTransformer()
    processed = []

    for record in records:
        try:
            transformed = transformer.transform_pipeline(record)
            processed.append(transformed)
        except Exception as e:
            print(f"Error processing record: {e}")

    return processed


def generate_report(aggregator: DataAggregator):
    """Generate analytics report"""
    print("\n=== Data Analytics Report ===")
    print(f"Total Records: {aggregator.count_records()}")

    # Example: Group by status if it exists
    groups = aggregator.group_by_field('status')
    if groups:
        print("\nRecords by Status:")
        for status, count in groups.items():
            print(f"  {status}: {count}")


def run_pipeline(input_file: str, output_file: str):
    """Run the complete data processing pipeline"""
    print("Starting data processing pipeline...")

    # Load data
    raw_data = load_data_from_json(input_file)
    print(f"Loaded {len(raw_data)} records")

    # Process data
    processed_data = process_batch(raw_data)
    print(f"Processed {len(processed_data)} records")

    # Aggregate data
    aggregator = DataAggregator()
    for record in processed_data:
        aggregator.add_record(record)

    # Generate report
    generate_report(aggregator)

    # Save results
    save_data_to_json(processed_data, output_file)


def create_sample_data() -> List[Dict]:
    """Create sample data for testing"""
    return [
        {"ID": 1, "Name": "Alice", "Age": 30, "Status": "active"},
        {"ID": 2, "Name": "Bob", "Age": 25, "Status": "active"},
        {"ID": 3, "Name": "Charlie", "Age": 35, "Status": "inactive"},
        {"ID": 4, "Name": "", "Age": 28, "Status": "active"},
    ]


def main():
    """Main entry point"""
    print("=== Data Processing Demo ===")

    # Create sample data
    sample_data = create_sample_data()

    # Process the data
    processed = process_batch(sample_data)

    # Aggregate results
    aggregator = DataAggregator()
    for record in processed:
        aggregator.add_record(record)

    # Generate report
    generate_report(aggregator)

    # Calculate average age
    avg_age = aggregator.calculate_average('age')
    print(f"\nAverage Age: {avg_age:.1f}")


if __name__ == "__main__":
    main()
