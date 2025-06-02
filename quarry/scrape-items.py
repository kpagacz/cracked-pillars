import requests
from bs4 import BeautifulSoup
import csv
import time

def scrape_all_items():
    base_url = "https://pillarsofeternity.fandom.com"
    category_url = "https://pillarsofeternity.fandom.com/wiki/Category:Pillars_of_Eternity_II:_Deadfire_items"
    all_items = []
    next_page_url = category_url
    page_count = 1
    
    print("Starting item scraping...")
    
    while next_page_url:
        print(f"Scraping page {page_count}: {next_page_url}")
        try:
            # Fetch page with custom headers
            headers = {
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
            }
            response = requests.get(next_page_url, headers=headers)
            response.raise_for_status()
            
            soup = BeautifulSoup(response.text, 'html.parser')
            
            # Find all category groups (both subcategories and items)
            category_groups = soup.find_all('div', class_='mw-category-group')
            
            if not category_groups:
                print("  No category groups found")
                break
                
            # Process each category group
            items_found = 0
            for group in category_groups:
                # Skip subcategory sections
                group_header = group.find('h3')
                if group_header and group_header.get_text().strip() == "Subcategories":
                    print("  Skipping subcategories section")
                    continue
                    
                # Extract item links
                for link in group.select('li a'):
                    item_name = link.get_text().strip()
                    item_url = base_url + link['href']
                    all_items.append((item_name, item_url))
                    items_found += 1
            
            print(f"  Found {items_found} items on this page")
            
            # Find next page link
            next_link = soup.find('a', string='next page')
            next_page_url = base_url + next_link['href'] if next_link else None
            page_count += 1
            
            # Add delay to be polite to the server
            time.sleep(1)
            
        except Exception as e:
            print(f"  Error encountered: {e}")
            break

    print(f"\nTotal items scraped: {len(all_items)}")
    return all_items

def save_to_csv(items, filename):
    with open(filename, 'w', newline='', encoding='utf-8') as f:
        writer = csv.writer(f)
        writer.writerow(['Item Name', 'URL'])
        writer.writerows(items)
    print(f"Saved to {filename}")

if __name__ == "__main__":
    items = scrape_all_items()
    save_to_csv(items, 'deadfire_items.csv')
    print("Scraping complete!")
