import requests
from bs4 import BeautifulSoup
import csv

def scrape_wiki_category(category_url, category_name):
    base_url = "https://pillarsofeternity.fandom.com"
    abilities = []
    next_page_url = category_url
    page_count = 1

    print(f"\nScraping {category_name} abilities...")
    
    while next_page_url:
        print(f"  Page {page_count}: {next_page_url}")
        try:
            response = requests.get(next_page_url)
            response.raise_for_status()
            soup = BeautifulSoup(response.text, 'html.parser')
            
            # Find the category listing
            category_div = soup.find('div', {'class': 'mw-category'})
            if not category_div:
                print("  No abilities found on page")
                break
                
            # Extract ability links
            new_abilities = []
            for link in category_div.select('li a'):
                ability_name = link.text.strip()
                ability_url = base_url + link['href']
                new_abilities.append((ability_name, ability_url))
            
            abilities.extend(new_abilities)
            print(f"  Found {len(new_abilities)} abilities")
            
            # Find next page link
            next_link = soup.find('a', text='next page') 
            next_page_url = base_url + next_link['href'] if next_link else None
            page_count += 1
            
        except Exception as e:
            print(f"  Error scraping page: {e}")
            break

    print(f"Total {category_name} abilities scraped: {len(abilities)}")
    return abilities

def save_to_csv(abilities, filename):
    with open(filename, 'w', newline='', encoding='utf-8') as f:
        writer = csv.writer(f)
        writer.writerow(['Ability Name', 'URL'])
        writer.writerows(abilities)
    print(f"Saved to {filename}")

if __name__ == "__main__":
    # Active abilities
    active_url = "https://pillarsofeternity.fandom.com/wiki/Category:Pillars_of_Eternity_II:_Deadfire_active_abilities"
    active_abilities = scrape_wiki_category(active_url, "active")
    save_to_csv(active_abilities, 'deadfire_active_abilities.csv')
    
    # Passive abilities
    passive_url = "https://pillarsofeternity.fandom.com/wiki/Category:Pillars_of_Eternity_II:_Deadfire_passive_abilities"
    passive_abilities = scrape_wiki_category(passive_url, "passive")
    save_to_csv(passive_abilities, 'deadfire_passive_abilities.csv')

    # Modal abilities
    modal_url = "https://pillarsofeternity.fandom.com/wiki/Category:Pillars_of_Eternity_II:_Deadfire_modal_abilities"
    modal_abilities = scrape_wiki_category(modal_url, "modal")
    save_to_csv(modal_abilities, 'deadfire_modal_abilities.csv')
    
    print("\nScraping complete!")
