// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }
        let v = if self.level < 10{
            None
        } else {
            Some(100)
        };

        return Some(Player {
            health: 100,
            mana: v,
            level: self.level
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None {
            if self.health >= mana_cost {
                self.health -= mana_cost;
                return 0;
            } else {
                self.health = 0;
                return 0;
            }
        }
        let mana = self.mana.unwrap();  
        if mana >= mana_cost {
            self.mana = Some(mana - mana_cost);
            return mana_cost * 2;
        } else {
            return 0;
        }
    }
}
