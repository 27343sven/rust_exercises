#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health <= 0 {
            Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana_pool) => {
                if mana_cost <= mana_pool {
                    self.mana = Some(mana_pool - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
            None => {
                self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}

#[test]
fn cast_with_enough_mana() {
    let mut wizard = Player {
        health: 123,
        mana: Some(30),
        level: 18,
    };
    assert_eq!(wizard.cast_spell(10), 20);
    assert_eq!(wizard.health, 123);
    assert_eq!(wizard.mana, Some(20));
}
