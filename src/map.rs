use crate::casting::{player::Player, types::Vector};
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

pub struct Map<'a> {
    pub mapping: HashMap<(u16, u16), (f64, f64, u8)>,
    pub player: &'a mut Player,
}

pub fn get_world_map<'a>(player: &'a mut Player) -> Map {
    let vec: Vec<u8> = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3,
        0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 4, 4, 4, 4,
        4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1,
    ];
    let mut map = HashMap::new();
    vec.chunks(24).enumerate().for_each(|(j, y)| {
        y.iter().enumerate().for_each(|(i, x)| {
            let tuple = (i as f64 + 1.0 / 2.0, j as f64 + 1.0 / 2.0, *x);
            map.insert((i as u16, j as u16), tuple);
        })
    });
    Map {
        mapping: map,
        player,
    }
}

impl<'a> Display for Map<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut acc = String::from("");
        let mut player_set = false;
        for j in 0u16..=23 {
            for i in 0u16..=23 {
                let location = self.mapping.get(&(i, j)).unwrap();
                if !player_set
                    && location.0 >= self.player.position.x
                    && location.1 >= self.player.position.y
                {
                    acc += "X";
                    player_set = true;
                } else {
                    acc += &location.2.to_string();
                }
            }
            acc += "\n"
        }
        write!(f, "{}", acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let mut player = Player::new(10.5, 20.5);
        let map = get_world_map(&mut player);
        assert_eq!(
            &(0.5f64, 0.5f64, 1u8),
            map.mapping.get(&(0u16, 0u16)).unwrap()
        );
        assert_eq!(
            &(23.5f64, 0.5f64, 1u8),
            map.mapping.get(&(23u16, 0u16)).unwrap()
        );
        assert_eq!(
            &(6.5f64, 18.5f64, 5u8),
            map.mapping.get(&(6u16, 18u16)).unwrap()
        );
    }

    #[test]
    fn test_display() {
        let mut player: Player = Player::new(10.5, 20.5);
        let map = get_world_map(&mut player);
        println!("{}", map);
        map.player.walk(Vector { x: 0.0, y: 1.0 });
        map.player.walk(Vector { x: 1.0, y: 0.0 });
        map.player.walk(Vector { x: 1.0, y: 0.0 });
        map.player.walk(Vector { x: 1.0, y: 0.0 });
        map.player.walk(Vector { x: 1.0, y: 0.0 });
        println!("{}", map);
    }
}
