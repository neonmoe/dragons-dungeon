use crate::layers;
use crate::world::{Item, World};
use fae::{Font, GraphicsContext, Spritesheet};
use std::sync::Mutex;

pub const UI_AREA_WIDTH: f32 = 300.0;

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
    fn new(x: f32, y: f32, width: f32) -> MenuFlow {
        MenuFlow { x, y, width }
    }

    fn print(
        &mut self,
        ctx: &mut GraphicsContext,
        font: &Font,
        message: &str,
        font_size: f32,
        indent: f32,
    ) {
        if message.is_empty() {
            self.y += font_size + 2.0;
        } else if let Some(rect) = font
            .draw(ctx, message, self.x, self.y, font_size)
            .max_width(self.width)
            .visibility(false)
            .finish()
        {
            font.draw(ctx, message, self.x + indent, self.y, font_size)
                .color((1.0, 1.0, 1.0, 1.0))
                .max_width(self.width)
                .z(layers::UI_TEXT)
                .finish();
            self.y += rect.height + 2.0;
        }
    }

    fn print_header(&mut self, ctx: &mut GraphicsContext, font: &Font, message: &str) {
        self.print(ctx, font, message, 24.0, 0.0);
        self.y += 4.0;
    }

    fn print_stat(&mut self, ctx: &mut GraphicsContext, font: &Font, message: &str) {
        self.print(ctx, font, message, 18.0, 10.0);
        self.y += 2.0;
    }

    fn print_item(&mut self, ctx: &mut GraphicsContext, font: &Font, item: &Item) {
        self.print(ctx, font, item.name(), 18.0, 10.0);
        self.y += 6.0;
        self.print(ctx, font, item.description(), 12.0, 16.0);
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
        show_debug_info: bool,
    ) {
        let (width, height) = (ctx.width, ctx.height);

        let padding = 10.0;
        let menu_x = width - UI_AREA_WIDTH + padding;
        let menu_y = padding;
        spritesheet
            .draw(ctx)
            .coordinates((
                menu_x,
                menu_y,
                width - menu_x - padding,
                height - padding * 2.0,
            ))
            .color((0.2, 0.2, 0.2, 1.0))
            .z(layers::UI_BG)
            .finish();

        let mut menu = MenuFlow::new(menu_x + 10.0, menu_y + 10.0, UI_AREA_WIDTH);

        let player = world.player();
        let entities = world.entities();

        // Stats:
        if let (Some(damage), Some(health)) = (&player.damage, &player.health) {
            menu.print_header(ctx, font, "Stats:");
            menu.print_stat(ctx, font, &format!("Damage: {}", damage.0));
            menu.print_stat(
                ctx,
                font,
                &format!("Health: {}/{}", health.current, health.max),
            );
        }
        menu.space();

        // Inventory
        // TODO: Print descriptions of items
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
            menu.print_header(ctx, font, "Pickup [,]:");
            menu.print_item(ctx, font, &pickup);
        }
        menu.space();

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
