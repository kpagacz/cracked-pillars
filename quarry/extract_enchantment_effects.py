#!/usr/bin/env python3
"""
Script to extract enchantment effects from Pillars of Eternity II: Deadfire item HTML files.
Supports both regular items and soulbound items with different table structures.
"""

import sys
import re
from typing import List, Dict, Optional
from bs4 import BeautifulSoup, Tag, NavigableString


def clean_text(text: str) -> str:
    """
    Clean and normalize text by removing extra whitespace and newlines.

    Args:
        text: Raw text string

    Returns:
        Cleaned text string
    """
    if not text:
        return ""
    # Replace multiple whitespace with single space
    text = re.sub(r'\s+', ' ', text)
    # Remove leading/trailing whitespace
    text = text.strip()
    return text


def is_soulbound_item(html_content: str) -> bool:
    """
    Check if the item is soulbound by looking at the is_soulbound data source value.

    Args:
        html_content: The HTML content as a string

    Returns:
        True if the item is soulbound, False otherwise
    """
    soup = BeautifulSoup(html_content, 'html.parser')

    # Find the is_soulbound data source
    soulbound_div = soup.find('div', {'data-source': 'is_soulbound'})
    if not soulbound_div or not isinstance(soulbound_div, Tag):
        return False

    # Look for the value in the pi-data-value div
    value_div = soulbound_div.find('div', {'class': 'pi-data-value'})
    if not value_div or not isinstance(value_div, Tag):
        return False

    # Check if there's an X mark (No) or check mark (Yes)
    img = value_div.find('img')
    if img and isinstance(img, Tag):
        alt_text = img.get('alt', '')
        if alt_text:
            alt_text = str(alt_text).lower()
            if 'no' in alt_text or 'x' in alt_text:
                return False
            elif 'yes' in alt_text or 'check' in alt_text:
                return True

    # Fallback: check the text content
    text_content = value_div.get_text().lower().strip()
    if 'no' in text_content or 'false' in text_content:
        return False
    elif 'yes' in text_content or 'true' in text_content:
        return True

    return False


def extract_regular_upgrade_effects(html_content: str) -> List[Dict[str, str]]:
    """
    Extract enchantment upgrade effects from regular items using the rowspan table structure.

    Args:
        html_content: The HTML content as a string

    Returns:
        List of dictionaries containing upgrade enchantment names and their effects
    """
    soup = BeautifulSoup(html_content, 'html.parser')
    upgrades: List[Dict[str, str]] = []

    upgrades_table = soup.find('table', {'class': 'wikitable'})
    if not upgrades_table:
        return upgrades

    rows = upgrades_table.find_all('tr')
    for row in rows:
        ths = row.find_all('th')
        tds = row.find_all('td')
        # Look for a th with rowspan=2 and a link (enchantment name)
        enchantment_th = None
        for th in ths:
            if th.get('rowspan') == '2' and th.find('a'):
                enchantment_th = th
                break
        # Look for a td with rowspan=2 (effect cell)
        effect_td = None
        for td in tds:
            if td.get('rowspan') == '2':
                effect_td = td
                break
        if enchantment_th and effect_td:
            link = enchantment_th.find('a')
            enchantment_name = link.get_text(strip=True)
            effect_text = effect_td.get_text(separator=' ', strip=True)
            effect_text = clean_text(effect_text)
            if effect_text and len(effect_text) > 10:
                upgrades.append({
                    'name': enchantment_name,
                    'effect': effect_text
                })
                print(f"DEBUG: Added {enchantment_name} with effect: {effect_text[:50]}...")
    return upgrades


def extract_soulbound_upgrade_effects(html_content: str) -> List[Dict[str, str]]:
    """
    Extract enchantment upgrade effects from soulbound items by gathering all <li> elements in the soulbound upgrades table.

    Args:
        html_content: The HTML content as a string

    Returns:
        List of dictionaries containing upgrade enchantment names and their effects
    """
    soup = BeautifulSoup(html_content, 'html.parser')
    upgrades: List[Dict[str, str]] = []

    # Find the soulbound upgrades table
    soulbound_table = soup.find('table', {'class': 'wikitable sortable'})
    if not soulbound_table:
        return upgrades

    # Get all <li> elements in the soulbound table
    li_elements = soulbound_table.find_all('li')

    for li in li_elements:
        li_text = clean_text(li.get_text())
        if not li_text:
            continue

        # Try to get the enchantment name from the <a> tag
        link = li.find('a')
        if link:
            name = clean_text(link.get_text())
        else:
            # Use first 30 characters as name if no link
            name = li_text[:30] + "..." if len(li_text) > 30 else li_text

        upgrades.append({
            'name': name,
            'effect': li_text
        })

    return upgrades


def extract_current_enchantments(html_content: str) -> List[Dict[str, str]]:
    """
    Extract current enchantments from the enchantments section (all <li> in curr_enchantments section).

    Args:
        html_content: The HTML content as a string

    Returns:
        List of dictionaries containing current enchantment names and their effects
    """
    soup = BeautifulSoup(html_content, 'html.parser')
    enchantments: List[Dict[str, str]] = []

    # Find all curr_enchantments sections (there may be more than one)
    curr_enchantments_divs = soup.find_all('div', {'data-source': 'curr_enchantments'})
    for enchantments_data in curr_enchantments_divs:
        # Find all <li> elements in the section
        li_elements = enchantments_data.find_all('li')
        for li in li_elements:
            li_text = clean_text(li.get_text())
            if not li_text:
                continue
            link = li.find('a')
            if link:
                name = clean_text(link.get_text())
            else:
                name = li_text[:30] + "..." if len(li_text) > 30 else li_text
            enchantments.append({
                'name': name,
                'effect': li_text
            })
    return enchantments


def extract_enchantment_effects(html_content: str) -> Dict[str, List[Dict[str, str]]]:
    """
    Main function to extract enchantment effects from HTML content.
    Detects whether the item is soulbound and uses appropriate extraction method.

    Args:
        html_content: The HTML content as a string

    Returns:
        Dictionary containing 'current' and 'upgrades' enchantments
    """
    current_enchantments = extract_current_enchantments(html_content)

    if is_soulbound_item(html_content):
        print("DEBUG: Detected soulbound item, using soulbound extraction method")
        upgrade_enchantments = extract_soulbound_upgrade_effects(html_content)
    else:
        print("DEBUG: Using regular item extraction method")
        upgrade_enchantments = extract_regular_upgrade_effects(html_content)

    return {
        'current': current_enchantments,
        'upgrades': upgrade_enchantments
    }


def main():
    """Main entry point for the script."""
    if len(sys.argv) != 2:
        print("Usage: python extract_enchantment_effects.py <html_file>")
        print("Example: python extract_enchantment_effects.py examples/item-html-example.html")
        sys.exit(1)

    html_file = sys.argv[1]

    try:
        with open(html_file, 'r', encoding='utf-8') as f:
            html_content = f.read()
    except FileNotFoundError:
        print(f"Error: File '{html_file}' not found.")
        sys.exit(1)
    except Exception as e:
        print(f"Error reading file: {e}")
        sys.exit(1)

    # Extract enchantments
    results = extract_enchantment_effects(html_content)

    # Display results
    print("CURRENT ENCHANTMENTS:")
    print("=" * 50)
    for i, enchantment in enumerate(results['current'], 1):
        print(f"{i}. {enchantment['name']}")
        print(f"   Effect: {enchantment['effect']}")
        print()

    print("UPGRADE ENCHANTMENTS:")
    print("=" * 50)
    for i, enchantment in enumerate(results['upgrades'], 1):
        print(f"{i}. {enchantment['name']}")
        print(f"   Effect: {enchantment['effect']}")
        print()


if __name__ == "__main__":
    main()
