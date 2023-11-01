use std::cmp::{max, min};

pub struct Segment {
    pub x1: isize,
    pub y1: isize,
    pub x2: isize,
    pub y2: isize
}

/// Returns the intersection of two segments, and the distance between ray origin
/// and the interecting point. None if no interesction.
/// See https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
pub fn find_intersection(seg_1: Segment, seg_2: Segment) -> Option<(isize, isize, isize)> {
    let denom = (seg_1.x1 - seg_1.x2) * (seg_2.y1 - seg_2.y2) - (seg_1.y1 - seg_1.y2) * (seg_2.x1 - seg_2.x2);
    if denom == 0 {
        return None;
    }

    let t_num = (seg_1.x1 - seg_2.x1) * (seg_2.y1 - seg_2.y2) - (seg_1.y1 - seg_2.y1) * (seg_2.x1 - seg_2.x2);
    let t: f32 = t_num as f32 / denom as f32;

    if !(0.0..=1.0).contains(&t) {
        return None;
    }

    let u_num = (seg_1.x1 - seg_2.x1) * (seg_1.y1 - seg_1.y2) - (seg_1.y1 - seg_2.y1) * (seg_1.x1 - seg_1.x2);
    let u: f32 = u_num as f32 / denom as f32;

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let result_x = (seg_2.x1 as f32 + u * (seg_2.x2 - seg_2.x1) as f32) as isize;
    let result_y = (seg_2.y1 as f32 + u * (seg_2.y2 - seg_2.y1) as f32) as isize;

    let distance = get_distance_between_points(seg_2.x1, seg_2.y1, result_x, result_y);

    Some((result_x, result_y, distance))
}

pub fn get_distance_between_points(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    let x_dist = max(x1, x2) - min(x1, x2);
    let y_dist = max(y1, y2) - min(y1, y2);

    let mut distance = (x_dist * x_dist + y_dist * y_dist) as f64;
    distance = distance.sqrt();
    distance as isize
}

pub fn cast_ray(x: isize, y: isize, angle_rad: f32, lenght: isize) -> (isize, isize) {
    let x1 = x;
    let y1 = y;

    let x_move = angle_rad.cos() * lenght as f32;
    let y_move = angle_rad.sin() * lenght as f32;

    let x2: isize = if x_move.is_sign_negative() {
        x1 - (-x_move).round() as isize
    } else {
        x1 + x_move.round() as isize
    };

    let y2: isize = if y_move.is_sign_negative() {
        y1 - (-y_move).round() as isize
    } else {
        y1 + y_move.round() as isize
    };

    (x2, y2)
}

pub fn range_conversion(
    old_min: f32,
    old_max: f32,
    old_value: f32,
    new_min: f32,
    new_max: f32,
) -> f32 {
    let old_range = old_max - old_min;
    let new_range = new_max - new_min;
    (((old_value - old_min) * new_range) / old_range) + new_min
}
