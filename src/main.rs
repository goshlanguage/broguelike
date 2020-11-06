use bear_lib_terminal::terminal::{self, config, Event, KeyCode};
use bear_lib_terminal::geometry::Size;

struct Player {
  x: i32,
  y: i32,
  visible: bool,
}

fn main() {
  terminal::open("BrogueLike", 80, 40);
  terminal::set(config::font::true_type(config::font::Origin::Root, "square.ttf", Size::new(0, 12)));

  let player = &mut Player{ x: 4, y: 0, visible: true };
  let welcome = "[color=green]BROGUELIKE";

  loop {
    terminal::clear(None);
    terminal::print_xy(4, 5, welcome);
    terminal::print_xy(4, 7, "[color=gray]## HACKERS");
    if player.visible {
      print_at(player);
    }
    terminal::refresh();
    // let _ = terminal::wait_event();

    let event = terminal::read_event();
    match event {
      Some(e) => match e {
        Event::KeyPressed { key: KeyCode::Up, ctrl: _, shift: _ } => move_y(player, -1),
        Event::KeyPressed { key: KeyCode::Down, ctrl: _, shift: _ } => move_y(player, 1),
        Event::KeyPressed { key: KeyCode::Left, ctrl: _, shift: _ } => move_x(player, -1),
        Event::KeyPressed { key: KeyCode::Right, ctrl: _, shift: _ } => move_x(player, 1),
        Event::KeyPressed { key: KeyCode::Escape, ctrl: _, shift: _ } => break,
        _ => {}
      }
      _ => (),
    }
  }

  terminal::close();
}

fn print_at(p: &mut Player){
  terminal::print_xy(p.x, p.y, "[color=yellow]@");
  terminal::refresh();
}

fn move_x(p: &mut Player, x: i32){
  p.x = p.x + x;
  if p.x < 0 {
    p.x = 0
  }
}

fn move_y(p: &mut Player, y: i32){
    p.y = p.y + y;
    if p.y < 0 {
      p.y = 0
    }
}

// fn scrollPrint(x: &mut Uint, text: &mut String) {
//   for y in 0..30 {
//     terminal::print_xy(x, y, text);
//     terminal::refresh();
//   }
// }
