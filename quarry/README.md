# Quarry

The `quarry` directory contains tools and scripts for extracting, processing, and structuring game data from Pillars of Eternity II: Deadfire. This data is used to build a comprehensive knowledge base of game mechanics, abilities, and items.

## Purpose

This directory serves as a data extraction and processing pipeline that:
1. Scrapes game data from the official wiki
2. Processes and structures the data using LLMs
3. Creates a searchable index of game abilities and items
4. Validates the processed data against a schema

## Scripts and Files

### Data Collection Scripts
- `scrape-abilities.py` - Scrapes active, passive, and modal abilities from the wiki
- `scrape-items.py` - Scrapes item data from the wiki
- Output files:
  - `deadfire_active_abilities.csv`
  - `deadfire_passive_abilities.csv`
  - `deadfire_modal_abilities.csv`
  - `deadfire_items.csv`

### Data Processing Scripts
- `process-abilities.py` - Uses LLM to process and structure ability data
- `validate-example.py` - Validates processed data against the schema
- Output files:
  - `processed_abilities.json` - Final processed ability data
  - `processing_progress.json` - Tracks processing status
  - `ability_processing.log` - Processing logs

### Schema and Examples
- `spell-template.jsonschema` - JSON schema for validating ability data
  - Defines the structure of processed abilities
  - Includes enums for standardized fields like `activation` and `targets`
  - Validates effect tags and targeting types
- Example files demonstrating different ability types:
  - `spell-example.json` - A wizard spell (Citzal's Martial Power) with self-targeting effects
  - `spell-example-2.json` - A barbarian passive (Carnage) with area effect targeting
  - `spell-example-3.json` - A chanter invocation with summoning effects
- `tags.md` - Documentation of ability tags and categories

### Data Structure
The processed abilities follow a strict schema with standardized fields:

#### Core Fields
- `name` - Ability name
- `url` - Wiki source URL
- `description` - Full ability description
- `activation` - One of: "passive", "active", "modal"
- `effects` - Array of effect objects

#### Effect Objects
Each effect has:
- `targets` - Array of target types (enum):
  - `self` - Affects the caster
  - `allied_aoe` - Area effect on allies
  - `hazard_aoe` - Area effect that creates a hazard
  - `foe_aoe` - Area effect on enemies
  - `foe_target` - Single enemy target
  - `attackers` - Affects those attacking the caster
  - `target` - Generic target
  - `jump_targets` - For abilities that involve jumping/movement
  - `summon` - For summoning effects
  - `target+beam` - For beam effects that require a target
  - `friendly_target` - For targeting allies
- `value` - Effect description
- `duration` - Effect duration in seconds (if applicable)
- `tags` - Array of standardized effect tags

## Usage

To process the game data, follow these steps in order:

1. **Setup Environment**
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Unix/macOS
   pip install -r requirements.txt
   ```

2. **Data Collection**
   ```bash
   python scrape-abilities.py  # Scrapes ability data
   python scrape-items.py      # Scrapes item data
   ```

3. **Data Processing**
   ```bash
   python process-abilities.py  # Processes and structures ability data
   ```

4. **Validation** (Optional)
   ```bash
   python validate-example.py  # Validates processed data against schema
   ```

## Directory Structure
- `bin/` - Executable scripts and utilities
- `lib/` - Library code and shared utilities
- `include/` - Header files and dependencies

## Notes
- The processed data is stored in JSON format for easy integration with other tools
- The schema ensures consistency in the processed data through enums and validation
- Processing logs and progress are tracked for monitoring and debugging
- All scraped data is stored in CSV format for easy inspection and modification
- Example files demonstrate the correct usage of the schema and can be used as templates
