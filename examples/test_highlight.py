#!/usr/bin/env python3
"""Test file for Python syntax highlighting"""

import sys
from typing import List, Dict


class ExampleClass:
    """A simple example class"""

    def __init__(self, name: str):
        self.name = name
        self.items: List[str] = []

    def add_item(self, item: str) -> None:
        """Add an item to the list"""
        self.items.append(item)

    def get_items(self) -> List[str]:
        """Get all items"""
        return self.items


def main():
    """Main function"""
    example = ExampleClass("test")

    # Add some items
    for i in range(5):
        example.add_item(f"item_{i}")

    # Print items
    print(f"Items in {example.name}:")
    for item in example.get_items():
        print(f"  - {item}")


if __name__ == "__main__":
    main()
