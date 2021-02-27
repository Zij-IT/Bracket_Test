use bracket_lib::prelude::BTermBuilder;
use bracket_lib::prelude::*;

struct MinimalExample;

impl GameState for MinimalExample {
    fn tick(&mut self, ctx: &mut BTerm) {
        //Clear every tick
        for i in 0..3 {
            ctx.set_active_console(i);
            ctx.cls();
        }

        //Change if necessary
        if ctx.key == Some(VirtualKeyCode::Key1) {
            for _ in 0..3 {
                ctx.set_active_font(0, true);
            }
        } else if ctx.key == Some(VirtualKeyCode::Key2) {
            for _ in 0..3 {
                ctx.set_active_font(1, true);
            }
        }

        //Test text
        ctx.set_active_console(0);
        ctx.print(0, 0, "Console zero");
        ctx.set_active_console(1);
        ctx.print(1, 1, "Console one");
        ctx.set_active_console(2);
        ctx.print(2, 2, "Console two");
    }
}
embedded_resource!(GAME_FONT, "../resources/cp437_8x8.png");
embedded_resource!(GAME_FONT_THIN, "../resources/cp437_8x8_thin.png");
fn main() -> BError {
    link_resource!(GAME_FONT, "/resources/cp437_8x8.png");
    link_resource!(GAME_FONT_THIN, "/resources/cp437_8x8_thin.png");

    let bterm = BTermBuilder::new()
        .with_title("Bashing Bytes")
        .with_font("cp437_8x8.png", 8, 8)
        .with_font("cp437_8x8_thin.png", 8, 8)
        .with_fullscreen(false)
        .with_dimensions(80, 60)
        .with_simple_console(80, 60, "cp437_8x8.png") // map
        .with_simple_console_no_bg(80, 60, "cp437_8x8.png") // creatures
        .with_sparse_console(80, 60, "cp437_8x8.png") // hud
        .with_tile_dimensions(8, 8)
        .build()?;

    main_loop(bterm, MinimalExample {})
}
