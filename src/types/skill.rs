#[derive(Clone, PartialEq)]
pub enum Skill {
    Attack,
    Defence,
    Strength,
    Hitpoints,
    Ranged,
    Prayer,
    Magic,
    Cooking,
    Woodcutting,
    Fletching,
    Fishing,
    Firemaking,
    Crafting,
    Smithing,
    Mining,
    Herblore,
    Agility,
    Thieving,
    Slayer,
    Farming,
    Runecraft,
    Hunter,
    Construction,
}

// This function will calculate how much xp is required to reach a given level
// https://oldschool.runescape.wiki/w/Experience
fn get_xp_for_level(level: u32) -> u32 {
    (1 / 4) * (level - 1 + (300 * 2 ^ ((level - 1) / 7)))
}

pub fn get_level_progress(level: u8) -> u8 {
    if level == 99 {
        100
    } else {
        (get_xp_for_level(level as u32) / get_xp_for_level((level + 1) as u32) * 100) as u8
    }
}