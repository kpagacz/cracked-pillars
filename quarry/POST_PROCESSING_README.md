# Post-Processing Scripts

This directory contains post-processing scripts for filtering and cleaning the processed data from the main processing scripts.

## Scripts

### `post-process-items.py`

Filters out items that have no enchantment effects from the processed items data.

**Usage:**
```bash
python post-process-items.py
```

**What it does:**
- Reads from `processed_items.json`
- Filters out items where `effects_description` equals "No enchantment effects found."
- Saves filtered results to `post_processed_items.json`
- Provides summary statistics

**Results from current data:**
- Original items: 1,650
- Items removed: 1,271 (77.0%)
- Items kept: 379 (23.0%)

### `post-process-abilities.py`

Filters abilities based on origin type and class criteria. Currently configured to keep abilities with:
- Origin type = "item" OR
- Origin type = "progression" AND class in ["priest", "chanter", "rogue", "cipher", "ranger", "fighter", "paladin", "wizard", "monk"]

**Usage:**
```bash
python post-process-abilities.py
```

**What it does:**
- Reads from `processed_abilities.json`
- Filters abilities based on origin type and class criteria
- Saves results to `post_processed_abilities.json`
- Provides summary statistics

**Results from current data:**
- Original abilities: 1,870
- Abilities removed: 766 (41.0%)
- Abilities kept: 1,104 (59.0%)

**Available Filter Functions:**

1. `filter_by_origin_type_and_class()` - **Currently active** - Filters by origin type and allowed classes
2. `filter_offensive_abilities()` - Keep only offensive abilities
3. `filter_by_activation_type(ability, 'active')` - Keep only active abilities
4. `filter_by_class(ability, 'wizard')` - Keep only wizard abilities
5. `filter_abilities_with_effects()` - Keep only abilities with effects

**To change filtering, modify the script:**
```python
# In the main() function, change:
filter_func = filter_by_origin_type_and_class

# To one of these examples:
filter_func = filter_offensive_abilities
filter_func = lambda ability: filter_by_activation_type(ability, 'active')
filter_func = lambda ability: filter_by_class(ability, 'wizard')
filter_func = filter_abilities_with_effects
filter_func = None  # No filtering
```

## Output Files

- `post_processed_items.json` - Items with enchantment effects only
- `post_processed_abilities.json` - Abilities filtered by origin type and class
- `item_post_processing.log` - Log file for items post-processing
- `ability_post_processing.log` - Log file for abilities post-processing

## Customization

Both scripts are designed to be easily extensible:

- **Items script**: Modify the filtering condition in the main loop
- **Abilities script**: Add new filter functions or modify existing ones

The abilities script is particularly flexible and can be adapted for various filtering needs without changing the core processing logic.
