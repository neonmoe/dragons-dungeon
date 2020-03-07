use crate::layers;
use crate::world::{Item, World};
use fae::{Alignment, Font, GraphicsContext, Spritesheet};
use std::sync::Mutex;

pub const UI_AREA_WIDTH: f32 = 230.0;

#[derive(Debug, Default)]
pub struct DebugState {
    pub entity_count: usize,
}

lazy_static::lazy_static! {
    static ref DEBUG_STATE: Mutex<DebugState> = Mutex::new(DebugState::default());
}

impl DebugState {
    pub fn modify<F: FnOnce(&mut DebugState)>(f: F) {
        let lock = DEBUG_STATE.lock();
        if let Ok(mut state) = lock {
            f(&mut state);
        }
    }
}

struct MenuFlow {
    x: f32,
    y: f32,
    width: f32,
}

impl MenuFlow {
    fn new(
        ctx: &mut GraphicsContext,
        spritesheet: &Spritesheet,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        padding: f32,
    ) -> MenuFlow {
        let outline_width = 3.0;
        spritesheet
            .draw(ctx)
            .coordinates((
                x - outline_width,
                y - outline_width,
                width + outline_width * 2.0,
                height + outline_width * 2.0,
            ))
            .color((0.1, 0.1, 0.1, 1.0))
            .z(layers::UI_BG)
            .finish();
        spritesheet
            .draw(ctx)
            .coordinates((x, y, width, height))
            .color((0.01, 0.01, 0.01, 1.0))
            .z(layers::UI_BG)
            .finish();

        MenuFlow {
            x: x + padding,
            y: y + padding,
            width: width - padding * 2.0,
        }
    }

    fn print(
        &mut self,
        ctx: &mut GraphicsContext,
        font: &Font,
        message: &str,
        font_size: f32,
        indent: f32,
        centered: bool,
    ) {
        let alignment = if centered {
            Alignment::Center
        } else {
            Alignment::Left
        };
        if message.is_empty() {
            self.y += font_size + 2.0;
        } else if let Some(rect) = font
            .draw(ctx, message, self.x, self.y, font_size)
            .max_width(self.width - 20.0 - indent)
            .alignment(alignment)
            .visibility(false)
            .finish()
        {
            font.draw(ctx, message, self.x + indent, self.y, font_size)
                .color((1.0, 1.0, 1.0, 1.0))
                .max_width(self.width - 20.0 - indent)
                .alignment(alignment)
                .z(layers::UI_TEXT)
                .finish();
            self.y += rect.height + 2.0;
        }
    }

    fn print_header(&mut self, ctx: &mut GraphicsContext, font: &Font, message: &str) {
        self.print(ctx, font, message, 24.0, 0.0, false);
        self.y += 4.0;
    }

    fn print_header_centered(&mut self, ctx: &mut GraphicsContext, font: &Font, message: &str) {
        self.print(ctx, font, message, 24.0, 0.0, true);
        self.y += 4.0;
    }

    fn print_text(&mut self, ctx: &mut GraphicsContext, font: &Font, message: &str) {
        self.print(ctx, font, message, 18.0, 10.0, false);
        self.y += 4.0;
    }

    fn print_stat(&mut self, ctx: &mut GraphicsContext, font: &Font, message: &str) {
        self.print(ctx, font, message, 18.0, 10.0, false);
        self.y += 2.0;
    }

    fn print_item(&mut self, ctx: &mut GraphicsContext, font: &Font, item: &Item) {
        self.print(ctx, font, item.name(), 18.0, 10.0, false);
        self.y += 6.0;
        self.print(ctx, font, item.description(), 12.0, 16.0, false);
        self.y += 6.0;
    }

    fn space(&mut self) {
        self.y += 20.0;
    }
}

pub struct Ui {}

impl Ui {
    pub fn new() -> Ui {
        Ui {}
    }

    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext,
        font: &Font,
        spritesheet: &Spritesheet,
        world: &World,
        game_over: bool,
        show_debug_info: bool,
    ) {
        let (width, height) = (ctx.width, ctx.height);

        let padding = 10.0;
        let menu_x = width - UI_AREA_WIDTH - padding;
        let menu_y = padding;

        let mut menu = MenuFlow::new(
            ctx,
            spritesheet,
            menu_x,
            menu_y,
            UI_AREA_WIDTH,
            height - padding * 2.0,
            padding,
        );

        let player = world.player();
        let entities = world.entities();

        // Stats:
        if let (Some(damage), Some(health), Some(inv)) =
            (&player.damage, &player.health, &player.inventory)
        {
            menu.print_header(ctx, font, "Stats:");
            menu.print_stat(
                ctx,
                font,
                &format!("Damage: {}", inv.damage_after_items(damage.0)),
            );
            menu.print_stat(
                ctx,
                font,
                &format!("Health: {}/{}", health.current, health.max),
            );
        }
        menu.space();

        // Inventory
        if let Some(inventory) = &player.inventory {
            if !inventory.is_empty() {
                menu.print_header(ctx, font, "Inventory:");

                if let Some(item) = &inventory.item_left {
                    menu.print_item(ctx, font, item);
                }
                if let Some(item) = &inventory.item_right {
                    menu.print_item(ctx, font, item);
                }
            }
        }
        menu.space();

        // Pickups:
        for pickup in entities
            .iter()
            .filter(|e| e.position.x == player.position.x && e.position.y == player.position.y)
            .filter_map(|e| e.drop)
        {
            menu.print_header(ctx, font, "Pick up [,]:");
            menu.print_item(ctx, font, &pickup);
        }
        menu.space();

        // Stairs:
        if entities
            .iter()
            .find(|e| {
                e.position.x == player.position.x
                    && e.position.y == player.position.y
                    && e.next_level
            })
            .is_some()
        {
            menu.print_header(ctx, font, "Mysterious hole");
            menu.print_text(ctx, font, "Press Enter to jump in the hole, where the next level awaits. You cannot climb back up from the hole.");
        }
        menu.space();

        if game_over {
            let width = 410.0;
            let height = 180.0;
            let menu_x = (ctx.width - UI_AREA_WIDTH - padding * 2.0) / 2.0 - width / 2.0;
            let menu_y = ctx.height / 2.0 - height / 2.0;
            let mut menu = MenuFlow::new(ctx, spritesheet, menu_x, menu_y, width, height, 10.0);
            menu.space();
            menu.print_header_centered(ctx, font, "Game over.");
            menu.space();
            menu.print_text(ctx, font, "Your adventure has ended in failure.");
            menu.print_text(ctx, font, "Better luck next time!");
            menu.space();
            menu.print_text(ctx, font, "Press R to retry.");
        }

        if show_debug_info {
            if let Ok(debug_state) = DEBUG_STATE.lock() {
                let ui_text = format!("{:#?}", debug_state);
                let font_size = 12.0;
                if let Some(mut rect) = font
                    .draw(ctx, &ui_text, 0.0, 0.0, font_size)
                    .visibility(false)
                    .finish()
                {
                    let (x, y) = (20.0, ctx.height - rect.height - 20.0);
                    let padding = 10.0;
                    rect.x = x - padding;
                    rect.y = y - padding;
                    rect.width += padding * 2.0;
                    rect.height += padding * 2.0;
                    spritesheet
                        .draw(ctx)
                        .coordinates(rect)
                        .color((0.1, 0.1, 0.1, 0.6))
                        .z(0.99)
                        .finish();
                    font.draw(ctx, &ui_text, x, y, font_size)
                        .z(1.0)
                        .color((1.0, 1.0, 1.0, 1.0))
                        .finish();
                }
            }
        }
    }
}
