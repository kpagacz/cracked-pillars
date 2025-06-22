#!/usr/bin/env python3

import json
import logging
from pathlib import Path
from typing import List, Dict, Callable, Optional

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('ability_post_processing.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

def filter_abilities_by_criteria(abilities: List[Dict], filter_func: Callable[[Dict], bool]) -> List[Dict]:
    """
    Filter abilities based on a custom filter function.

    Args:
        abilities: List of ability dictionaries
        filter_func: Function that takes an ability dict and returns True to keep, False to remove

    Returns:
        List of filtered abilities
    """
    filtered_abilities = []
    removed_count = 0

    for ability in abilities:
        if filter_func(ability):
            filtered_abilities.append(ability)
        else:
            removed_count += 1
            logger.debug(f"Removing ability: {ability.get('name', 'Unknown')}")

    logger.info(f"Filtered out {removed_count} abilities")
    logger.info(f"Kept {len(filtered_abilities)} abilities")

    return filtered_abilities

def post_process_abilities(input_file: Path, output_file: Path, filter_func: Optional[Callable[[Dict], bool]] = None) -> None:
    """
    Post-process abilities by applying custom filtering criteria.

    Args:
        input_file: Path to the input processed_abilities.json file
        output_file: Path to the output filtered_abilities.json file
        filter_func: Optional custom filter function. If None, no filtering is applied.
    """
    logger.info(f"Starting post-processing of abilities from {input_file}")

    # Load the original processed abilities
    try:
        with open(input_file, 'r', encoding='utf-8') as f:
            abilities = json.load(f)
        logger.info(f"Loaded {len(abilities)} abilities from {input_file}")
    except Exception as e:
        logger.error(f"Failed to load abilities from {input_file}: {str(e)}")
        raise

    # Apply filtering if a filter function is provided
    if filter_func is not None:
        abilities = filter_abilities_by_criteria(abilities, filter_func)

    # Save the processed abilities
    try:
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(abilities, f, indent=2, ensure_ascii=False)
        logger.info(f"Saved processed abilities to {output_file}")
    except Exception as e:
        logger.error(f"Failed to save processed abilities to {output_file}: {str(e)}")
        raise

    # Print summary statistics
    print(f"\nPost-processing Summary:")
    print(f"Original abilities: {len(abilities)}")
    if filter_func is not None:
        print(f"Filtering applied: Yes")
    else:
        print(f"Filtering applied: No")
    print(f"Final abilities: {len(abilities)}")

# Example filter functions that can be used or extended
def filter_offensive_abilities(ability: Dict) -> bool:
    """Filter to keep only offensive abilities."""
    keywords = ability.get('keywords', [])
    return any(keyword.lower() in ['offensive', 'damage', 'attack'] for keyword in keywords)

def filter_by_activation_type(ability: Dict, activation_type: str) -> bool:
    """Filter abilities by activation type (active, passive, modal)."""
    return ability.get('activation', '').lower() == activation_type.lower()

def filter_by_class(ability: Dict, class_name: str) -> bool:
    """Filter abilities by class."""
    origin = ability.get('origin', {})
    if origin.get('type') == 'progression':
        class_info = origin.get('value', {}).get('class', '')
        return class_info.lower() == class_name.lower()
    return False

def filter_abilities_with_effects(ability: Dict) -> bool:
    """Filter to keep only abilities that have effects."""
    effects = ability.get('effects', [])
    return len(effects) > 0

def main():
    """Main function to run the post-processing."""
    # Define file paths
    input_file = Path(__file__).parent / 'processed_abilities.json'
    output_file = Path(__file__).parent / 'post_processed_abilities.json'

    # Check if input file exists
    if not input_file.exists():
        logger.error(f"Input file not found: {input_file}")
        print(f"Error: Input file not found: {input_file}")
        return

    # Example: Apply a filter (uncomment and modify as needed)
    # filter_func = filter_offensive_abilities  # Keep only offensive abilities
    # filter_func = lambda ability: filter_by_activation_type(ability, 'active')  # Keep only active abilities
    # filter_func = lambda ability: filter_by_class(ability, 'wizard')  # Keep only wizard abilities
    # filter_func = filter_abilities_with_effects  # Keep only abilities with effects

    # For now, no filtering is applied (pass None as filter_func)
    filter_func = None

    try:
        post_process_abilities(input_file, output_file, filter_func)
        print(f"\nPost-processing completed successfully!")
        print(f"Processed abilities saved to: {output_file}")
    except Exception as e:
        logger.error(f"Post-processing failed: {str(e)}")
        print(f"Error: Post-processing failed: {str(e)}")

if __name__ == "__main__":
    main()
