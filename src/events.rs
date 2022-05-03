use std::f32::consts::PI;

use super::*;

pub fn lmb_drag_event(
    is_lmb_down: &mut bool,
    click_timer: &mut Instant,
    pos: (f32, f32),
    last: (f32, f32),
    angle: &mut Angle,
    scroll_delta: f32,
) {
    if !*is_lmb_down {
        *is_lmb_down = true;
        *click_timer = Instant::now();
    }
    if click_timer.elapsed().as_millis() >= CLICK_TIMEOUT {
        let x_delta = (pos.0 - last.0) / 200.0;
        let y_delta = (pos.1 - last.1) / 200.0;
        if is_key_down(KeyCode::LeftShift) {
            angle.yz += y_delta;
            angle.xz += x_delta;
        } else {
            angle.yw += y_delta;
            angle.xw += x_delta;
        }
        angle.yz = angle.yz.max(-PI / 2.0).min(PI / 2.0);
        angle.yw = angle.yw.max(-PI / 2.0).min(PI / 2.0);
        angle.zw += scroll_delta / 100.0;
    }
}

pub fn lmb_click_event(
    hover: bool,
    selection_type_buttons: &mut Vec<(Texture2D, bool, bool)>,
    hover_i: usize,
    objects: &mut Vec<Object>,
    xy: (f32, f32),
) {
    if hover {
        if is_key_down(KeyCode::LeftShift) {
            if get_enabled_buttons_count(selection_type_buttons) > 1 {
                selection_type_buttons[hover_i].1 = !selection_type_buttons[hover_i].1;
            } else if !selection_type_buttons[hover_i].1 {
                selection_type_buttons[hover_i].1 = true;
            }
        } else {
            for b in selection_type_buttons.iter_mut() {
                b.1 = false
            }
            selection_type_buttons[hover_i].1 = true;
        }
    }
    for obj in objects.iter_mut() {
        if selection_type_buttons[0].1 {
            if let Some(index) = find_closest_vertice(xy.0, xy.1, &obj.vertices) {
                let v = obj.vertices.get_mut(index).unwrap();
                if is_key_down(KeyCode::LeftShift) { v.selected = !v.selected; }
                else { clear_selection_vertices(&mut obj.vertices, index); }
                break;
            }
        }
        if selection_type_buttons[1].1 {
            if let Some(index) = find_closest_edge(xy.0, xy.1, &obj) {
                let e = obj.edges.get_mut(index).unwrap();
                if is_key_down(KeyCode::LeftShift) { e.2 = !e.2; }
                else { clear_selection_edges(&mut obj.edges, index); }
            }
        }
    }
}

pub fn resize_event<'a>(windows: &'a mut WindowGroup) {
    (*windows).main.set_size(screen_width(), screen_height());
}