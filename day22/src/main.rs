use std::cmp::{max, min};
use std::i32;

const BOSS_HP: i32 = 58;
const BOSS_DMG: i32 = 9;

fn fight(dmg_per_turn: i32) -> i32 {
    let mut stack = Vec::new();
    stack.push((true, BOSS_HP, 50_i32, 500_i32, [0; 3], 0_i32, 0_i32));
    let mut min_mana = i32::MAX;
    while let Some((my_turn, mut boss_hp, mut hp, mut mana, mut effects, mana_used, rnds)) = stack.pop() {
        if mana_used > min_mana || rnds > 20 {
            continue;
        }
        let mut def = 0;
        if effects[0] > 0 {
            def += 7;
            effects[0] -= 1;
        }
        if effects[1] > 0 {
            boss_hp -= 3;
            if boss_hp <= 0 {
                min_mana = min(min_mana, mana_used);
                continue;
            }
            effects[1] -= 1;
        }
        if effects[2] > 0 {
            mana += 101;
            effects[2] -= 1;
        }
        if !my_turn {
            hp -= max(1, BOSS_DMG - def);
            if hp <= 0 {
                continue;
            }
            stack.push((!my_turn, boss_hp, hp, mana, effects, mana_used, rnds + 1));
        } else {
            hp -= dmg_per_turn;
            if hp <= 0 {
                continue;
            }
            if mana >= 53 {  // Missile
                let new_boss_hp = boss_hp - 4;
                if new_boss_hp < 0 {
                    min_mana = min(min_mana, mana_used);
                } else {
                    stack.push((!my_turn, new_boss_hp, hp, mana - 53,
                                effects.clone(), mana_used + 53, rnds + 1));
                }
            }
            if mana >= 73 {  // Drain
                let new_boss_hp = boss_hp - 2;
                if new_boss_hp < 0 {
                    min_mana = min(min_mana, mana_used);
                } else {
                    stack.push((!my_turn, new_boss_hp, hp + 2, mana - 73,
                                effects.clone(), mana_used + 73, rnds + 1));
                }
            }
            for (i, &(cost, last)) in [(113, 6), (173, 6), (229, 5)].iter().enumerate() {
                if mana >= cost && effects[i] == 0 {
                    let mut effects = effects.clone();
                    effects[i] = last;
                    stack.push((!my_turn, boss_hp, hp, mana - cost, effects, mana_used + cost, rnds + 1));
                }
            }
        }
    }
    min_mana
}

fn main() {
    println!("Min mana for win: {}", fight(0));
    println!("Min mana for win with 1 dmg/turn: {}", fight(1));
}