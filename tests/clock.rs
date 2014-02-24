extern mod native;
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

  // use clutter::timeline::IsTimeline;

  clutter::init();

  let mut stage = clutter::stage::StageRef::new();

  stage.set_title("2D Clock");

  stage.set_size(300.0, 300.0);
  stage.set_user_resizable(true);

  // stage.set_background_color(&clutter::Color { red: 0, green: 0, blue: 0, alpha: 255 });

  stage.show();

  clutter::main();
}