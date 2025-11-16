// Test file for JavaScript syntax highlighting

/**
 * Example class
 */
class Example {
    constructor(name) {
        this.name = name;
        this.items = [];
    }

    /**
     * Add an item
     * @param {string} item - The item to add
     */
    addItem(item) {
        this.items.push(item);
    }

    /**
     * Get all items
     * @returns {Array} The items
     */
    getItems() {
        return this.items;
    }
}

// Main function
function main() {
    const example = new Example("test");

    // Add some items
    for (let i = 0; i < 5; i++) {
        example.addItem(`item_${i}`);
    }

    // Print items
    console.log(`Items in ${example.name}:`);
    example.getItems().forEach(item => {
        console.log(`  - ${item}`);
    });
}

main();
