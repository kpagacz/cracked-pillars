-- Abilities
CREATE TABLE abilities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE abilities_tags (
  ability_id INTEGER NOT NULL,
  tag_name TEXT NOT NULL,
  FOREIGN KEY (ability_id) references abilities(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_name) references tags(name) ON DELETE CASCADE
);

-- Items
CREATE TABLE items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  slug TEXT NOT NULL UNIQUE,
  wiki_url TEXT NOT NULL,
  effects_description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE items_tags (
  item_id INTEGER NOT NULL,
  tag_name TEXT NOT NULL,
  FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_name) REFERENCES tags(name) ON DELETE CASCADE
);

-- Effect tags
CREATE TABLE tags (
    name TEXT PRIMARY KEY
);

-- Indexes
CREATE INDEX idx_abiltiies_slug ON abilities(slug);
CREATE INDEX idx_tags_name ON tags(name);
CREATE INDEX idx_items_name ON items(name);
CREATE INDEX idx_abilities_tags_ability_id ON abilities_tags(ability_id);
CREATE INDEX idx_items_tags_item_id ON items_tags(item_id);


-- Create a trigger to update the updated_at timestamp
CREATE TRIGGER update_abilities_timestamp
AFTER UPDATE ON abilities
BEGIN
    UPDATE abilities SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_items_timestamp
AFTER UPDATE ON items
BEGIN
   UPDATE items SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
