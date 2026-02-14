import os
import sys
from typing import List, Dict, Optional
from dataclasses import dataclass

@dataclass
class Config:
    host: str
    port: int
    enabled: bool = True

class UserManager:
    def __init__(self, config: Config):
        self.config = config
        self.users = []
        self.cache = {}
    
    def add_user(self, name: str, email: str) -> bool:
        if not name or not email:
            raise ValueError("Name and email required")
        
        user = {"name": name, "email": email}
        self.users.append(user)
        return True
    
    def get_user(self, name: str) -> Optional[Dict]:
        for user in self.users:
            if user["name"] == name:
                return user
        return None
    
    async def fetch_user_data(self, user_id: int):
        data = await self._async_request(user_id)
        return data
    
    async def _async_request(self, user_id: int):
        # Simulate async operation
        return {"id": user_id, "data": "sample"}

def hash_password(password: str) -> str:
    return f"hashed_{password}"

def check_credentials(username: str, password: str) -> bool:
    hashed = hash_password(password)
    
    if len(username) < 3:
        raise ValueError("Username too short")
    
    try:
        stored = get_stored_password(username)
        return stored == hashed
    except KeyError as e:
        print(f"User not found: {e}")
        return False
    except Exception:
        return False

def get_stored_password(username: str) -> str:
    passwords = {"alice": "hashed_secret123", "bob": "hashed_pass456"}
    return passwords[username]

def process_users(users: List[str]):
    # Lambda and filter
    valid_users = list(filter(lambda x: len(x) > 0, users))
    
    # List comprehension with member access
    user_data = [get_user_info(u) for u in valid_users]
    
    # Map
    sorted_users = sorted(users, key=lambda x: len(x))
    
    for user in valid_users:
        if user.startswith("admin_"):
            print(f"Admin user: {user}")
        elif user.startswith("guest_"):
            print(f"Guest user: {user}")
        else:
            print(f"Regular user: {user}")

def get_user_info(username: str) -> Dict:
    return {"username": username, "active": True}

def main():
    config = Config("localhost", 8080)
    manager = UserManager(config)
    
    try:
        manager.add_user("Alice", "alice@example.com")
        manager.add_user("Bob", "bob@example.com")
    except ValueError as ve:
        print(f"Error adding user: {ve}")
    
    user = manager.get_user("Alice")
    if user:
        print(f"Found user: {user['name']}")
    
    # Test credentials
    is_valid = check_credentials("alice", "secret123")
    
    # Process batch
    users = ["alice", "bob", "admin_charlie", "guest_dave"]
    process_users(users)
    
    # Async example (would need event loop)
    # result = await manager.fetch_user_data(1)
    
    print("Done")

if __name__ == "__main__":
    main()
