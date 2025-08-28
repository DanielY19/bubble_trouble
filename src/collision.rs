use crate::{entities::*,WINDOW_WIDTH};
use coffee::graphics::Rectangle;

pub enum Collision {
    None,
    Left,
    Right,
    Top,
    Bottom,
}

pub struct CollisionSystem;

impl CollisionSystem{

    pub fn check_collision(lhs: &Rectangle<f32>, rhs: &Rectangle<f32>) -> Collision {
        let overlap_x = lhs.x < rhs.x + rhs.width && lhs.x + lhs.width > rhs.x;
        let overlap_y = lhs.y < rhs.y + rhs.height && lhs.y + lhs.height > rhs.y;

        if overlap_x && overlap_y {
            let overlap_width = (lhs.x + lhs.width).min(rhs.x + rhs.width) 
                              - lhs.x.max(rhs.x);
            let overlap_height = (lhs.y + lhs.height).min(rhs.y + rhs.height) 
                               - lhs.y.max(rhs.y);

            match overlap_width < overlap_height {
                true    => if lhs.x < rhs.x { Collision::Left } else { Collision::Right }
                false   => if lhs.y < rhs.y { Collision::Top } else { Collision::Bottom }
            }
        } else {
            Collision::None
        }
    }

    pub fn player_platforms_collision(player: &mut Player, platforms: &Vec<Platform>) {
        if let Some(platform) = platforms.iter().find(|platform| {
            let collision = CollisionSystem::check_collision(&player.position, &platform.position);
            matches!(collision, Collision::Left | Collision::Right | Collision::Top | Collision::Bottom)
            }) {
                let collision = CollisionSystem::check_collision(&player.position, &platform.position);
                match collision {
                    Collision::Left | Collision::Right  => player.collide_with_platform_horizontal(platform, collision),
                    Collision::Top                      => player.collide_with_platform_top(platform),
                    Collision::Bottom                   => player.collide_with_platform_bottom(platform),
                    Collision::None                     => ()
                }
            }
        else {
            player.no_collision();
        }

    }

    pub fn player_bubbles_collision(player: &Player, bubbles: &Vec<Bubble>) -> bool {
        bubbles.iter().any(|bubble| {
             !matches!(CollisionSystem::check_collision(&player.position, &bubble.position), Collision::None)
        })
    }

    pub fn bubbles_platforms_collision(bubbles: &mut Vec<Bubble>, platforms: &Vec<Platform>) {
        bubbles.iter_mut().for_each(|bubble| {
            platforms.iter().enumerate().for_each(|(i,platform)| {
                match CollisionSystem::check_collision(&bubble.position, &platform.position) {
                    Collision::Left | Collision::Right => {bubble.bounce_off_wall();}
                    Collision::Top | Collision::Bottom => 
                    if i == 0 {bubble.bounce_off_ground();} 
                    else {bubble.bounce_off_ceiling();}
                    _ => ()
                }
            }); 
        });
    }

    pub fn bubbles_walls_collision(bubbles: &mut Vec<Bubble>) {
        bubbles.iter_mut().for_each(|bubble| {
            if bubble.position.x + bubble.radius <= bubble.radius || bubble.position.x + bubble.radius >= WINDOW_WIDTH - bubble.radius {
                bubble.bounce_off_wall();
            }
        }) 
    }

    pub fn bubbles_ceiling_collision(bubbles: &mut Vec<Bubble>) {
        bubbles.iter_mut().for_each(|bubble| {
            if bubble.position.y <= 0.0 {
                bubble.bounce_off_ceiling();
            }
        }) 
    }

    pub fn harpoon_bubbles_collision(harpoon: &mut Harpoon, bubbles: &mut Vec<Bubble>) {
        let mut spawned_bubbles = Vec::new();

        bubbles.retain(|bubble| {
            match CollisionSystem::check_collision(&harpoon.position, &bubble.position) {
                Collision::None => true,
                _  => if let HarpoonState::Active | HarpoonState::Stationary = harpoon.state {
                    harpoon.hit_bubble();
                    if let Some(mut child_bubbles) = bubble.pop() {
                        spawned_bubbles.append(&mut child_bubbles);
                    }
                    false 
                }
                else {
                    true
                }
            }
        });

        bubbles.append(&mut spawned_bubbles);
    }

    pub fn harpoon_platform_collision(harpoon: &mut Harpoon, platforms: &Vec<Platform>) {
        platforms.iter().for_each(|platform| {
            match CollisionSystem::check_collision(&harpoon.position, &platform.position) {
                Collision::Bottom => harpoon.hit_platform(),
               _ => (),
            }

        })
    }

    pub fn harpoon_ceiling_collision(harpoon: &mut Harpoon) {
        if harpoon.position.y <= 0.0 {
            harpoon.hit_platform();
        }
    }
}