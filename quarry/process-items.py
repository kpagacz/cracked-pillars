#!/usr/bin/env python3

import csv
import json
import logging
import os
import time
from pathlib import Path
from typing import Dict, List, Optional
import jsonschema
import google.generativeai as genai
from tenacity import retry, stop_after_attempt, wait_exponential
from dotenv import load_dotenv
import requests
from bs4 import BeautifulSoup, Tag
from urllib.parse import unquote
from extract_enchantment_effects import extract_enchantment_effects

# Load environment variables from .env file
load_dotenv(Path(__file__).parent / '.env')

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('item_processing.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

# Load schema for validation
SCHEMA_PATH = Path(__file__).parent / 'jsonschemas' / 'item-template.jsonschema'
with open(SCHEMA_PATH) as f:
    SCHEMA = json.load(f)

# Configuration
class Config:
    def __init__(self):
        # Try to get API key from environment
        self.api_key = os.getenv('GOOGLE_API_KEY')
        if not self.api_key:
            # Check if .env file exists
            env_path = Path(__file__).parent / '.env'
            if not env_path.exists():
                raise ValueError(
                    "GOOGLE_API_KEY not found in environment and .env file is missing.\n"
                    "Please create a .env file in the quarry directory with:\n"
                    "GOOGLE_API_KEY=your_api_key_here"
                )
            else:
                raise ValueError(
                    "GOOGLE_API_KEY not found in environment.\n"
                    "Please ensure your .env file contains:\n"
                    "GOOGLE_API_KEY=your_api_key_here"
                )

        # Configure the API
        genai.configure(api_key=self.api_key)
        self.model = genai.GenerativeModel('models/gemini-2.5-flash-preview-05-20')
        self.max_retries = 3
        self.batch_size = 10
        self.delay_between_batches = 2  # seconds

        # Input files
        self.items_csv = Path(__file__).parent / 'deadfire_items.csv'

        # Output file for processed items
        self.output_file = Path(__file__).parent / 'processed_items.json'

        # File to track progress
        self.progress_file = Path(__file__).parent / 'item_processing_progress.json'

class ItemProcessor:
    def __init__(self, config: Config):
        self.config = config
        self.processed_urls = self._load_progress()
        self.session = requests.Session()
        # Add a user agent to avoid being blocked
        self.session.headers.update({
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
        })

    def _load_progress(self) -> set:
        """Load already processed URLs from progress file."""
        if self.config.progress_file.exists():
            with open(self.config.progress_file) as f:
                return set(json.load(f))
        return set()

    def _save_progress(self):
        """Save progress to file."""
        with open(self.config.progress_file, 'w') as f:
            json.dump(list(self.processed_urls), f)

    def _load_items(self) -> List[Dict]:
        """Load all items from CSV file."""
        items: List[Dict] = []
        if not self.config.items_csv.exists():
            logger.warning(f"File not found: {self.config.items_csv}")
            return items

        with open(self.config.items_csv) as f:
            reader = csv.DictReader(f)
            for row in reader:
                # Skip category pages and empty entries
                if (row['URL'] and
                    row['Item Name'] and
                    'Category:' not in row['URL'] and
                    row['URL'] not in self.processed_urls):
                    items.append(row)

        return items

    def _fetch_raw_html(self, url: str) -> str:
        """Fetch the raw HTML from the wiki page."""
        try:
            logger.info(f"Fetching content from {url}")
            response = self.session.get(url)
            response.raise_for_status()

            return response.text
        except Exception as e:
            logger.error(f"Error fetching content from {url}: {str(e)}")
            raise

    @retry(stop=stop_after_attempt(3), wait=wait_exponential(multiplier=1, min=4, max=10))
    def _process_item(self, item: Dict) -> Optional[Dict]:
        """Process a single item through Gemini."""
        # Extract valid tags from schema
        valid_tags = SCHEMA['properties']['tags']['items']['enum']

        # Fetch the item content
        try:
            html_content = self._fetch_raw_html(url=item['URL'])
        except Exception as e:
            logger.error(f"Failed to fetch content for {item['URL']}: {str(e)}")
            raise

        # Extract enchantment effects using the existing function
        try:
            enchantment_data = extract_enchantment_effects(html_content)

            # Combine current and upgrade enchantments into a single description
            effects_description = ""

            if enchantment_data['current']:
                effects_description += "CURRENT ENCHANTMENTS:\n"
                for enchantment in enchantment_data['current']:
                    effects_description += f"- {enchantment['name']}: {enchantment['effect']}\n"
                effects_description += "\n"

            if enchantment_data['upgrades']:
                effects_description += "UPGRADE ENCHANTMENTS:\n"
                for enchantment in enchantment_data['upgrades']:
                    effects_description += f"- {enchantment['name']}: {enchantment['effect']}\n"

            if not effects_description.strip():
                effects_description = "No enchantment effects found."

        except Exception as e:
            logger.error(f"Failed to extract enchantment effects for {item['URL']}: {str(e)}")
            effects_description = "Error extracting enchantment effects."

        prompt = f"""Analyze this Pillars of Eternity 2: Deadfire item and return ONLY a list of relevant tags.

Item URL: {item['URL']}
Name: {item['Item Name']}

ITEM ENCHANTMENT EFFECTS:

{effects_description}

IMPORTANT: Use ONLY the enchantment effects provided above. Do not rely on any external knowledge about the item.

Return ONLY a JSON array of tags from this list:
{json.dumps(valid_tags, indent=2)}

TAGGING GUIDELINES:

1. Use damage type tags (slashing, piercing, burning, etc.) for direct damage effects
2. Use affliction/inspiration tags for status effects:
   - might afflictions are: staggered, dazed, stunned
   - dexterity afflications are: hobbled, immobilized, paralyzed, petrified
   - constitution afflictions are: sickened, weakened, enfeebled
   - intellect afflictions are: confused, charmed, dominated
   - perception afflictions are: distracted, disoriented, blinded
   - resolve afflictions are: shackled, frightened, terrified
   - might inspirations are: strong, tenacious, energized
   - dexterity inspirations are: quick, nimble, swift
   - constitution inspirations are: fit, hardy, robust
   - intellect inspirations are: smart, acute, brilliant
   - perception inspirations are: insightful, aware, intuitive
   - resolve inspirations are: steadfast, resolute, courageous
3. If the effect targets will, use "targets_will" tag
4. If the effect targets reflex, use "targets_reflex" tag
5. If the effect targets fortitude, use "targets_fortitude" tag
6. If the effect targets deflection, use "targets_deflection" tag
7. If the effect only lists +5 to some attribute, do not use "x_inspiration" tag,
   instead use mod_x tag (e.g. +5 Might is mod_might, +25% action speed is mod_action_speed)
8. If the effect only lists -5 to some attribute, do not use "x_afflication" tag,
   instead use mod_x tag (e.g. -5 Consitution is mod_constitution, -25% action speed is mod_action_speed)
9. Use mod_* tags for stat modifications (e.g. + 5 Might is mod_might, -5 Intellect is mod_intellect)
10. Use summon_* tags for summoning effects
11. If the effect is +X to all defenses, use 4 tags for this effect:
    mod_deflection, mod_fortitude, mod_reflex, mod_will
12. Use special tag only if no other tag fits and the effect is unique
13. For armor effects, use the appropriate armor tags (slashing_armour, piercing_armour, etc.)
14. For weapon effects, use damage type tags and weapon-specific tags
15. For accessory effects, use appropriate stat modification tags
16. Use counterattack tag for effects that trigger counterattacks when hit or attacked or missed.
17. Use lash tag for items that have an effect that mentions Lash.

Focus on:
1. Correctly identifying all enchantment effects
2. Properly tagging the item using ONLY the valid tags listed above
3. Accurately capturing the effects description

Return ONLY the JSON array of tags, no other text. The response must be valid JSON array containing only tags from the valid list."""

        try:
            response = self.config.model.generate_content(
                prompt,
                generation_config={
                    "temperature": 0.1,  # Low temperature for more consistent output
                    "top_p": 0.8,
                    "top_k": 40,
                }
            )

            # Extract JSON array from response
            try:
                # Try to find JSON array in the response text
                text = response.text
                # Find the first [ and last ] to extract JSON array
                start = text.find('[')
                end = text.rfind(']') + 1
                if start == -1 or end == 0:
                    raise ValueError("No JSON array found in response")
                json_str = text[start:end]
                tags = json.loads(json_str)

                # Validate that all tags are in the valid list
                if not isinstance(tags, list):
                    raise ValueError("Response is not a list")

                invalid_tags = [tag for tag in tags if tag not in valid_tags]
                if invalid_tags:
                    raise ValueError(f"Invalid tags found: {invalid_tags}")

            except (json.JSONDecodeError, ValueError) as e:
                logger.error(f"Failed to parse JSON from response: {text}")
                raise

            # Construct the result object
            result = {
                "name": item['Item Name'],
                "wiki_url": item['URL'],
                "tags": tags,
                "effects_description": effects_description
            }

            # Validate against schema
            jsonschema.validate(instance=result, schema=SCHEMA)

            return result

        except Exception as e:
            logger.error(f"Error processing item {item['URL']}: {str(e)}")
            raise

    def process_all(self):
        """Process all unprocessed items."""
        items = self._load_items()
        logger.info(f"Found {len(items)} items to process")

        processed_results = []
        if self.config.output_file.exists():
            with open(self.config.output_file) as f:
                processed_results = json.load(f)

        for i in range(0, len(items), self.config.batch_size):
            batch = items[i:i + self.config.batch_size]
            logger.info(f"Processing batch {i//self.config.batch_size + 1}/{(len(items)-1)//self.config.batch_size + 1}")

            for item in batch:
                try:
                    result = self._process_item(item)
                    if result:
                        processed_results.append(result)
                        self.processed_urls.add(item['URL'])

                        # Save progress after each successful processing
                        self._save_progress()
                        with open(self.config.output_file, 'w') as f:
                            json.dump(processed_results, f, indent=2)

                except Exception as e:
                    logger.error(f"Failed to process {item['URL']} after retries: {str(e)}")
                    continue

            # Delay between batches to avoid rate limits
            if i + self.config.batch_size < len(items):
                time.sleep(self.config.delay_between_batches)

        logger.info("Processing completed!")

def main():
    try:
        config = Config()
        processor = ItemProcessor(config)
        processor.process_all()
    except Exception as e:
        logger.error(f"Fatal error: {str(e)}")
        raise

if __name__ == "__main__":
    main()
