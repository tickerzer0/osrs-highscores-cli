use std::thread;

use serde::Deserialize;

use super::skill::{Skill, SKILL_ORDER, get_level_progress};

#[derive(Clone, Deserialize, Debug)]
pub struct SkillData {
    name: Skill,
    level: u32,
    xp: u32,
}

fn format_row(cols: Vec<&SkillData>) -> String {
    let mut formatted = vec![];
    for col in cols {
        formatted.push(format!("{col}"))
    }

    let lines: Vec<Vec<&str>> = formatted.iter().map(|col| col.lines().collect()).collect();
    let mut joined = vec![];
    for i in 0..5 {
        joined.push(format!("{}{}{}", lines[0][i], lines[1][i], lines[2][i]));
    }
    joined.join("\n")
}


impl std::fmt::Display for SkillData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{}", self.name);
        let level = format!("{}", self.level);
        let progress = format!("{}%", get_level_progress(self.xp, self.level));
        write!(
            f,"+------------------+
| {:<16} |\n\
| Lv {:<13} |\n\
| {:<16} |\n\
+------------------+",
            name, level, progress
        )
    }
}

#[derive(Deserialize)]
pub struct Player {
    pub skills: Vec<SkillData>
}


impl Player {
    pub fn get_skill(&self, skill_name: &Skill) -> &SkillData {
        self.skills.iter().find(|current_skill| current_skill.name == *skill_name).unwrap()
    }
}

impl std::fmt::Display for Player {
    // Print all skills in the order they are displayed in game. Rows, then columns.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let skill_rows: Vec<String> = thread::scope(|s| {
            let mut handles = vec![];

            for row in SKILL_ORDER.chunks(3) {
                let handle = s.spawn(|| {
                    let cols = row.iter().map(|c| self.get_skill(c)).collect();
                    format_row(cols)
                });
                handles.push(handle);
            }
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        for row in skill_rows {
            writeln!(f, "{row}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::{SkillData, Player};
    use super::Skill;

    fn get_dummy_player() -> Player {
        let attack = SkillData {
            id: 1,
            name: Skill::Attack,
            rank: 1234,
            level: 15,
            xp: 12345
        };

        let thieving = SkillData {
            id: 15,
            name: Skill::Thieving,
            rank: 5678,
            level: 99,
            xp: 13000000
        };
        
        let player_skills: Vec<SkillData> = vec![attack, thieving];
        Player { skills: player_skills }
    }

    #[test]
    fn test_get_skill_pass() {
        let player = get_dummy_player();
        
        assert_eq!(player.get_skill(&Skill::Attack).rank, 1234);
        assert_eq!(player.get_skill(&Skill::Thieving).xp, 13000000);
    }
}