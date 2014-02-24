extern mod cairo;

#[link(name = "clutter-1.0")]
extern {}

pub fn init() {
  unsafe {
    clutter_init(std::ptr::mut_null(), std::ptr::mut_null());
  }
}

pub fn main() {
  unsafe {
    clutter_main();
  }
}

pub mod actor {
  use std;
  use cairo;

  #[repr(i32)]
  pub enum Flags {
    Mapped = 2,
    Realized = 4,
    Reactive = 8,
    Visible = 16,
    NoLayout = 32
  }

  pub struct ActorRef {
    opaque: *mut std::libc::c_void
  }

  impl ActorRef {
    pub fn new() -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_new();
        return foreign_result;
      }
    }
  }

  pub trait Actor {
    fn as_actor(&self) -> *mut std::libc::c_void;

    fn set_flags(&mut self, flags: Flags) {
      unsafe {
        clutter_actor_set_flags(self.as_actor(), flags);
      }
    }

    fn unset_flags(&mut self, flags: Flags) {
      unsafe {
        clutter_actor_unset_flags(self.as_actor(), flags);
      }
    }

    fn get_flags(&mut self) -> Flags {
      unsafe {
        let foreign_result = clutter_actor_get_flags(self.as_actor());
        return foreign_result;
      }
    }

    fn set_name(&mut self, name: &str) {
      unsafe {
        use std::c_str::ToCStr;
        clutter_actor_set_name(self.as_actor(), name.to_c_str());
      }
    }

    fn get_name(&mut self) -> std::c_str::CString {
      unsafe {
        let foreign_result = clutter_actor_get_name(self.as_actor());
        return std::c_str::CString::new(foreign_result, false);
      }
    }

    fn get_gid(&mut self) -> i32 {
      unsafe {
        let foreign_result = clutter_actor_get_gid(self.as_actor());
        return foreign_result;
      }
    }

    fn show(&mut self) {
      unsafe {
        clutter_actor_show(self.as_actor());
      }
    }

    fn show_all(&mut self) {
      unsafe {
        clutter_actor_show_all(self.as_actor());
      }
    }

    fn hide(&mut self) {
      unsafe {
        clutter_actor_hide(self.as_actor());
      }
    }

    fn hide_all(&mut self) {
      unsafe {
        clutter_actor_hide_all(self.as_actor());
      }
    }

    fn realize(&mut self) {
      unsafe {
        clutter_actor_realize(self.as_actor());
      }
    }

    fn unrealize(&mut self) {
      unsafe {
        clutter_actor_unrealize(self.as_actor());
      }
    }

    fn paint(&mut self) {
      unsafe {
        clutter_actor_paint(self.as_actor());
      }
    }

    fn continue_paint(&mut self) {
      unsafe {
        clutter_actor_continue_paint(self.as_actor());
      }
    }

    fn queue_redraw(&mut self) {
      unsafe {
        clutter_actor_queue_redraw(self.as_actor());
      }
    }

    fn queue_redraw_with_clip(&mut self, clip: &cairo::region::Rectangle) {
      unsafe {
        clutter_actor_queue_redraw_with_clip(self.as_actor(), clip);
      }
    }

    fn queue_relayout(&mut self) {
      unsafe {
        clutter_actor_queue_relayout(self.as_actor());
      }
    }

    fn should_pick_repaint(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_should_pick_repaint(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn map(&mut self) {
      unsafe {
        clutter_actor_map(self.as_actor());
      }
    }

    fn unmap(&mut self) {
      unsafe {
        clutter_actor_unmap(self.as_actor());
      }
    }

    fn has_overlaps(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_overlaps(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn has_mapped_clones(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_mapped_clones(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn get_preferred_size(&mut self) -> (f32, f32, f32, f32) {
      unsafe {
        let mut min_width_p:f32 = std::unstable::intrinsics::init();
        let mut min_height_p:f32 = std::unstable::intrinsics::init();
        let mut natural_width_p:f32 = std::unstable::intrinsics::init();
        let mut natural_height_p:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_preferred_size(self.as_actor(), &mut min_width_p, &mut min_height_p, &mut natural_width_p, &mut natural_height_p);
        return (min_width_p, min_height_p, natural_width_p, natural_height_p);
      }
    }

    fn get_preferred_width(&mut self, for_height: f32) -> (f32, f32) {
      unsafe {
        let mut min_width_p:f32 = std::unstable::intrinsics::init();
        let mut natural_width_p:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_preferred_width(self.as_actor(), for_height, &mut min_width_p, &mut natural_width_p);
        return (min_width_p, natural_width_p);
      }
    }

    fn get_preferred_height(&mut self, for_width: f32) -> (f32, f32) {
      unsafe {
        let mut min_height_p:f32 = std::unstable::intrinsics::init();
        let mut natural_height_p:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_preferred_height(self.as_actor(), for_width, &mut min_height_p, &mut natural_height_p);
        return (min_height_p, natural_height_p);
      }
    }

    fn set_fixed_position_set(&mut self, fixed_position_set: bool) {
      unsafe {
        clutter_actor_set_fixed_position_set(self.as_actor(), (fixed_position_set as i32));
      }
    }

    fn get_fixed_position_set(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_fixed_position_set(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn has_allocation(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_allocation(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_x_expand(&mut self, expand: bool) {
      unsafe {
        clutter_actor_set_x_expand(self.as_actor(), (expand as i32));
      }
    }

    fn get_x_expand(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_x_expand(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_y_expand(&mut self, expand: bool) {
      unsafe {
        clutter_actor_set_y_expand(self.as_actor(), (expand as i32));
      }
    }

    fn get_y_expand(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_y_expand(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn needs_expand(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_needs_expand(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_size(&mut self, for_width: f32, for_height: f32) {
      unsafe {
        clutter_actor_set_size(self.as_actor(), for_width, for_height);
      }
    }

    fn get_size(&mut self) -> (f32, f32) {
      unsafe {
        let mut width:f32 = std::unstable::intrinsics::init();
        let mut height:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_size(self.as_actor(), &mut width, &mut height);
        return (width, height);
      }
    }

    fn set_position(&mut self, x: f32, y: f32) {
      unsafe {
        clutter_actor_set_position(self.as_actor(), x, y);
      }
    }

    fn get_position(&mut self) -> (f32, f32) {
      unsafe {
        let mut x:f32 = std::unstable::intrinsics::init();
        let mut y:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_position(self.as_actor(), &mut x, &mut y);
        return (x, y);
      }
    }

    fn set_width(&mut self, width: f32) {
      unsafe {
        clutter_actor_set_width(self.as_actor(), width);
      }
    }

    fn get_width(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_width(self.as_actor());
        return foreign_result;
      }
    }

    fn set_height(&mut self, height: f32) {
      unsafe {
        clutter_actor_set_height(self.as_actor(), height);
      }
    }

    fn get_height(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_height(self.as_actor());
        return foreign_result;
      }
    }

    fn set_x(&mut self, x: f32) {
      unsafe {
        clutter_actor_set_x(self.as_actor(), x);
      }
    }

    fn get_x(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_x(self.as_actor());
        return foreign_result;
      }
    }

    fn set_y(&mut self, y: f32) {
      unsafe {
        clutter_actor_set_y(self.as_actor(), y);
      }
    }

    fn get_y(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_y(self.as_actor());
        return foreign_result;
      }
    }

    fn move_by(&mut self, dx: f32, dy: f32) {
      unsafe {
        clutter_actor_move_by(self.as_actor(), dx, dy);
      }
    }

    fn set_z_position(&mut self, y: f32) {
      unsafe {
        clutter_actor_set_z_position(self.as_actor(), y);
      }
    }

    fn get_z_position(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_z_position(self.as_actor());
        return foreign_result;
      }
    }

    fn set_pivotal_point(&mut self, pivot_x: f32, pivot_y: f32) {
      unsafe {
        clutter_actor_set_pivotal_point(self.as_actor(), pivot_x, pivot_y);
      }
    }

    fn get_pivotal_point(&mut self) -> (f32, f32) {
      unsafe {
        let mut pivot_x:f32 = std::unstable::intrinsics::init();
        let mut pivot_y:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_pivotal_point(self.as_actor(), &mut pivot_x, &mut pivot_y);
        return (pivot_x, pivot_y);
      }
    }

    fn set_pivot_point_z(&mut self, pivot_z: f32) {
      unsafe {
        clutter_actor_set_pivot_point_z(self.as_actor(), pivot_z);
      }
    }

    fn get_pivot_point_z(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_pivot_point_z(self.as_actor());
        return foreign_result;
      }
    }

    fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
      unsafe {
        clutter_actor_set_scale(self.as_actor(), scale_x, scale_y);
      }
    }

    fn get_scale(&mut self) -> (f32, f32) {
      unsafe {
        let mut scale_x:f32 = std::unstable::intrinsics::init();
        let mut scale_y:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_scale(self.as_actor(), &mut scale_x, &mut scale_y);
        return (scale_x, scale_y);
      }
    }

    fn set_scale_z(&mut self, pivot_z: f32) {
      unsafe {
        clutter_actor_set_scale_z(self.as_actor(), pivot_z);
      }
    }

    fn get_scale_z(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_scale_z(self.as_actor());
        return foreign_result;
      }
    }

    fn is_scaled(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_is_scaled(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn is_rotated(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_is_rotated(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_translation(&mut self, translate_x: f32, translate_y: f32, translate_z: f32) {
      unsafe {
        clutter_actor_set_translation(self.as_actor(), translate_x, translate_y, translate_z);
      }
    }

    fn get_translation(&mut self) -> (f32, f32, f32) {
      unsafe {
        let mut translate_x:f32 = std::unstable::intrinsics::init();
        let mut translate_y:f32 = std::unstable::intrinsics::init();
        let mut translate_z:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_translation(self.as_actor(), &mut translate_x, &mut translate_y, &mut translate_z);
        return (translate_x, translate_y, translate_z);
      }
    }

    fn apply_transform_to_point(&mut self, x: f32, y: f32) -> (bool, f32, f32) {
      unsafe {
        let mut x_out:f32 = std::unstable::intrinsics::init();
        let mut y_out:f32 = std::unstable::intrinsics::init();
        let foreign_result = clutter_actor_apply_transform_to_point(self.as_actor(), x, y, &mut x_out, &mut y_out);
        return (foreign_result != 0, x_out, y_out);
      }
    }

    fn transform_stage_point(&mut self, x: f32, y: f32) -> (bool, f32, f32) {
      unsafe {
        let mut x_out:f32 = std::unstable::intrinsics::init();
        let mut y_out:f32 = std::unstable::intrinsics::init();
        let foreign_result = clutter_actor_transform_stage_point(self.as_actor(), x, y, &mut x_out, &mut y_out);
        return (foreign_result != 0, x_out, y_out);
      }
    }

    fn get_transformed_position(&mut self) -> (f32, f32) {
      unsafe {
        let mut x:f32 = std::unstable::intrinsics::init();
        let mut y:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_transformed_position(self.as_actor(), &mut x, &mut y);
        return (x, y);
      }
    }

    fn get_transformed_size(&mut self) -> (f32, f32) {
      unsafe {
        let mut width:f32 = std::unstable::intrinsics::init();
        let mut height:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_transformed_size(self.as_actor(), &mut width, &mut height);
        return (width, height);
      }
    }

    fn get_paint_opacity(&mut self) -> i8 {
      unsafe {
        let foreign_result = clutter_actor_get_paint_opacity(self.as_actor());
        return foreign_result;
      }
    }

    fn get_paint_visibility(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_paint_visibility(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_clip(&mut self, xoff: f32, yoff: f32, width: f32, height: f32) {
      unsafe {
        clutter_actor_set_clip(self.as_actor(), xoff, yoff, width, height);
      }
    }

    fn remove_clip(&mut self) {
      unsafe {
        clutter_actor_remove_clip(self.as_actor());
      }
    }

    fn has_clip(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_clip(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn get_clip(&mut self) -> (f32, f32, f32, f32) {
      unsafe {
        let mut xoff:f32 = std::unstable::intrinsics::init();
        let mut yoff:f32 = std::unstable::intrinsics::init();
        let mut width:f32 = std::unstable::intrinsics::init();
        let mut height:f32 = std::unstable::intrinsics::init();
        clutter_actor_get_clip(self.as_actor(), &mut xoff, &mut yoff, &mut width, &mut height);
        return (xoff, yoff, width, height);
      }
    }

    fn set_clip_to_allocation(&mut self, clip_set: bool) {
      unsafe {
        clutter_actor_set_clip_to_allocation(self.as_actor(), (clip_set as i32));
      }
    }

    fn get_clip_to_allocation(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_clip_to_allocation(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_opacity(&mut self, opacity: i8) {
      unsafe {
        clutter_actor_set_opacity(self.as_actor(), opacity);
      }
    }

    fn get_opacity(&mut self) -> i8 {
      unsafe {
        let foreign_result = clutter_actor_get_opacity(self.as_actor());
        return foreign_result;
      }
    }

    fn is_in_clone_paint(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_is_in_clone_paint(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn add_child<T: Actor>(&mut self, child: T) {
      unsafe {
        clutter_actor_add_child(self.as_actor(), child.as_actor());
      }
    }

    fn insert_child_above<T: Actor, U: Actor>(&mut self, child: T, sibling: U) {
      unsafe {
        clutter_actor_insert_child_above(self.as_actor(), child.as_actor(), sibling.as_actor());
      }
    }

    fn insert_child_at_index<T: Actor>(&mut self, child: T, index: i32) {
      unsafe {
        clutter_actor_insert_child_at_index(self.as_actor(), child.as_actor(), index);
      }
    }

    fn insert_child_below<T: Actor, U: Actor>(&mut self, child: T, sibling: U) {
      unsafe {
        clutter_actor_insert_child_below(self.as_actor(), child.as_actor(), sibling.as_actor());
      }
    }

    fn replace_child<T: Actor, U: Actor>(&mut self, old_child: T, new_child: U) {
      unsafe {
        clutter_actor_replace_child(self.as_actor(), old_child.as_actor(), new_child.as_actor());
      }
    }

    fn remove_child<T: Actor>(&mut self, child: T) {
      unsafe {
        clutter_actor_remove_child(self.as_actor(), child.as_actor());
      }
    }

    fn remove_all_children(&mut self) {
      unsafe {
        clutter_actor_remove_all_children(self.as_actor());
      }
    }

    fn destroy_all_children(&mut self) {
      unsafe {
        clutter_actor_destroy_all_children(self.as_actor());
      }
    }

    fn get_first_child(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_first_child(self.as_actor());
        return foreign_result;
      }
    }

    fn get_next_sibling(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_next_sibling(self.as_actor());
        return foreign_result;
      }
    }

    fn get_previous_sibling(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_previous_sibling(self.as_actor());
        return foreign_result;
      }
    }

    fn get_last_child(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_last_child(self.as_actor());
        return foreign_result;
      }
    }

    fn get_child_at_index(&mut self, index: i32) {
      unsafe {
        clutter_actor_get_child_at_index(self.as_actor(), index);
      }
    }

    fn get_n_children(&mut self) -> i32 {
      unsafe {
        let foreign_result = clutter_actor_get_n_children(self.as_actor());
        return foreign_result;
      }
    }

    fn get_parent(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_parent(self.as_actor());
        return foreign_result;
      }
    }

    fn set_child_above_sibling<T: Actor, U: Actor>(&mut self, child: T, sibling: U) {
      unsafe {
        clutter_actor_set_child_above_sibling(self.as_actor(), child.as_actor(), sibling.as_actor());
      }
    }

    fn set_child_at_index<T: Actor>(&mut self, child: T, index: i32) {
      unsafe {
        clutter_actor_set_child_at_index(self.as_actor(), child.as_actor(), index);
      }
    }

    fn set_child_below_sibling<T: Actor, U: Actor>(&mut self, child: T, sibling: U) {
      unsafe {
        clutter_actor_set_child_below_sibling(self.as_actor(), child.as_actor(), sibling.as_actor());
      }
    }

    fn contains<T: Actor>(&mut self, descendant: T) -> bool {
      unsafe {
        let foreign_result = clutter_actor_contains(self.as_actor(), descendant.as_actor());
        return foreign_result != 0;
      }
    }

    fn get_stage(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_stage(self.as_actor());
        return foreign_result;
      }
    }

    fn save_easing_state(&mut self) {
      unsafe {
        clutter_actor_save_easing_state(self.as_actor());
      }
    }

    fn restore_easing_state(&mut self) {
      unsafe {
        clutter_actor_restore_easing_state(self.as_actor());
      }
    }

    fn set_easing_duration(&mut self, msecs: i32) {
      unsafe {
        clutter_actor_set_easing_duration(self.as_actor(), msecs);
      }
    }

    fn get_easing_duration(&mut self) -> i32 {
      unsafe {
        let foreign_result = clutter_actor_get_easing_duration(self.as_actor());
        return foreign_result;
      }
    }

    fn set_easing_delay(&mut self, msecs: i32) {
      unsafe {
        clutter_actor_set_easing_delay(self.as_actor(), msecs);
      }
    }

    fn get_easing_delay(&mut self) -> i32 {
      unsafe {
        let foreign_result = clutter_actor_get_easing_delay(self.as_actor());
        return foreign_result;
      }
    }

    fn set_reactive(&mut self, msecs: bool) {
      unsafe {
        clutter_actor_set_reactive(self.as_actor(), (msecs as i32));
      }
    }

    fn get_reactive(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_reactive(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn has_key_focus(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_key_focus(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn grap_key_focus(&mut self) {
      unsafe {
        clutter_actor_grap_key_focus(self.as_actor());
      }
    }

    fn has_pointer(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_pointer(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn has_actions(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_actions(self.as_actor());
        return foreign_result != 0;
      }
    }
  }

  impl Actor for ActorRef {
    fn as_actor(&self) -> *mut std::libc::c_void {
      return self.opaque;
    }
  }

  extern {
    fn clutter_actor_new() -> ActorRef;
    fn clutter_actor_set_flags(self_value: *mut std::libc::c_void, flags: Flags);
    fn clutter_actor_unset_flags(self_value: *mut std::libc::c_void, flags: Flags);
    fn clutter_actor_get_flags(self_value: *mut std::libc::c_void) -> Flags;
    fn clutter_actor_set_name(self_value: *mut std::libc::c_void, name: std::c_str::CString);
    fn clutter_actor_get_name(self_value: *mut std::libc::c_void) -> *i8;
    fn clutter_actor_get_gid(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_show(self_value: *mut std::libc::c_void);
    fn clutter_actor_show_all(self_value: *mut std::libc::c_void);
    fn clutter_actor_hide(self_value: *mut std::libc::c_void);
    fn clutter_actor_hide_all(self_value: *mut std::libc::c_void);
    fn clutter_actor_realize(self_value: *mut std::libc::c_void);
    fn clutter_actor_unrealize(self_value: *mut std::libc::c_void);
    fn clutter_actor_paint(self_value: *mut std::libc::c_void);
    fn clutter_actor_continue_paint(self_value: *mut std::libc::c_void);
    fn clutter_actor_queue_redraw(self_value: *mut std::libc::c_void);
    fn clutter_actor_queue_redraw_with_clip(self_value: *mut std::libc::c_void, clip: *cairo::region::Rectangle);
    fn clutter_actor_queue_relayout(self_value: *mut std::libc::c_void);
    fn clutter_actor_should_pick_repaint(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_map(self_value: *mut std::libc::c_void);
    fn clutter_actor_unmap(self_value: *mut std::libc::c_void);
    fn clutter_actor_has_overlaps(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_has_mapped_clones(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_get_preferred_size(self_value: *mut std::libc::c_void, min_width_p: *mut f32, min_height_p: *mut f32, natural_width_p: *mut f32, natural_height_p: *mut f32);
    fn clutter_actor_get_preferred_width(self_value: *mut std::libc::c_void, for_height: f32, min_width_p: *mut f32, natural_width_p: *mut f32);
    fn clutter_actor_get_preferred_height(self_value: *mut std::libc::c_void, for_width: f32, min_height_p: *mut f32, natural_height_p: *mut f32);
    fn clutter_actor_set_fixed_position_set(self_value: *mut std::libc::c_void, fixed_position_set: i32);
    fn clutter_actor_get_fixed_position_set(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_has_allocation(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_set_x_expand(self_value: *mut std::libc::c_void, expand: i32);
    fn clutter_actor_get_x_expand(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_set_y_expand(self_value: *mut std::libc::c_void, expand: i32);
    fn clutter_actor_get_y_expand(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_needs_expand(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_set_size(self_value: *mut std::libc::c_void, for_width: f32, for_height: f32);
    fn clutter_actor_get_size(self_value: *mut std::libc::c_void, width: *mut f32, height: *mut f32);
    fn clutter_actor_set_position(self_value: *mut std::libc::c_void, x: f32, y: f32);
    fn clutter_actor_get_position(self_value: *mut std::libc::c_void, x: *mut f32, y: *mut f32);
    fn clutter_actor_set_width(self_value: *mut std::libc::c_void, width: f32);
    fn clutter_actor_get_width(self_value: *mut std::libc::c_void) -> f32;
    fn clutter_actor_set_height(self_value: *mut std::libc::c_void, height: f32);
    fn clutter_actor_get_height(self_value: *mut std::libc::c_void) -> f32;
    fn clutter_actor_set_x(self_value: *mut std::libc::c_void, x: f32);
    fn clutter_actor_get_x(self_value: *mut std::libc::c_void) -> f32;
    fn clutter_actor_set_y(self_value: *mut std::libc::c_void, y: f32);
    fn clutter_actor_get_y(self_value: *mut std::libc::c_void) -> f32;
    fn clutter_actor_move_by(self_value: *mut std::libc::c_void, dx: f32, dy: f32);
    fn clutter_actor_set_z_position(self_value: *mut std::libc::c_void, y: f32);
    fn clutter_actor_get_z_position(self_value: *mut std::libc::c_void) -> f32;
    fn clutter_actor_set_pivotal_point(self_value: *mut std::libc::c_void, pivot_x: f32, pivot_y: f32);
    fn clutter_actor_get_pivotal_point(self_value: *mut std::libc::c_void, pivot_x: *mut f32, pivot_y: *mut f32);
    fn clutter_actor_set_pivot_point_z(self_value: *mut std::libc::c_void, pivot_z: f32);
    fn clutter_actor_get_pivot_point_z(self_value: *mut std::libc::c_void) -> f32;
    fn clutter_actor_set_scale(self_value: *mut std::libc::c_void, scale_x: f32, scale_y: f32);
    fn clutter_actor_get_scale(self_value: *mut std::libc::c_void, scale_x: *mut f32, scale_y: *mut f32);
    fn clutter_actor_set_scale_z(self_value: *mut std::libc::c_void, pivot_z: f32);
    fn clutter_actor_get_scale_z(self_value: *mut std::libc::c_void) -> f32;
    fn clutter_actor_is_scaled(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_is_rotated(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_set_translation(self_value: *mut std::libc::c_void, translate_x: f32, translate_y: f32, translate_z: f32);
    fn clutter_actor_get_translation(self_value: *mut std::libc::c_void, translate_x: *mut f32, translate_y: *mut f32, translate_z: *mut f32);
    fn clutter_actor_apply_transform_to_point(self_value: *mut std::libc::c_void, x: f32, y: f32, x_out: *mut f32, y_out: *mut f32) -> i32;
    fn clutter_actor_transform_stage_point(self_value: *mut std::libc::c_void, x: f32, y: f32, x_out: *mut f32, y_out: *mut f32) -> i32;
    fn clutter_actor_get_transformed_position(self_value: *mut std::libc::c_void, x: *mut f32, y: *mut f32);
    fn clutter_actor_get_transformed_size(self_value: *mut std::libc::c_void, width: *mut f32, height: *mut f32);
    fn clutter_actor_get_paint_opacity(self_value: *mut std::libc::c_void) -> i8;
    fn clutter_actor_get_paint_visibility(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_set_clip(self_value: *mut std::libc::c_void, xoff: f32, yoff: f32, width: f32, height: f32);
    fn clutter_actor_remove_clip(self_value: *mut std::libc::c_void);
    fn clutter_actor_has_clip(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_get_clip(self_value: *mut std::libc::c_void, xoff: *mut f32, yoff: *mut f32, width: *mut f32, height: *mut f32);
    fn clutter_actor_set_clip_to_allocation(self_value: *mut std::libc::c_void, clip_set: i32);
    fn clutter_actor_get_clip_to_allocation(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_set_opacity(self_value: *mut std::libc::c_void, opacity: i8);
    fn clutter_actor_get_opacity(self_value: *mut std::libc::c_void) -> i8;
    fn clutter_actor_is_in_clone_paint(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_add_child(self_value: *mut std::libc::c_void, child: *mut std::libc::c_void);
    fn clutter_actor_insert_child_above(self_value: *mut std::libc::c_void, child: *mut std::libc::c_void, sibling: *mut std::libc::c_void);
    fn clutter_actor_insert_child_at_index(self_value: *mut std::libc::c_void, child: *mut std::libc::c_void, index: i32);
    fn clutter_actor_insert_child_below(self_value: *mut std::libc::c_void, child: *mut std::libc::c_void, sibling: *mut std::libc::c_void);
    fn clutter_actor_replace_child(self_value: *mut std::libc::c_void, old_child: *mut std::libc::c_void, new_child: *mut std::libc::c_void);
    fn clutter_actor_remove_child(self_value: *mut std::libc::c_void, child: *mut std::libc::c_void);
    fn clutter_actor_remove_all_children(self_value: *mut std::libc::c_void);
    fn clutter_actor_destroy_all_children(self_value: *mut std::libc::c_void);
    fn clutter_actor_get_first_child(self_value: *mut std::libc::c_void) -> ActorRef;
    fn clutter_actor_get_next_sibling(self_value: *mut std::libc::c_void) -> ActorRef;
    fn clutter_actor_get_previous_sibling(self_value: *mut std::libc::c_void) -> ActorRef;
    fn clutter_actor_get_last_child(self_value: *mut std::libc::c_void) -> ActorRef;
    fn clutter_actor_get_child_at_index(self_value: *mut std::libc::c_void, index: i32);
    fn clutter_actor_get_n_children(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_get_parent(self_value: *mut std::libc::c_void) -> ActorRef;
    fn clutter_actor_set_child_above_sibling(self_value: *mut std::libc::c_void, child: *mut std::libc::c_void, sibling: *mut std::libc::c_void);
    fn clutter_actor_set_child_at_index(self_value: *mut std::libc::c_void, child: *mut std::libc::c_void, index: i32);
    fn clutter_actor_set_child_below_sibling(self_value: *mut std::libc::c_void, child: *mut std::libc::c_void, sibling: *mut std::libc::c_void);
    fn clutter_actor_contains(self_value: *mut std::libc::c_void, descendant: *mut std::libc::c_void) -> i32;
    fn clutter_actor_get_stage(self_value: *mut std::libc::c_void) -> ActorRef;
    fn clutter_actor_save_easing_state(self_value: *mut std::libc::c_void);
    fn clutter_actor_restore_easing_state(self_value: *mut std::libc::c_void);
    fn clutter_actor_set_easing_duration(self_value: *mut std::libc::c_void, msecs: i32);
    fn clutter_actor_get_easing_duration(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_set_easing_delay(self_value: *mut std::libc::c_void, msecs: i32);
    fn clutter_actor_get_easing_delay(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_set_reactive(self_value: *mut std::libc::c_void, msecs: i32);
    fn clutter_actor_get_reactive(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_has_key_focus(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_grap_key_focus(self_value: *mut std::libc::c_void);
    fn clutter_actor_has_pointer(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_actor_has_actions(self_value: *mut std::libc::c_void) -> i32;
  }
}

pub mod stage {
  use std;
  use super::actor::Actor;

  pub struct StageRef {
    opaque: *mut std::libc::c_void
  }

  impl StageRef {
    pub fn new() -> StageRef {
      unsafe {
        let foreign_result = clutter_stage_new();
        return foreign_result;
      }
    }
  }

  pub trait Stage {
    fn as_stage(&self) -> *mut std::libc::c_void;

    fn ensure_current(&mut self) {
      unsafe {
        clutter_stage_ensure_current(self.as_stage());
      }
    }

    fn ensure_viewport(&mut self) {
      unsafe {
        clutter_stage_ensure_viewport(self.as_stage());
      }
    }

    fn ensure_redraw(&mut self) {
      unsafe {
        clutter_stage_ensure_redraw(self.as_stage());
      }
    }

    fn set_key_focus<T: Actor>(&mut self, actor: T) {
      unsafe {
        clutter_stage_set_key_focus(self.as_stage(), actor.as_actor());
      }
    }

    fn get_key_focus(&mut self) -> super::actor::ActorRef {
      unsafe {
        let foreign_result = clutter_stage_get_key_focus(self.as_stage());
        return foreign_result;
      }
    }

    fn set_throttle_motion_events(&mut self, throttle: bool) {
      unsafe {
        clutter_stage_set_throttle_motion_events(self.as_stage(), (throttle as i32));
      }
    }

    fn get_throttle_motion_events(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_throttle_motion_events(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_use_alpha(&mut self, use_alpha: bool) {
      unsafe {
        clutter_stage_set_use_alpha(self.as_stage(), (use_alpha as i32));
      }
    }

    fn get_use_alpha(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_use_alpha(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_minimum_size(&mut self, width: i32, height: i32) {
      unsafe {
        clutter_stage_set_minimum_size(self.as_stage(), width, height);
      }
    }

    fn get_minimum_size(&mut self) -> (i32, i32) {
      unsafe {
        let mut width:i32 = std::unstable::intrinsics::init();
        let mut height:i32 = std::unstable::intrinsics::init();
        clutter_stage_get_minimum_size(self.as_stage(), &mut width, &mut height);
        return (width, height);
      }
    }

    fn set_no_clear_hint(&mut self, use_alpha: bool) {
      unsafe {
        clutter_stage_set_no_clear_hint(self.as_stage(), (use_alpha as i32));
      }
    }

    fn get_no_clear_hint(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_no_clear_hint(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_motion_events_enabled(&mut self, enabled: bool) {
      unsafe {
        clutter_stage_set_motion_events_enabled(self.as_stage(), (enabled as i32));
      }
    }

    fn get_motion_events_enabled(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_motion_events_enabled(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_title(&mut self, title: &str) {
      unsafe {
        use std::c_str::ToCStr;
        clutter_stage_set_title(self.as_stage(), title.to_c_str());
      }
    }

    fn get_title(&mut self) -> std::c_str::CString {
      unsafe {
        let foreign_result = clutter_stage_get_title(self.as_stage());
        return std::c_str::CString::new(foreign_result, false);
      }
    }

    fn set_user_resizable(&mut self, resizable: bool) {
      unsafe {
        clutter_stage_set_user_resizable(self.as_stage(), (resizable as i32));
      }
    }

    fn get_user_resizable(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_user_resizable(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
      unsafe {
        clutter_stage_set_fullscreen(self.as_stage(), (fullscreen as i32));
      }
    }

    fn get_fullscreen(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_fullscreen(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn show_cursor(&mut self) {
      unsafe {
        clutter_stage_show_cursor(self.as_stage());
      }
    }

    fn hide_cursor(&mut self) {
      unsafe {
        clutter_stage_hide_cursor(self.as_stage());
      }
    }

    fn set_accept_focus(&mut self, accept_focus: bool) {
      unsafe {
        clutter_stage_set_accept_focus(self.as_stage(), (accept_focus as i32));
      }
    }

    fn get_accept_focus(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_accept_focus(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_sync_delay(&mut self, accept_focus: i32) {
      unsafe {
        clutter_stage_set_sync_delay(self.as_stage(), accept_focus);
      }
    }

    fn skip_sync_delay(&mut self) {
      unsafe {
        clutter_stage_skip_sync_delay(self.as_stage());
      }
    }
  }

  impl Stage for StageRef {
    fn as_stage(&self) -> *mut std::libc::c_void {
      return self.opaque;
    }
  }

  impl Actor for StageRef {
    fn as_actor(&self) -> *mut std::libc::c_void {
      return self.opaque;
    }
  }

  extern {
    fn clutter_stage_new() -> StageRef;
    fn clutter_stage_ensure_current(self_value: *mut std::libc::c_void);
    fn clutter_stage_ensure_viewport(self_value: *mut std::libc::c_void);
    fn clutter_stage_ensure_redraw(self_value: *mut std::libc::c_void);
    fn clutter_stage_set_key_focus(self_value: *mut std::libc::c_void, actor: *mut std::libc::c_void);
    fn clutter_stage_get_key_focus(self_value: *mut std::libc::c_void) -> super::actor::ActorRef;
    fn clutter_stage_set_throttle_motion_events(self_value: *mut std::libc::c_void, throttle: i32);
    fn clutter_stage_get_throttle_motion_events(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_stage_set_use_alpha(self_value: *mut std::libc::c_void, use_alpha: i32);
    fn clutter_stage_get_use_alpha(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_stage_set_minimum_size(self_value: *mut std::libc::c_void, width: i32, height: i32);
    fn clutter_stage_get_minimum_size(self_value: *mut std::libc::c_void, width: *mut i32, height: *mut i32);
    fn clutter_stage_set_no_clear_hint(self_value: *mut std::libc::c_void, use_alpha: i32);
    fn clutter_stage_get_no_clear_hint(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_stage_set_motion_events_enabled(self_value: *mut std::libc::c_void, enabled: i32);
    fn clutter_stage_get_motion_events_enabled(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_stage_set_title(self_value: *mut std::libc::c_void, title: std::c_str::CString);
    fn clutter_stage_get_title(self_value: *mut std::libc::c_void) -> *i8;
    fn clutter_stage_set_user_resizable(self_value: *mut std::libc::c_void, resizable: i32);
    fn clutter_stage_get_user_resizable(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_stage_set_fullscreen(self_value: *mut std::libc::c_void, fullscreen: i32);
    fn clutter_stage_get_fullscreen(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_stage_show_cursor(self_value: *mut std::libc::c_void);
    fn clutter_stage_hide_cursor(self_value: *mut std::libc::c_void);
    fn clutter_stage_set_accept_focus(self_value: *mut std::libc::c_void, accept_focus: i32);
    fn clutter_stage_get_accept_focus(self_value: *mut std::libc::c_void) -> i32;
    fn clutter_stage_set_sync_delay(self_value: *mut std::libc::c_void, accept_focus: i32);
    fn clutter_stage_skip_sync_delay(self_value: *mut std::libc::c_void);
  }
}

extern {
  fn clutter_init(argc: *mut i32, argv: *mut i32);
  fn clutter_main();
}
