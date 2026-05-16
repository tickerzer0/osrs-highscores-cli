use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub enum Skill {
    Overall,
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
    Sailing
}

pub const SKILL_ORDER: &[Skill] = &[
    Skill::Attack,       Skill::Hitpoints,    Skill::Mining,
    Skill::Strength,     Skill::Agility,      Skill::Smithing,
    Skill::Defence,      Skill::Herblore,     Skill::Fishing,
    Skill::Ranged,       Skill::Thieving,     Skill::Cooking,
    Skill::Prayer,       Skill::Crafting,     Skill::Firemaking,
    Skill::Magic,        Skill::Fletching,    Skill::Woodcutting,
    Skill::Runecraft,    Skill::Slayer,       Skill::Farming,
    Skill::Construction, Skill::Hunter,       Skill::Sailing,
];

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// This function will calculate how much xp is required to reach a given level
// https://oldschool.runescape.wiki/w/Experience
fn get_xp_for_level(level: u32) -> u32 {
    if level <= 1 {
        return 0;
    }
    let l = level as f64;
    let two_one_seventh = 2f64.powf(1.0 / 7.0);
    let result = (1.0 / 8.0) * (l * l - l + 600.0 * (2f64.powf(l / 7.0) - two_one_seventh) / (two_one_seventh - 1.0));
    result.floor() as u32
}

pub fn get_level_progress(current_xp: u32, level: u32) -> u8 {
    let xp_floor = get_xp_for_level(level);

    if level == 99 {
        100
    } else {
        (((current_xp - xp_floor) * 100) / ((get_xp_for_level(level + 1) - xp_floor) as u32)) as u8
    }
}

