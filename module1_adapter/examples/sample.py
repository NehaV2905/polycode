"""
Example Python code for testing the Language Adapter.

This file demonstrates various language constructs that should be
detected by the IR event extractor.
"""

import os
import sys
from typing import List, Optional


class UserManager:
    """Manages user authentication and data."""
    
    def __init__(self, database_url: str):
        self.database_url = database_url
        self.connected = False
    
    def connect(self):
        """Establish database connection."""
        if not self.connected:
            print(f"Connecting to {self.database_url}")
            self.connected = True
            return True
        return False
    
    def create_user(self, username: str, password: str) -> Optional[int]:
        """Create a new user account."""
        if not username or not password:
            return None
        
        hashed_password = hash_password(password)
        user_id = self._insert_user(username, hashed_password)
        
        return user_id
    
    def _insert_user(self, username: str, hashed: str) -> int:
        """Internal method to insert user into database."""
        # Simulate database insertion
        return 42


def hash_password(password: str) -> str:
    """Hash a password using a secure algorithm."""
    import hashlib
    return hashlib.sha256(password.encode()).hexdigest()


def login(username: str, password: str) -> bool:
    """Authenticate a user."""
    manager = UserManager("sqlite:///users.db")
    manager.connect()
    
    # Check credentials
    hashed = hash_password(password)
    result = check_credentials(username, hashed)
    
    if result:
        print(f"User {username} logged in successfully")
        return True
    else:
        print(f"Login failed for {username}")
        return False


def check_credentials(username: str, hashed: str) -> bool:
    """Verify user credentials against database."""
    # Simulate credential check
    return True


def process_users(usernames: List[str]):
    """Process a batch of users."""
    for username in usernames:
        print(f"Processing {username}")
        
        if len(username) > 20:
            print("Username too long")
            continue
        
        try:
            result = validate_username(username)
            if result:
                print(f"{username} is valid")
        except ValueError as e:
            print(f"Validation error: {e}")


def validate_username(username: str) -> bool:
    """Validate a username format."""
    if not username.isalnum():
        raise ValueError("Username must be alphanumeric")
    return True


def main():
    """Main entry point."""
    users = ["alice", "bob", "charlie"]
    
    # Process all users
    process_users(users)
    
    # Test login
    success = login("alice", "secret123")
    
    if success:
        print("Application started")
    else:
        print("Failed to start")
        sys.exit(1)


if __name__ == "__main__":
    main()
