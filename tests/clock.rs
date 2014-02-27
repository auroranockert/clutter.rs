extern mod native;

extern mod cairo;
extern mod clutter;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
  use clutter;
  use clutter::actor::Actor;
  use clutter::stage::Stage;
  use clutter::content::Content;
  use clutter::canvas::Canvas;

  // use clutter::timeline::IsTimeline;

  clutter::init();

  let mut stage = clutter::stage::StageRef::new();
  stage.set_title("2D Clock");
  stage.set_user_resizable(true);
  // stage.set_background_color(…)
  stage.set_size(300.0, 300.0);
  stage.show();

  let mut canvas = clutter::canvas::CanvasRef::new();
  canvas.set_size(300, 300);

  let mut actor = clutter::actor::ActorRef::new();
  actor.set_content(&mut canvas);
  actor.set_content_scaling_filters(clutter::scaling::Trilinear, clutter::scaling::Linear);
  stage.add_child(&mut actor);

  let mut constraint = clutter::constraint::BindConstraintRef::new(&mut stage, clutter::constraint::bind::Size, 0.0);
  actor.add_constraint(&mut constraint);

  let mut c = unsafe { clutter::canvas::CanvasRef { opaque: g_object_ref(canvas.opaque) } };

  actor.on_allocation_changed(&|a, _, _| {
    let v = a.get_size();

    match v { (w, h) => c.set_size(w.ceil() as i32, h.ceil() as i32) };
  });

  stage.on_destroy(&|_| {
    clutter::main_quit()
  });

  canvas.on_draw(&|canvas, cairo, width, height| {
    use clutter::content::Content;

    /* get the current time and compute the angles */
    // now = g_date_time_new_now_local ();
    let seconds = 52.0 / 60.0 * 2.0 * std::f64::consts::PI;
    let minutes = 22.0 / 60.0 * 2.0 * std::f64::consts::PI;
    let hours = 2.0 / 12.0 * 2.0 * std::f64::consts::PI;

    cairo.save();
    cairo.set_operator(cairo::operator::Clear);
    cairo.paint();
    cairo.restore();

    cairo.set_operator(cairo::operator::Over);

    cairo.scale(width as f64, height as f64);

    cairo.set_line_cap(cairo::line_cap::Round);
    cairo.set_line_width(0.1);

    /* the black rail that holds the seconds indicator */
    // clutter_cairo_set_source_color(cr, CLUTTER_COLOR_Black);
    cairo.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cairo.translate(0.5, 0.5);
    cairo.arc(0.0, 0.0, 0.4, 0.0, 2.0 * std::f64::consts::PI);
    cairo.stroke();

    /* the seconds indicator */
    // color = *CLUTTER_COLOR_White;
    // color.alpha = 128;
    // clutter_cairo_set_source_color (cr, &color);
    cairo.set_source_rgba(1.0, 1.0, 1.0, 0.5);
    cairo.move_to(0.0, 0.0);
    cairo.arc(std::f64::sin(seconds) * 0.4, -std::f64::cos(seconds) * 0.4, 0.05, 0.0, std::f64::consts::PI * 2.0);
    cairo.fill();

    /* the minutes hand */
    // color = *CLUTTER_COLOR_DarkChameleon;
    // color.alpha = 196;
    // clutter_cairo_set_source_color (cr, &color);
    cairo.set_source_rgba(0.5, 1.0, 0.5, 0.75);
    cairo.move_to(0.0, 0.0);
    cairo.line_to(std::f64::sin(minutes) * 0.4, -std::f64::cos(minutes) * 0.4);
    cairo.stroke();

    /* the hours hand */
    cairo.move_to(0.0, 0.0);
    cairo.line_to(std::f64::sin(hours) * 0.2, -std::f64::cos(hours) * 0.2);
    cairo.stroke();

    /* we're done drawing */
    return true;
  });

  canvas.invalidate();

  // clutter::thread::add_timeout(1000, |c| { c.invalidate() }, canvas)

  clutter::main();
}

extern {
  fn g_object_ref(obj: *mut std::libc::c_void) -> *mut std::libc::c_void;
}