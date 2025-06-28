-- Seed the database with tags from tags.md
-- This script inserts all tags with their semantic explanations

-- Class tags (0-10)
INSERT INTO tags (name, description) VALUES
('barbarian', 'Related to Barbarian class abilities and features'),
('chanter', 'Related to Chanter class abilities and features'),
('cipher', 'Related to Cipher class abilities and features'),
('druid', 'Related to Druid class abilities and features'),
('fighter', 'Related to Fighter class abilities and features'),
('monk', 'Related to Monk class abilities and features'),
('paladin', 'Related to Paladin class abilities and features'),
('priest', 'Related to Priest class abilities and features'),
('ranger', 'Related to Ranger class abilities and features'),
('rogue', 'Related to Rogue class abilities and features'),
('wizard', 'Related to Wizard class abilities and features');

-- Magic school tags (11-27)
INSERT INTO tags (name, description) VALUES
('evocation', 'Evocation magic - destructive and elemental spells'),
('veil_piercing', 'Abilities that can pierce through magical veils or barriers'),
('interrupt', 'Abilities that can interrupt enemy actions or spellcasting'),
('slashing', 'Slashing damage type'),
('piercing', 'Piercing damage type'),
('crashing', 'Crashing damage type'),
('shock', 'Shock damage type'),
('burn', 'Burn damage type'),
('freeze', 'Freeze damage type'),
('corrode', 'Corrode damage type'),
('raw', 'Raw damage type'),
('illusion', 'Illusion magic - deceptive and mind-affecting spells'),
('conjuration', 'Conjuration magic - summoning and creation spells'),
('enchanting', 'Enchanting magic - mind control and charm spells'),
('transmutation', 'Transmutation magic - transformation and alteration spells');

-- Affliction tags (28-33)
INSERT INTO tags (name, description) VALUES
('constitution_affliction', 'Negative effects that reduce constitution'),
('dexterity_affliction', 'Negative effects that reduce dexterity'),
('might_affliction', 'Negative effects that reduce might'),
('intellect_affliction', 'Negative effects that reduce intellect'),
('perception_affliction', 'Negative effects that reduce perception'),
('resolve_affliction', 'Negative effects that reduce resolve');

-- Inspiration tags (34-39)
INSERT INTO tags (name, description) VALUES
('constitution_inspiration', 'Positive effects that boost constitution'),
('dexterity_inspiration', 'Positive effects that boost dexterity'),
('might_inspiration', 'Positive effects that boost might'),
('intellect_inspiration', 'Positive effects that boost intellect'),
('perception_inspiration', 'Positive effects that boost perception'),
('resolve_inspiration', 'Positive effects that boost resolve');

-- Status effect tags (40-76)
INSERT INTO tags (name, description) VALUES
('sickened', 'Status effect that reduces constitution'),
('weakened', 'Status effect that reduces might'),
('enfeebled', 'Status effect that reduces overall strength'),
('fit', 'Status effect that boosts constitution'),
('hardy', 'Status effect that provides constitution resistance'),
('robust', 'Status effect that provides health and endurance'),
('hobbled', 'Status effect that reduces movement speed'),
('immobilized', 'Status effect that prevents movement'),
('paralyzed', 'Status effect that prevents all actions'),
('petrified', 'Status effect that turns target to stone'),
('quick', 'Status effect that increases action speed'),
('nimble', 'Status effect that increases dexterity'),
('swift', 'Status effect that increases movement speed'),
('staggered', 'Status effect that reduces accuracy'),
('dazed', 'Status effect that reduces perception'),
('stunned', 'Status effect that prevents actions temporarily'),
('strong', 'Status effect that increases might'),
('tenacious', 'Status effect that provides resistance to afflictions'),
('energized', 'Status effect that provides energy or power'),
('confused', 'Status effect that causes random actions'),
('charmed', 'Status effect that makes target friendly'),
('dominated', 'Status effect that gives control over target'),
('smart', 'Status effect that increases intellect'),
('acute', 'Status effect that increases perception'),
('brilliant', 'Status effect that provides intellect bonuses'),
('distracted', 'Status effect that reduces perception'),
('disoriented', 'Status effect that reduces perception'),
('blinded', 'Status effect that reduces perception'),
('insightful', 'Status effect that increases perception'),
('aware', 'Status effect that increases awareness'),
('intuitive', 'Status effect that provides perception bonuses'),
('shaken', 'Status effect that reduces resolve'),
('frightened', 'Status effect that causes fear'),
('terrified', 'Status effect that causes extreme fear'),
('steadfast', 'Status effect that provides resolve resistance'),
('resolute', 'Status effect that increases resolve'),
('courageous', 'Status effect that provides fear resistance');

-- Targeting tags (77-80)
INSERT INTO tags (name, description) VALUES
('targets_deflection', 'Abilities that target deflection defense'),
('targets_reflex', 'Abilities that target reflex defense'),
('targets_fortitude', 'Abilities that target fortitude defense'),
('targets_will', 'Abilities that target will defense');

-- Summoning and engagement tags (81-83)
INSERT INTO tags (name, description) VALUES
('engagement_slots', 'Abilities that affect engagement mechanics'),
('summon_weapon', 'Abilities that summon weapons'),
('summon_creature', 'Abilities that summon creatures');

-- Modification tags (84-89)
INSERT INTO tags (name, description) VALUES
('mod_accuracy', 'Modifies accuracy values'),
('mod_deflection', 'Modifies deflection defense'),
('mod_reflex', 'Modifies reflex defense'),
('mod_fortitude', 'Modifies fortitude defense'),
('mod_will', 'Modifies will defense'),
('mod_move_speed', 'Modifies movement speed');

-- Status and armor tags (90-99)
INSERT INTO tags (name, description) VALUES
('prone', 'Status effect that knocks target to ground'),
('mod_armour', 'Modifies armor values'),
('slashing_armour', 'Armor specifically against slashing damage'),
('piercing_armour', 'Armor specifically against piercing damage'),
('crashing_armour', 'Armor specifically against crashing damage'),
('shock_armour', 'Armor specifically against shock damage'),
('burn_armour', 'Armor specifically against burn damage'),
('freeze_armour', 'Armor specifically against freeze damage'),
('corrode_armour', 'Armor specifically against corrode damage');

-- Hit conversion tags (100-103)
INSERT INTO tags (name, description) VALUES
('graze_to_hit', 'Converts graze results to hits'),
('hit_to_crit', 'Converts hit results to critical hits'),
('hit_to_graze', 'Converts hit results to grazes'),
('crit_to_hit', 'Converts critical hit results to hits');

-- Movement and action tags (104-105)
INSERT INTO tags (name, description) VALUES
('mod_stride', 'Modifies stride distance'),
('mod_action_speed', 'Modifies action speed');

-- Healing tags (106-107)
INSERT INTO tags (name, description) VALUES
('restore_health', 'Restores health points'),
('mod_restore_health', 'Modifies health restoration');

-- Special effect tags (108-114)
INSERT INTO tags (name, description) VALUES
('duplicate', 'Creates duplicates or copies'),
('mod_spell_reflect', 'Modifies spell reflection'),
('spell_steal', 'Steals or copies enemy spells'),
('transform', 'Transforms the target'),
('mod_effects_duration', 'Modifies duration of effects'),
('mod_healing_received', 'Modifies healing received'),
('untargetable', 'Makes target untargetable');

-- Immunity tags (115-120)
INSERT INTO tags (name, description) VALUES
('might_immunity', 'Provides immunity to might afflictions'),
('constitution_immunity', 'Provides immunity to constitution afflictions'),
('dexterity_immunity', 'Provides immunity to dexterity afflictions'),
('intellect_immunity', 'Provides immunity to intellect afflictions'),
('resolve_immunity', 'Provides immunity to resolve afflictions'),
('perception_immunity', 'Provides immunity to perception afflictions');

-- Power and weapon tags (121-125)
INSERT INTO tags (name, description) VALUES
('mod_power_level', 'Modifies power level'),
('mod_weapon_sets', 'Modifies weapon set capabilities'),
('mod_recovery_time', 'Modifies recovery time'),
('mod_reload_time', 'Modifies reload time'),
('mod_penetration', 'Modifies penetration values');

-- Special and range tags (126-127)
INSERT INTO tags (name, description) VALUES
('special', 'Special or unique effects not covered by other tags'),
('mod_ability_range', 'Modifies ability range');

-- Health and resource tags (128-129)
INSERT INTO tags (name, description) VALUES
('mod_max_health', 'Modifies maximum health'),
('mod_empower_points', 'Modifies empower points'),
('concentration', 'Allows to ignore one interrupt');

-- Spellcasting and attribute tags (130-136)
INSERT INTO tags (name, description) VALUES
('spellcasting_disabled', 'Disables spellcasting abilities'),
('mod_might', 'Modifies might attribute'),
('mod_constitution', 'Modifies constitution attribute'),
('mod_dexterity', 'Modifies dexterity attribute'),
('mod_intellect', 'Modifies intellect attribute'),
('mod_perception', 'Modifies perception attribute'),
('mod_resolve', 'Modifies resolve attribute');

-- Visibility and resistance tags (137-143)
INSERT INTO tags (name, description) VALUES
('invisible', 'Makes target invisible'),
('resistance_might', 'Provides resistance to might effects'),
('resistance_constitution', 'Provides resistance to constitution effects'),
('resistance_dexterity', 'Provides resistance to dexterity effects'),
('resistance_intellect', 'Provides resistance to intellect effects'),
('resistance_perception', 'Provides resistance to perception effects'),
('resistance_resolve', 'Provides resistance to resolve effects');

-- Combat effect tags (144-152)
INSERT INTO tags (name, description) VALUES
('mod_damage', 'Modifies damage output'),
('additional_attacks', 'Provides additional attack opportunities'),
('on_hit_dot', 'Damage over time effect on hit'),
('pull', 'Pulls target toward caster'),
('immunity_pull', 'Provides immunity to pull effects'),
('immunity_paralyzed', 'Provides immunity to paralysis'),
('mod_ranged_accuracy', 'Modifies ranged weapon accuracy'),
('counterattack', 'Provides counterattack abilities'),
('lash', 'Additional damage effect on attacks');
