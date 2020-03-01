use fae::{Font, GraphicsContext, Spritesheet};

pub const UI_AREA_WIDTH: f32 = 170.0;

pub struct Ui {}

impl Ui {
    pub fn new() -> Ui {
        Ui {}
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext, font: &Font, spritesheet: &Spritesheet) {
        let (width, height) = (ctx.width, ctx.height);

        let mut menu_cursor_y = 10.0;
        let mut menu_cursor_x = width - UI_AREA_WIDTH + menu_cursor_y;
        spritesheet
            .draw(ctx)
            .coordinates((
                menu_cursor_x,
                menu_cursor_y,
                width - menu_cursor_x - menu_cursor_y,
                height - menu_cursor_y * 2.0,
            ))
            .color((0.2, 0.2, 0.2, 1.0))
            .z(0.75)
            .finish();
        menu_cursor_x += 10.0;
        menu_cursor_y += 10.0;
        font.draw(
            ctx,
            "TODO:\n- User interface",
            menu_cursor_x,
            menu_cursor_y,
            12.0,
        )
        .z(1.0)
        .color((0.9, 0.9, 0.9, 1.0))
        .finish();
    }
}
