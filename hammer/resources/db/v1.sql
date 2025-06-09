-- Create tables with CHECK constraints instead of ENUMs
CREATE TABLE abilities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    description TEXT NOT NULL,
    activation TEXT NOT NULL CHECK (activation IN ('passive', 'active', 'modal')),
    area_of_effect TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Notes table (one-to-many relationship with abilities)
CREATE TABLE ability_notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ability_id INTEGER NOT NULL,
    note TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ability_id) REFERENCES abilities(id) ON DELETE CASCADE
);

-- Effects table (one-to-many relationship with abilities)
CREATE TABLE effects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ability_id INTEGER NOT NULL,
    condition TEXT,
    value TEXT NOT NULL,
    duration INTEGER, -- Duration in seconds
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ability_id) REFERENCES abilities(id) ON DELETE CASCADE
);

-- Effect targets (many-to-many relationship between effects and targets)
CREATE TABLE effect_targets (
    effect_id INTEGER NOT NULL,
    target TEXT NOT NULL CHECK (target IN (
        'self', 'allied_aoe', 'hazard_aoe', 'foe_aoe', 'foe_target',
        'attackers', 'target', 'jump_targets', 'summon', 'target+beam', 'friendly_target'
    )),
    PRIMARY KEY (effect_id, target),
    FOREIGN KEY (effect_id) REFERENCES effects(id) ON DELETE CASCADE
);

-- Effect tags (many-to-many relationship between effects and tags)
CREATE TABLE effect_tags (
    effect_id INTEGER NOT NULL,
    tag TEXT NOT NULL CHECK (tag IN (
        -- Class tags
        'barbarian', 'chanter', 'cipher', 'druid', 'fighter', 'monk',
        'paladin', 'priest', 'ranger', 'rogue', 'wizard',
        -- School tags
        'evocation', 'illusion', 'conjuration', 'enchanting', 'transmutation',
        -- Damage type tags
        'slashing', 'piercing', 'crashing', 'shock', 'burn', 'freeze', 'corrode', 'raw',
        -- Affliction tags
        'constitution_affliction', 'dexterity_affliction', 'might_affliction',
        'intellect_affliction', 'perception_affliction', 'resolve_affliction',
        -- Inspiration tags
        'constitution_inspiration', 'dexterity_inspiration', 'might_inspiration',
        'intellect_inspiration', 'perception_inspiration', 'resolve_inspiration',
        -- Status effect tags
        'sickened', 'weakened', 'enfeebled', 'fit', 'hardy', 'robust',
        'hobbled', 'immobilized', 'paralyzed', 'petrified',
        'quick', 'nimble', 'swift',
        'staggered', 'dazed', 'stunned',
        'strong', 'tenacious', 'energized',
        'confused', 'charmed', 'dominated',
        'smart', 'acute', 'brilliant',
        'distracted', 'disoriented', 'blinded',
        'insightful', 'aware', 'intuitive',
        'shaken', 'frightened', 'terrified',
        'steadfast', 'resolute', 'courageous',
        -- Target defence tags
        'targets_deflection', 'targets_reflex', 'targets_fortitude', 'targets_will',
        -- Resistance tags
        'resistance_might', 'resistance_constitution', 'resistance_dexterity',
        'resistance_intellect', 'resistance_perception', 'resistance_resolve',
        -- Immunity tags
        'might_immunity', 'constitution_immunity', 'dexterity_immunity',
        'intellect_immunity', 'resolve_immunity', 'perception_immunity',
        -- Summon tags
        'summon_weapon', 'summon_creature',
        -- Armour tags
        'mod_armour', 'slashing_armour', 'piercing_armour', 'crashing_armour',
        'shock_armour', 'burn_armour', 'freeze_armour', 'corrode_armour',
        -- Stats tags
        'mod_might', 'mod_constitution', 'mod_dexterity',
        'mod_intellect', 'mod_perception', 'mod_resolve',
        -- Hit conversion tags
        'graze_to_hit', 'hit_to_crit', 'hit_to_graze', 'crit_to_hit',
        -- Defenses tags
        'mod_deflection', 'mod_reflex', 'mod_fortitude', 'mod_will',
        -- Miscellaneous tags
        'engagement_slots', 'mod_move_speed', 'prone', 'concentration', 'mod_accuracy',
        'mod_stride', 'mod_action_speed', 'restore_health', 'mod_restore_health',
        'duplicate', 'mod_spell_reflect', 'spell_steal', 'transform',
        'mod_effects_duration', 'mod_healing_received', 'untargetable',
        'mod_power_level', 'mod_weapon_sets', 'mod_recovery_time', 'mod_reload_time',
        'mod_penetration', 'special', 'mod_ability_range', 'mod_max_health',
        'mod_empower_points', 'spellcasting_disabled', 'invisible', 'mod_damage',
        'veil_piercing', 'interrupt'
    )),
    PRIMARY KEY (effect_id, tag),
    FOREIGN KEY (effect_id) REFERENCES effects(id) ON DELETE CASCADE
);

-- Origin table (one-to-one relationship with abilities)
CREATE TABLE ability_origins (
    ability_id INTEGER PRIMARY KEY,
    type TEXT NOT NULL CHECK (type IN ('progression', 'item')),
    class TEXT, -- Only for progression type
    learn_level TEXT, -- Can be integer or string like 'At character creation'
    level INTEGER, -- Only for progression type
    item_name TEXT, -- Only for item type
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ability_id) REFERENCES abilities(id) ON DELETE CASCADE,
    CHECK (
        (type = 'progression' AND class IS NOT NULL AND learn_level IS NOT NULL AND level IS NOT NULL AND item_name IS NULL) OR
        (type = 'item' AND item_name IS NOT NULL AND class IS NULL AND learn_level IS NULL AND level IS NULL)
    )
);

-- Keywords table (many-to-many relationship with abilities)
CREATE TABLE keywords (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    keyword TEXT NOT NULL UNIQUE
);

CREATE TABLE ability_keywords (
    ability_id INTEGER NOT NULL,
    keyword_id INTEGER NOT NULL,
    PRIMARY KEY (ability_id, keyword_id),
    FOREIGN KEY (ability_id) REFERENCES abilities(id) ON DELETE CASCADE,
    FOREIGN KEY (keyword_id) REFERENCES keywords(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX idx_abilities_name ON abilities(name);
CREATE INDEX idx_abilities_activation ON abilities(activation);
CREATE INDEX idx_effects_ability_id ON effects(ability_id);
CREATE INDEX idx_ability_origins_type ON ability_origins(type);
CREATE INDEX idx_ability_origins_class ON ability_origins(class);
CREATE INDEX idx_ability_origins_item_name ON ability_origins(item_name);
CREATE INDEX idx_keywords_keyword ON keywords(keyword);

-- Create a trigger to update the updated_at timestamp
CREATE TRIGGER update_abilities_timestamp
AFTER UPDATE ON abilities
BEGIN
    UPDATE abilities SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
