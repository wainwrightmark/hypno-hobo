use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CharacterStats {
    pub strength: AbilityScore,
    pub dexterity: AbilityScore,
    pub constitution: AbilityScore,
    pub wisdom: AbilityScore,
    pub intelligence: AbilityScore,
    pub charisma: AbilityScore,
}

impl CharacterStats {
    pub fn is_legal(&self) -> bool {
        self.points_remaining() == 0
    }

    pub fn points_remaining(&self) -> isize {
        let sum: isize = self.all().into_iter().map(|x| x.1 .0 as isize).sum();
        70 - sum
    }

    pub fn all(&self) -> [(Ability, AbilityScore); 6] {
        use Ability::*;
        [
            (Strength, self.strength),
            (Dexterity, self.dexterity),
            (Constitution, self.constitution),
            (Wisdom, self.wisdom),
            (Intelligence, self.intelligence),
            (Charisma, self.charisma),
        ]
    }
}

impl Index<Ability> for CharacterStats {
    type Output = AbilityScore;

    fn index(&self, index: Ability) -> &Self::Output {
        use Ability::*;
        match index {
            Strength => &self.strength,
            Dexterity => &self.dexterity,
            Constitution => &self.constitution,
            Wisdom => &self.wisdom,
            Intelligence => &self.intelligence,
            Charisma => &self.charisma,
        }
    }
}

impl IndexMut<Ability> for CharacterStats {
    fn index_mut(&mut self, index: Ability) -> &mut Self::Output {
        use Ability::*;
        match index {
            Strength => &mut self.strength,
            Dexterity => &mut self.dexterity,
            Constitution => &mut self.constitution,
            Wisdom => &mut self.wisdom,
            Intelligence => &mut self.intelligence,
            Charisma => &mut self.charisma,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AbilityScore(pub u8);

impl std::fmt::Display for AbilityScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for AbilityScore {
    fn default() -> Self {
        Self(10)
    }
}
