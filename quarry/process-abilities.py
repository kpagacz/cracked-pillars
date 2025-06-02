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

# Load environment variables from .env file
load_dotenv(Path(__file__).parent / '.env')

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('ability_processing.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger(__name__)

# Load schema for validation
SCHEMA_PATH = Path(__file__).parent / 'spell-template.jsonschema'
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
        self.active_abilities_csv = Path(__file__).parent / 'deadfire_active_abilities.csv'
        self.passive_abilities_csv = Path(__file__).parent / 'deadfire_passive_abilities.csv'
        self.modal_abilities_csv = Path(__file__).parent / 'deadfire_modal_abilities.csv'

        # Output file for processed abilities
        self.output_file = Path(__file__).parent / 'processed_abilities.json'

        # File to track progress
        self.progress_file = Path(__file__).parent / 'processing_progress.json'

class AbilityProcessor:
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

    def _load_abilities(self) -> List[Dict]:
        """Load all abilities from CSV files."""
        abilities = []
        for csv_file in [self.config.active_abilities_csv,
                        self.config.passive_abilities_csv,
                        self.config.modal_abilities_csv]:
            if not csv_file.exists():
                logger.warning(f"File not found: {csv_file}")
                continue

            with open(csv_file) as f:
                reader = csv.DictReader(f)
                for row in reader:
                    if row['URL'] not in self.processed_urls:
                        abilities.append(row)

        return abilities

    def _extract_text_from_html(self, html: str) -> str:
        """Extract (and return) the text (and icon names) from the given HTML string. Do not drop anything. In addition, replace images (i.e. <img> tags) with their alternative text (i.e. the value of the 'alt' attribute)."""
        soup = BeautifulSoup(html, "html.parser")
        for img in soup.find_all("img"):
             if not isinstance(img, Tag):
                 continue
             alt_attr = img.get("alt")
             alt = (alt_attr if isinstance(alt_attr, str) else "image")
             new_tag = BeautifulSoup("<span>" + alt + "</span>", "html.parser").span
             if new_tag is not None:
                 img.replace_with(new_tag)
        # (We use get_text(separator=" ", strip=True) to extract all text nodes (and icon names) and join them with a space.)
        return soup.get_text(separator=" ", strip=True)

    def _fetch_raw_html(self, url: str) -> str:
        """Fetch the raw HTML from the left-hand side (mw-parser-output) of the wiki page, then extract (and return) the text (and icon names) from it."""
        try:
            logger.info(f"Fetching content from {url}")
            response = self.session.get(url)
            response.raise_for_status()

            soup = BeautifulSoup(response.text, "html.parser")
            logger.debug("Successfully parsed HTML")

            # --- Left-hand side (mw-parser-output) ---
            left_div = soup.find("div", {"class": "mw-parser-output"})
            left_html = str(left_div) if left_div else ""
            left_text = self._extract_text_from_html(html=left_html)
            logger.debug("Extracted left-hand side text (and icon names)")

            return left_text
        except Exception as e:
            logger.error(f"Error fetching content from {url}: {str(e)}")
            raise

    @retry(stop=stop_after_attempt(3), wait=wait_exponential(multiplier=1, min=4, max=10))
    def _process_ability(self, ability: Dict) -> Optional[Dict]:
        """Process a single ability through Gemini."""
        # Extract valid tags from schema
        valid_tags = SCHEMA['properties']['effects']['items']['properties']['tags']['items']['enum']
        valid_targets = SCHEMA['properties']['effects']['items']['properties']['targets']['items']['enum']

        # Fetch the ability content
        try:
            content = self._fetch_raw_html(url=ability['URL'])
        except Exception as e:
            logger.error(f"Failed to fetch content for {ability['URL']}: {str(e)}")
            raise

        prompt = f"""Analyze this Pillars of Eternity 2: Deadfire ability and return a structured summary.

Ability URL: {ability['URL']}
Name: {ability['Ability Name']}

ABILITY CONTENT FROM WIKI:

{content}

IMPORTANT: Use ONLY the ability content provided above. Do not rely on any external knowledge about the ability.

Please analyze the ability and return a JSON object matching this schema:
{json.dumps(SCHEMA, indent=2)}

IMPORTANT INSTRUCTIONS:

1. TARGET SELECTION:
   - For each effect, you MUST ONLY use target types from this list:
   {json.dumps(valid_targets, indent=2)}
   - Choose the most appropriate target type(s) for each effect
   - Use multiple targets if an effect affects multiple types of targets
   - Common patterns:
     * Self-buffs use "self"
     * Area effects on enemies use "foe_aoe"
     * Single target effects use "foe_target" or "friendly_target"
     * Summoning effects use "summon"
     * Beam effects use "target+beam"

2. TAGGING EFFECTS:
   - When tagging effects, you MUST ONLY use tags from this list:
   {json.dumps(valid_tags, indent=2)}
   - For each effect:
     * Analyze the effect description carefully
     * Match it to the most appropriate tags from the valid list above
     * DO NOT create new tags or use tags not in the list
     * If an effect doesn't clearly match any tag, omit the tag rather than guessing
     * Use multiple tags when an effect has multiple aspects (e.g., both damage type and status effect)
     * DO NOT use tags for attack modifiers of the ability itself
       (e.g., accuracy bonus, damage bonus of the full attack of the ability)

3. Tag selection guidelines:
   - Use class tags (barbarian, chanter, etc.) to indicate with which class the ability is associated
   - Use damage type tags (slashing, piercing, burning, etc.) for direct damage effects
   - Use affliction/inspiration tags for status effects
   - Use mod_* tags for stat modifications
   - Use summon_* tags for summoning effects
   - Use special tag only if no other tag fits and the effect is unique

Focus on:
1. Correctly identifying all effects and their targets using ONLY the valid target types
2. Properly tagging the ability using ONLY the valid tags listed above
3. Accurately capturing duration and conditions
4. Identifying the origin (class progression or item)

Return ONLY the JSON object, no other text. The response must be valid JSON that matches the schema exactly."""

        try:
            response = self.config.model.generate_content(
                prompt,
                generation_config={
                    "temperature": 0.1,  # Low temperature for more consistent output
                    "top_p": 0.8,
                    "top_k": 40,
                }
            )

            # Extract JSON from response
            try:
                # Try to find JSON in the response text
                text = response.text
                # Find the first { and last } to extract JSON
                start = text.find('{')
                end = text.rfind('}') + 1
                if start == -1 or end == 0:
                    raise ValueError("No JSON object found in response")
                json_str = text[start:end]
                result = json.loads(json_str)
            except (json.JSONDecodeError, ValueError) as e:
                logger.error(f"Failed to parse JSON from response: {text}")
                raise

            # Validate against schema
            jsonschema.validate(instance=result, schema=SCHEMA)

            return result

        except Exception as e:
            logger.error(f"Error processing ability {ability['URL']}: {str(e)}")
            raise

    def process_all(self):
        """Process all unprocessed abilities."""
        abilities = self._load_abilities()
        logger.info(f"Found {len(abilities)} abilities to process")

        processed_results = []
        if self.config.output_file.exists():
            with open(self.config.output_file) as f:
                processed_results = json.load(f)

        for i in range(0, len(abilities), self.config.batch_size):
            batch = abilities[i:i + self.config.batch_size]
            logger.info(f"Processing batch {i//self.config.batch_size + 1}/{(len(abilities)-1)//self.config.batch_size + 1}")

            for ability in batch:
                try:
                    result = self._process_ability(ability)
                    if result:
                        processed_results.append(result)
                        self.processed_urls.add(ability['URL'])

                        # Save progress after each successful processing
                        self._save_progress()
                        with open(self.config.output_file, 'w') as f:
                            json.dump(processed_results, f, indent=2)

                except Exception as e:
                    logger.error(f"Failed to process {ability['URL']} after retries: {str(e)}")
                    continue

            # Delay between batches to avoid rate limits
            if i + self.config.batch_size < len(abilities):
                time.sleep(self.config.delay_between_batches)

        logger.info("Processing completed!")

def main():
    try:
        config = Config()
        processor = AbilityProcessor(config)
        processor.process_all()
    except Exception as e:
        logger.error(f"Fatal error: {str(e)}")
        raise

if __name__ == "__main__":
    main()
