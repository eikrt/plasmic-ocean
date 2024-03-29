use crate::graphics_utils::{Camera, MoveDirection};
use crate::world_structs::{
    ActionType, CategoryType, EntityType, ItemType, ReligionType, TaskType,
};
use std::collections::HashMap;
const METEOROID_TIME: u128 = 200;
const METEOROID_COST: i32 = 20;
const RAFT_COST: i32 = 50;
const SIPHON_COST: i32 = 10;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShootData {
    pub shooting: bool,
    pub mx: i32,
    pub my: i32,
    pub action_type: PlayerAction,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub energy: i32,
    pub xp: i32,
    pub lp: i32,
    pub level: i32,
    pub speed: f32,
    pub dir: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub stopped: bool,
    pub id: i32,
    pub faction: String,
    pub faction_id: i32,
    pub backpack_amount: u8,
    pub time: u128,
    pub shoot_change_1: u128,
    pub shoot_data: ShootData,
}
impl Player {
    pub fn mov(&mut self, dir: MoveDirection, delta: u128) {
        if dir == MoveDirection::Up {
            self.y -= self.speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Left {
            self.x -= self.speed * delta as f32 / 100.0;
            self.dir = std::f64::consts::PI as f32 * (2.0);
        } else if dir == MoveDirection::Down {
            self.y += self.speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Right {
            self.x += self.speed * delta as f32 / 100.0;
            self.dir = std::f64::consts::PI as f32 / 2.0;
        }
        if dir == MoveDirection::Nothing {
            self.stopped = true;
        } else {
            self.stopped = false;
        }
    }
    pub fn tick(&mut self, delta: u128) {
        self.time += 10;
        self.shoot_change_1 += delta;
        if self.xp >= 100 {
            self.level += 1;
            self.lp += 2;
            self.xp = 0;
        }
    }
    pub fn get_relative_x(&self, camera: &Camera) -> f32 {
        return self.x - camera.x;
    }
    pub fn get_relative_y(&self, camera: &Camera) -> f32 {
        return self.y - camera.y;
    }
    pub fn shoot_meteoroid(&mut self, x: i32, y: i32) {
        if self.energy - METEOROID_COST >= 0 && self.shoot_change_1 > METEOROID_TIME {
            self.shoot_data.shooting = true;
            self.shoot_change_1 = 0;
            self.energy -= METEOROID_COST;
        }
    }
    pub fn build_raft(&mut self, x: i32, y: i32) {
        if self.energy - RAFT_COST >= 0 && self.shoot_change_1 > METEOROID_TIME {
            self.shoot_data.shooting = true;
            self.shoot_change_1 = 0;
            self.energy -= RAFT_COST;
        }
    }
    pub fn build_soul_trap(&mut self, x: i32, y: i32) {
        if self.shoot_change_1 > METEOROID_TIME {
            self.shoot_data.shooting = true;
            self.shoot_change_1 = 0;
            self.hp -= SIPHON_COST;
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientPacket {
    pub camera: Camera,
    pub player: Player,
    pub faction_relations: HashMap<String, i32>,
}
#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum PlayerAction {
    Meteoroid,
    Raft,
    Siphon,
    Nothing,
}
