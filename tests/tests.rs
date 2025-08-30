#[cfg(test)]
mod collision_tests {
    use bubble_trouble::WINDOW_WIDTH;
    #[cfg(test)]
    use bubble_trouble::{entities::*,collision::*};
    use coffee::graphics::{Rectangle};

    const EMPTY_RECTANGLE:Rectangle<u16> = Rectangle{x:0,y:0,width:0,height:0};

    #[test]
    fn test_player_platform_top_collision() {
        let mut player_on_top = Player::new(Rectangle{x:50.0,y:95.0,width:30.0,height:30.0},&EMPTY_RECTANGLE);
        let platform = Platform::new(Rectangle{x:50.0,y:120.0,width:100.0,height:30.0});

        CollisionSystem::player_platforms_collision(&mut player_on_top, &vec![platform]);
        assert!(player_on_top.on_ground);
    }

    #[test]
    fn test_player_platform_side_collision() {
        let mut player = Player::new(Rectangle{x:90.0,y: 90.0,width: 30.0,height: 30.0}, &EMPTY_RECTANGLE);
        let platform = Platform::new(Rectangle{x:120.0,y: 90.0,width: 30.0,height: 30.0});
        let platforms = vec![platform];

        CollisionSystem::player_platforms_collision(&mut player, &platforms);
        assert!(player.position.x + player.position.width <= platforms.first().unwrap().position.x);
    }

    #[test]
    fn test_player_platform_no_collision() {
        let mut player = Player::new(Rectangle{x:0.0,y: 0.0,width: 30.0,height: 30.0}, &EMPTY_RECTANGLE);
        let platform = Platform::new(Rectangle{x:200.0,y: 200.0,width: 100.0,height: 30.0});

        CollisionSystem::player_platforms_collision(&mut player, &vec![platform]);
        assert!(!player.on_ground);
    }

    #[test]
    fn test_player_bubble_collision() {
        let player = Player::new(Rectangle{x:50.0,y: 50.0,width: 30.0,height: 30.0}, &EMPTY_RECTANGLE);
        let bubble = Bubble::new(Rectangle{x:60.0,y: 60.0,width: 30.0,height: 30.0},15.0 ,(0.0,0.0));

        let collided = CollisionSystem::player_bubbles_collision(&player, &vec![bubble]);
        assert!(collided);
    }

    #[test]
    fn test_player_bubble_no_collision() {
        let player = Player::new(Rectangle{x:0.0,y: 0.0,width: 30.0,height: 30.}, &EMPTY_RECTANGLE);
        let bubble = Bubble::new(Rectangle{x:200.0,y:200.0,width:30.0,height: 30.0},15.0,(0.0,0.0));

        let collided = CollisionSystem::player_bubbles_collision(&player, &vec![bubble]);
        assert!(!collided);
    }

        #[test]
    fn test_bubble_bounces_off_wall() {
        let mut bubbles = vec![Bubble::new(Rectangle{x:0.0,y: 100.0,width: 20.0,height: 20.0},10.0,( -5.0, 0.0))];
        bubbles.push(Bubble::new(Rectangle{x:WINDOW_WIDTH,y: 100.0,width: 20.0,height: 20.0},10.0,( 5.0, 0.0)));
        CollisionSystem::bubbles_walls_collision(&mut bubbles);

        assert!(bubbles.first().unwrap().velocity.0 > 0.0);
        assert!(bubbles.last().unwrap().velocity.0 < 0.0);
    }

    #[test]
    fn test_bubble_bounces_off_ceiling() {
        let mut bubbles = vec![Bubble::new(Rectangle{x:50.0,y: 0.0,width: 20.0,height: 20.0},10.0,( 0.0, -5.0))];
        CollisionSystem::bubbles_ceiling_collision(&mut bubbles);

        assert!(bubbles.first().unwrap().velocity.1 > 0.0);
    }

    #[test]
    fn test_bubble_hits_platform_top() {
        let platform = Platform::new(Rectangle{x:50.0,y: 100.0,width: 100.0,height: 20.0});
        let mut bubbles = vec![Bubble::new(Rectangle{x:60.0,y: 95.0,width: 20.0,height: 20.0},10.0,( 0.0, 5.0))];

        CollisionSystem::bubbles_platforms_collision(&mut bubbles, &vec![platform]);

        assert!(bubbles.first().unwrap().velocity.1 < 0.0);
    }

    #[test]
    fn test_bubble_hits_platform_bottom() {
        let platform = Platform::new(Rectangle{x:50.0,y: 100.0,width: 100.0,height: 20.0});
        let mut bubbles = vec![Bubble::new(Rectangle{x:60.0,y: 115.0,width: 20.0,height: 20.0},10.0,( 0.0, -5.0))];

        CollisionSystem::bubbles_platforms_collision(&mut bubbles, &vec![platform]);

        assert!(bubbles.first().unwrap().velocity.1 > 0.0);
    }

    #[test]
    fn test_bubble_hits_platform_side() {
        let platform = Platform::new(Rectangle{x:100.0,y: 50.0,width: 50.0,height: 50.0});
        let mut bubbles = vec![Bubble::new(Rectangle{x:95.0,y: 60.0,width: 20.0,height: 20.0},10.0,( 5.0, 0.0))];

        CollisionSystem::bubbles_platforms_collision(&mut bubbles, &vec![platform]);

        assert!(bubbles.first().unwrap().velocity.0 < 0.0);
    }

    #[test]
    fn test_multiple_bubbles_multiple_platforms() {
        let platforms = vec![
            Platform::new(Rectangle{x:0.0,y: 200.0,width: 300.0,height: 20.0}),
            Platform::new(Rectangle{x:150.0,y: 100.0,width: 100.0,height: 20.0}),
        ];

        let mut bubbles = vec![
            Bubble::new( Rectangle{x:50.0,y: 195.0,width: 20.0,height: 20.0},10.0,( 0.0, 5.0)),
            Bubble::new( Rectangle{x:160.0,y: 95.0,width: 20.0,height: 20.0},10.0,( 0.0, 5.0)),
        ];

        CollisionSystem::bubbles_platforms_collision(&mut bubbles, &platforms);

        assert!(bubbles.first().unwrap().velocity.1 < 0.0);
        assert!(bubbles.first().unwrap().velocity.1 < 0.0);
    }

        #[test]
    fn test_harpoon_hits_bubble_and_removes_it() {
        let mut harpoon = Harpoon::new(Rectangle { x: 50.0, y: 50.0, width: 10.0, height: 10.0 },HarpoonState::Active,);
        let mut bubbles = vec![Bubble::new( Rectangle { x: 50.0, y: 50.0, width: 10.0, height: 10.0 },10.0,(0.0,0.0))];

        CollisionSystem::harpoon_bubbles_collision(&mut harpoon, &mut bubbles);

        assert!(bubbles.is_empty());
        assert!(matches!(harpoon.state, HarpoonState::Inactive));
    }

    #[test]
    fn test_harpoon_hits_bubble_and_spawns_children() {
        let mut harpoon = Harpoon::new(Rectangle { x: 50.0, y: 50.0, width: 10.0, height: 10.0 },HarpoonState::Active,);
        let mut bubbles = vec![Bubble::new(Rectangle { x: 50.0, y: 50.0, width: 20.0, height: 20.0 },25.0,(1.0,0.0))];

        CollisionSystem::harpoon_bubbles_collision(&mut harpoon, &mut bubbles);

        assert_eq!(bubbles.len(), 2);
    }

    #[test]
    fn test_harpoon_does_not_remove_bubble_when_inactive() {
        let mut harpoon = Harpoon::new(Rectangle { x: 50.0, y: 50.0, width: 10.0, height: 10.0 },HarpoonState::Inactive);
        let mut bubbles = vec![Bubble::new(Rectangle { x: 50.0, y: 50.0, width: 10.0, height: 10.0 },10.0,(0.0,0.0))];

        CollisionSystem::harpoon_bubbles_collision(&mut harpoon, &mut bubbles);

        assert_eq!(bubbles.len(), 1);
    }

    #[test]
    fn test_harpoon_platform_bottom_collision() {
        let mut harpoon = Harpoon::new(Rectangle { x: 50.0, y: 100.0, width: 10.0, height: 10.0 },HarpoonState::Active);
        let platform = Platform::new(Rectangle { x: 50.0, y: 100.0, width: 50.0, height: 20.0 });

        CollisionSystem::harpoon_platform_collision(&mut harpoon, &vec![platform]);

        assert!(matches!(harpoon.state, HarpoonState::Stationary));
    }

    #[test]
    fn test_harpoon_no_collision() {
        let mut harpoon = Harpoon::new(Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 },HarpoonState::Active);
        let platform = Platform::new(Rectangle { x: 100.0, y: 100.0, width: 50.0, height: 20.0 });

        CollisionSystem::harpoon_platform_collision(&mut harpoon, &vec![platform]);

        assert!(matches!(harpoon.state, HarpoonState::Active));
    }

    #[test]
    fn test_harpoon_hits_ceiling() {
        let mut harpoon = Harpoon::new(Rectangle { x: 50.0, y: 0.0, width: 10.0, height: 10.0 },HarpoonState::Active);

        CollisionSystem::harpoon_ceiling_collision(&mut harpoon);

        assert!(matches!(harpoon.state, HarpoonState::Stationary));
    }

    #[test]
    fn test_harpoon_below_ceiling_no_collision() {
        let mut harpoon = Harpoon::new(Rectangle { x: 50.0, y: 10.0, width: 10.0, height: 10.0 },HarpoonState::Active);
        CollisionSystem::harpoon_ceiling_collision(&mut harpoon);

        assert!(matches!(harpoon.state, HarpoonState::Active));
    }
}
