use crate::world::{Item, World};
use fae::{Font, GraphicsContext, Spritesheet};

pub const UI_AREA_WIDTH: f32 = 300.0;

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
    ) {
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

        let mut ui_text = String::new();
        let player = world.player();
        let entities = world.entities();

        // Stats:
        if let (Some(damage), Some(health)) = (&player.damage, &player.health) {
            ui_text.push_str(&format!(
                "Stats:\n Damage: {}\n Health: {}/{}\n\n",
                damage.0, health.current, health.max
            ));
        }

        // Inventory
        if let Some(inventory) = &player.inventory {
            if !inventory.is_empty() {
                ui_text.push_str("Inventory:\n");
                let mut print_item = |item: &Item| ui_text.push_str(&format!(" {}\n", item.name()));
                if let Some(item) = &inventory.item_left {
                    print_item(item);
                }
                if let Some(item) = &inventory.item_right {
                    print_item(item);
                }
                ui_text.push('\n');
            }
        }

        // Pickups:
        for pickup in entities
            .iter()
            .filter(|e| e.position.x == player.position.x && e.position.y == player.position.y)
            .filter_map(|e| e.drop)
        {
            ui_text.push_str(&format!("Pickup [,]:\n {}\n", pickup.name()));
        }

        font.draw(ctx, &ui_text, menu_cursor_x, menu_cursor_y, 24.0)
            .z(1.0)
            .color((0.9, 0.9, 0.9, 1.0))
            .finish();
    }
}
