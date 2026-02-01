"""
Enhanced test file for extended IR events.

Tests: async/await, exceptions, lambdas, member access
"""

import asyncio
from typing import List


class DataProcessor:
    """Processes data with async operations and error handling."""
    
    def __init__(self, config: dict):
        self.config = config
        self.data = []
    
    async def fetch_data(self, url: str):
        """Async function to fetch data."""
        try:
            # Simulate async operation
            result = await self._async_request(url)
            self.data.append(result)
            return result
        except ValueError as e:
            print(f"Value error: {e}")
            raise
        except (KeyError, TypeError):
            print("Multiple exceptions caught")
            return None
        except:
            print("Catch all")
            raise RuntimeError("Unknown error")
    
    async def _async_request(self, url: str):
        """Internal async helper."""
        await asyncio.sleep(0.1)
        return {"url": url, "data": "sample"}
    
    def process_with_lambda(self, items: List[int]):
        """Demonstrates lambda usage."""
        # Lambda for filtering
        filtered = list(filter(lambda x: x > 0, items))
        
        # Lambda for mapping
        doubled = list(map(lambda x: x * 2, filtered))
        
        # Lambda in sorting
        sorted_items = sorted(items, key=lambda x: abs(x))
        
        return doubled
    
    def member_access_demo(self):
        """Demonstrates member access patterns."""
        # Property access
        length = self.config.get("length")
        
        # Method call
        keys = self.config.keys()
        
        # Chained member access
        first_item = self.data[0].get("url") if self.data else None
        
        return length


def handle_errors():
    """Error handling demonstration."""
    try:
        value = int("not a number")
    except ValueError:
        # Re-raise
        raise ValueError("Invalid input")
    finally:
        print("Cleanup")


async def main():
    """Async main function."""
    processor = DataProcessor({"length": 100})
    
    # Await async function
    result = await processor.fetch_data("https://example.com")
    
    # Process with lambdas
    nums = processor.process_with_lambda([1, -2, 3, -4, 5])
    
    # Member access
    config_value = processor.member_access_demo()
    
    print(f"Result: {result}")


if __name__ == "__main__":
    asyncio.run(main())
