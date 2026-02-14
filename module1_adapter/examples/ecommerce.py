"""
E-commerce Order Processing System
Demonstrates async functions, error handling, and business logic
"""

import asyncio
from typing import List, Optional, Dict
from enum import Enum


class OrderStatus(Enum):
    """Order status enumeration"""
    PENDING = 1
    PROCESSING = 2
    SHIPPED = 3
    DELIVERED = 4
    CANCELLED = 5


class Product:
    """Product model"""

    def __init__(self, product_id: str, name: str, price: float, stock: int):
        self.product_id = product_id
        self.name = name
        self.price = price
        self.stock = stock

    def is_available(self, quantity: int) -> bool:
        """Check if product is available in requested quantity"""
        return self.stock >= quantity

    def reduce_stock(self, quantity: int):
        """Reduce stock after purchase"""
        if self.is_available(quantity):
            self.stock -= quantity
        else:
            raise ValueError(f"Insufficient stock for {self.name}")


class ShoppingCart:
    """Shopping cart management"""

    def __init__(self, customer_id: str):
        self.customer_id = customer_id
        self.items: Dict[str, int] = {}

    def add_item(self, product_id: str, quantity: int):
        """Add item to cart"""
        if product_id in self.items:
            self.items[product_id] += quantity
        else:
            self.items[product_id] = quantity

    def remove_item(self, product_id: str):
        """Remove item from cart"""
        if product_id in self.items:
            del self.items[product_id]

    def get_total_items(self) -> int:
        """Get total number of items"""
        return sum(self.items.values())

    def clear(self):
        """Clear all items from cart"""
        self.items.clear()


class Order:
    """Order processing"""

    def __init__(self, order_id: str, customer_id: str):
        self.order_id = order_id
        self.customer_id = customer_id
        self.status = OrderStatus.PENDING
        self.total_amount = 0.0

    def calculate_total(self, products: List[Product], quantities: List[int]) -> float:
        """Calculate order total"""
        total = 0.0
        for product, quantity in zip(products, quantities):
            total += product.price * quantity

        self.total_amount = total
        return total

    def process_payment(self, amount: float) -> bool:
        """Process payment for order"""
        try:
            if amount >= self.total_amount:
                self.status = OrderStatus.PROCESSING
                return True
            else:
                raise ValueError("Insufficient payment amount")
        except ValueError as e:
            print(f"Payment failed: {e}")
            return False

    def ship_order(self):
        """Mark order as shipped"""
        if self.status == OrderStatus.PROCESSING:
            self.status = OrderStatus.SHIPPED
            print(f"Order {self.order_id} shipped")

    def deliver_order(self):
        """Mark order as delivered"""
        if self.status == OrderStatus.SHIPPED:
            self.status = OrderStatus.DELIVERED
            print(f"Order {self.order_id} delivered")

    def cancel_order(self):
        """Cancel the order"""
        if self.status in [OrderStatus.PENDING, OrderStatus.PROCESSING]:
            self.status = OrderStatus.CANCELLED
            print(f"Order {self.order_id} cancelled")


async def fetch_product_details(product_id: str) -> Optional[Product]:
    """Async function to fetch product details from database"""
    await asyncio.sleep(0.1)  # Simulate database query

    # Mock product data
    mock_products = {
        "P001": Product("P001", "Laptop", 999.99, 50),
        "P002": Product("P002", "Mouse", 29.99, 200),
        "P003": Product("P003", "Keyboard", 79.99, 150),
    }

    return mock_products.get(product_id)


async def process_order_async(order: Order, cart: ShoppingCart) -> bool:
    """Asynchronously process an order"""
    print(f"Processing order {order.order_id}")

    try:
        # Fetch all products
        products = []
        quantities = []

        for product_id, quantity in cart.items.items():
            product = await fetch_product_details(product_id)
            if product:
                products.append(product)
                quantities.append(quantity)

        # Calculate total
        total = order.calculate_total(products, quantities)
        print(f"Order total: ${total:.2f}")

        # Process payment
        payment_success = order.process_payment(total)

        if payment_success:
            # Reduce stock
            for product, quantity in zip(products, quantities):
                product.reduce_stock(quantity)

            # Ship order
            order.ship_order()
            return True
        else:
            order.cancel_order()
            return False

    except Exception as e:
        print(f"Error processing order: {e}")
        order.cancel_order()
        return False


def create_sample_order() -> tuple:
    """Create a sample order for testing"""
    cart = ShoppingCart("CUST001")
    cart.add_item("P001", 1)
    cart.add_item("P002", 2)

    order = Order("ORD001", "CUST001")
    return order, cart


async def main():
    """Main entry point"""
    print("=== E-commerce Order Processing Demo ===")

    # Create sample order
    order, cart = create_sample_order()

    print(f"Cart has {cart.get_total_items()} items")

    # Process order
    success = await process_order_async(order, cart)

    if success:
        print(f"Order completed successfully! Status: {order.status.name}")
    else:
        print("Order processing failed")


if __name__ == "__main__":
    asyncio.run(main())
