use coffee::graphics::Rectangle;

pub enum Collision {
    None,
    Horizontal,
    Vertical,
}

pub fn check_collision(lhs: &Rectangle<f32>, rhs: &Rectangle<f32>) -> Collision {
        let overlap_x = lhs.x < rhs.x + rhs.width && lhs.x + lhs.width > rhs.x;
        let overlap_y = lhs.y < rhs.y + rhs.height && lhs.y + lhs.height > rhs.y;

        if overlap_x && overlap_y {
            // Compute overlap amounts along each axis
            let overlap_width = (lhs.x + lhs.width).min(rhs.x + rhs.width) 
                                - lhs.x.max(rhs.x);
            let overlap_height = (lhs.y + lhs.height).min(rhs.y + rhs.height) 
                                 - lhs.y.max(rhs.y);

            if overlap_width < overlap_height {
                Collision::Horizontal
            } else {
                Collision::Vertical
            }
        } else {
            Collision::None
        }
}
