use serde::Deserialize;

use super::skill::Skill;

#[derive(Clone, Deserialize, Debug)]
pub struct SkillData {
    id: u8,
    name: Skill,
    rank: i32,
    level: u32,
    xp: u32
} 

#[derive(Deserialize)]
pub struct Player {
    pub skills: Vec<SkillData>
}


impl Player {
    pub fn get_skill(&self, skill_name: Skill) -> Option<&SkillData> {
        self.skills.iter().find(|current_skill| current_skill.name == skill_name)
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for skill in &self.skills {
            writeln!(f, "{:?}", skill);
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
        
        assert_eq!(player.get_skill(Skill::Attack).unwrap().rank, 1234);
        assert_eq!(player.get_skill(Skill::Thieving).unwrap().xp, 13000000);
    }

    #[test]
        fn test_get_skill_fail() {
            let player = get_dummy_player();

            let skill = player.get_skill(Skill::Construction);
            assert!(skill.is_none());
    }
}