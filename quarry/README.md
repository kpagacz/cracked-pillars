# Quarry - Pillars of Eternity 2: Deadfire Data Pipeline

A comprehensive data extraction and processing pipeline that builds a searchable knowledge base for Pillars of Eternity 2: Deadfire game mechanics.

## Overview

This directory contains tools for:

- **Scraping game data** (abilities, items) from the official wiki
- **Processing and structuring the data** using LLMs (Google Gemini)
- **Creating a searchable index** of game mechanics
- **Validating the processed data** against JSON schemas

## Project Structure

```
quarry/
├── scrape-abilities.py          # Scrapes ability data from wiki
├── scrape-items.py              # Scrapes item data from wiki
├── process-abilities.py         # Processes abilities using LLM
├── validate-example.py          # Validates processed data
├── spell-template.jsonschema    # JSON schema for ability data
├── requirements.txt             # Python dependencies
├── tags.md                      # Available effect tags
├── *.csv                        # Raw scraped data
├── *.json                       # Processed and example data
└── *.log                        # Processing logs
```

## Features

### Data Scraping
- **Wiki Scraping**: Automated extraction from the official Pillars of Eternity wiki
- **Multiple Data Types**: Abilities (active, passive, modal) and items
- **Robust Error Handling**: Retry logic and polite rate limiting
- **Progress Tracking**: Resume capability for interrupted scraping

### Data Processing
- **LLM-Powered Analysis**: Uses Google Gemini to structure raw wiki content
- **Structured Output**: Converts unstructured text into validated JSON
- **Effect Tagging**: Categorizes abilities with standardized tags
- **Target Classification**: Identifies ability targets (self, foes, allies, etc.)

### Data Validation
- **JSON Schema Validation**: Ensures data integrity and consistency
- **Comprehensive Schema**: Covers all ability properties and relationships
- **Example Validation**: Test suite with sample data

## Installation

1. **Clone the repository** and navigate to the quarry directory:
   ```bash
   cd quarry
   ```

2. **Create a virtual environment**:
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```

3. **Install dependencies**:
   ```bash
   pip install -r requirements.txt
   ```

4. **Set up API credentials**:
   Create a `.env` file in the quarry directory:
   ```
   GOOGLE_API_KEY=your_google_api_key_here
   ```

## Usage

### 1. Scraping Game Data

**Scrape abilities:**
```bash
python scrape-abilities.py
```
This creates:
- `deadfire_active_abilities.csv`
- `deadfire_passive_abilities.csv`
- `deadfire_modal_abilities.csv`

**Scrape items:**
```bash
python scrape-items.py
```
This creates:
- `deadfire_items.csv`

### 2. Processing Abilities

**Process scraped abilities using LLM:**
```bash
python process-abilities.py
```

This script:
- Loads scraped ability data from CSV files
- Fetches detailed content from wiki pages
- Uses Google Gemini to analyze and structure the data
- Outputs structured JSON to `processed_abilities.json`
- Tracks progress in `processing_progress.json`

**Features:**
- Resume capability (skips already processed abilities)
- Batch processing with rate limiting
- Comprehensive error handling and logging
- Schema validation of output

### 3. Validating Data

**Validate example data:**
```bash
python validate-example.py --file spell-example.json
```

**Validate your own data:**
```bash
python validate-example.py --file your-data.json
```

## Data Schema

The processed abilities follow a comprehensive JSON schema (`spell-template.jsonschema`) that includes:

### Core Properties
- **name**: Ability name
- **url**: Wiki source URL
- **description**: Ability description
- **activation**: Type (passive/active/modal)
- **effects**: Array of ability effects
- **origin**: How the ability is acquired

### Effect Structure
Each effect contains:
- **targets**: Who the effect affects (self, foes, allies, etc.)
- **value**: Effect description
- **tags**: Categorized effect types
- **condition**: Trigger conditions (optional)
- **duration**: Effect duration in seconds (optional)

### Available Tags
The system supports 145+ standardized tags covering:
- **Classes**: barbarian, chanter, cipher, etc.
- **Damage Types**: slashing, piercing, shock, burn, etc.
- **Afflictions**: sickened, weakened, paralyzed, etc.
- **Inspirations**: fit, hardy, robust, etc.
- **Combat Modifiers**: mod_accuracy, mod_deflection, etc.
- **Special Effects**: summon_creature, transform, etc.

See `tags.md` for the complete list.

## Output Files

### Raw Data (CSV)
- `deadfire_active_abilities.csv`: Active abilities with URLs
- `deadfire_passive_abilities.csv`: Passive abilities with URLs
- `deadfire_modal_abilities.csv`: Modal abilities with URLs
- `deadfire_items.csv`: Items with URLs

### Processed Data (JSON)
- `processed_abilities.json`: Structured ability data
- `processing_progress.json`: Processing state tracking
- `ability_processing.log`: Detailed processing logs

### Example Data
- `spell-example.json`: Sample processed ability
- `spell-example-2.json`: Additional examples
- `spell-example-3.json`: More examples

## Configuration

### Environment Variables
- `GOOGLE_API_KEY`: Required for LLM processing

### Processing Options
The `process-abilities.py` script includes configurable options:
- **Batch size**: Number of abilities processed per batch
- **Retry attempts**: Number of retries for failed requests
- **Rate limiting**: Delay between API calls
- **Model selection**: Gemini model version

## Error Handling

The pipeline includes robust error handling:
- **Network failures**: Automatic retries with exponential backoff
- **API limits**: Rate limiting and polite delays
- **Invalid data**: Schema validation and error reporting
- **Progress tracking**: Resume capability for interrupted processing

## Contributing

When adding new features:
1. Update the JSON schema if adding new data fields
2. Add corresponding tags to `tags.md`
3. Update validation examples
4. Test with the validation script

## Dependencies

- **google-generativeai**: LLM processing
- **python-dotenv**: Environment variable management
- **jsonschema**: Data validation
- **tenacity**: Retry logic
- **requests**: HTTP requests
- **beautifulsoup4**: HTML parsing
