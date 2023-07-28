use std::cmp::{max, min};

/// Returns the intersection of two segments, and the distance between ray origin
/// and the interecting point. None if no interesction.
/// See https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
pub fn find_intersection(
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
    x3: isize,
    y3: isize,
    x4: isize,
    y4: isize,
) -> Option<(isize, isize, isize)> {
    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if denom == 0 {
        return None;
    }

    let t_num = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let t: f32 = t_num as f32 / denom as f32;

    if !(0.0..=1.0).contains(&t) {
        return None;
    }

    let u_num = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);
    let u: f32 = u_num as f32 / denom as f32;

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let result_x = (x3 as f32 + u * (x4 - x3) as f32) as isize;
    let result_y = (y3 as f32 + u * (y4 - y3) as f32) as isize;

    let distance = get_distance_between_points(x3, y3, result_x, result_y);

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
