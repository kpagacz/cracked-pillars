#!/usr/bin/env python3

import json
import logging
from pathlib import Path
from typing import List, Dict

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('item_post_processing.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

def post_process_items(input_file: Path, output_file: Path) -> None:
    """
    Post-process items by filtering out those with "No enchantment effects found."

    Args:
        input_file: Path to the input processed_items.json file
        output_file: Path to the output filtered_items.json file
    """
    logger.info(f"Starting post-processing of items from {input_file}")

    # Load the original processed items
    try:
        with open(input_file, 'r', encoding='utf-8') as f:
            items = json.load(f)
        logger.info(f"Loaded {len(items)} items from {input_file}")
    except Exception as e:
        logger.error(f"Failed to load items from {input_file}: {str(e)}")
        raise

    # Filter out items with "No enchantment effects found."
    filtered_items = []
    removed_count = 0

    for item in items:
        if item.get('effects_description') == "No enchantment effects found.":
            removed_count += 1
            logger.debug(f"Removing item: {item.get('name', 'Unknown')}")
        else:
            filtered_items.append(item)

    logger.info(f"Filtered out {removed_count} items with no enchantment effects")
    logger.info(f"Kept {len(filtered_items)} items with enchantment effects")

    # Save the filtered items
    try:
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(filtered_items, f, indent=2, ensure_ascii=False)
        logger.info(f"Saved filtered items to {output_file}")
    except Exception as e:
        logger.error(f"Failed to save filtered items to {output_file}: {str(e)}")
        raise

    # Print summary statistics
    print(f"\nPost-processing Summary:")
    print(f"Original items: {len(items)}")
    print(f"Items removed: {removed_count}")
    print(f"Items kept: {len(filtered_items)}")
    print(f"Removal percentage: {(removed_count / len(items) * 100):.1f}%")

def main():
    """Main function to run the post-processing."""
    # Define file paths
    input_file = Path(__file__).parent / 'processed_items.json'
    output_file = Path(__file__).parent / 'post_processed_items.json'

    # Check if input file exists
    if not input_file.exists():
        logger.error(f"Input file not found: {input_file}")
        print(f"Error: Input file not found: {input_file}")
        return

    try:
        post_process_items(input_file, output_file)
        print(f"\nPost-processing completed successfully!")
        print(f"Filtered items saved to: {output_file}")
    except Exception as e:
        logger.error(f"Post-processing failed: {str(e)}")
        print(f"Error: Post-processing failed: {str(e)}")

if __name__ == "__main__":
    main()
