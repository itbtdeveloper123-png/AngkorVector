use super::{CornerRadius, Tokens, Ui, VectorcraftApp, json, theme, vec2};
use egui::{Color32, Rect, Sense, Stroke, pos2};

/// The Home screen (`app.home`; with no document open, unless its preference is off).
pub(super) fn home(app: &mut VectorcraftApp, ui: &mut Ui, rect: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    // Adobe Illustrator dark canvas background
    ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(26, 26, 26));

    let sidebar_w = 180.0_f32;
    let sidebar_rect = Rect::from_min_max(rect.left_top(), pos2(rect.left() + sidebar_w, rect.bottom()));
    let main_rect = Rect::from_min_max(pos2(sidebar_rect.right(), rect.top()), rect.right_bottom());

    // Dividing border line
    ui.painter().line_segment(
        [sidebar_rect.right_top(), sidebar_rect.right_bottom()],
        Stroke::new(1.0, Color32::from_rgb(38, 38, 42)),
    );

    home_sidebar(app, ui, sidebar_rect);
    home_main(app, ui, main_rect, &t);
}

/// Adobe Illustrator left sidebar navigation.
fn home_sidebar(app: &mut VectorcraftApp, ui: &mut Ui, rect: Rect) {
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.add_space(18.0);
        let nav_w = (rect.width() - 36.0).max(120.0);

        // Adobe Illustrator "Ai" square badge
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            let (badge_r, _) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::hover());
            ui.painter().rect_filled(badge_r, CornerRadius::same(6), Color32::from_rgb(24, 18, 12));
            ui.painter().rect_stroke(
                badge_r,
                CornerRadius::same(6),
                Stroke::new(1.5, Color32::from_rgb(255, 122, 0)),
                egui::StrokeKind::Inside,
            );
            let ai_galley = ui.painter().layout_no_wrap(
                "Ai".to_string(),
                theme::semibold(17.0),
                Color32::from_rgb(255, 154, 54),
            );
            let text_pos = pos2(
                badge_r.center().x - ai_galley.size().x / 2.0,
                badge_r.center().y - ai_galley.size().y / 2.0,
            );
            ui.painter().galley(text_pos, ai_galley, Color32::from_rgb(255, 154, 54));
        });

        ui.add_space(22.0);

        // Navigation: "Home" (Active pill)
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            let (r, _resp) = ui.allocate_exact_size(vec2(nav_w, 32.0), Sense::click());
            ui.painter().rect_filled(r, CornerRadius::same(6), Color32::from_rgb(46, 46, 50));
            let g = ui.painter().layout_no_wrap(tl!("Home").to_string(), theme::semibold(13.5), Color32::WHITE);
            ui.painter().galley(pos2(r.left() + 12.0, r.center().y - g.size().y / 2.0), g, Color32::WHITE);
        });

        ui.add_space(4.0);

        // Navigation: "Learn" (Inactive tab)
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            let (r, resp) = ui.allocate_exact_size(vec2(nav_w, 28.0), Sense::click());
            let hover = resp.hovered();
            if hover {
                ui.painter().rect_filled(r, CornerRadius::same(6), Color32::from_white_alpha(15));
            }
            let col = if hover { Color32::WHITE } else { Color32::from_rgb(165, 165, 170) };
            let g = ui.painter().layout_no_wrap(tl!("Learn").to_string(), egui::FontId::proportional(13.5), col);
            ui.painter().galley(pos2(r.left() + 12.0, r.center().y - g.size().y / 2.0), g, col);
        });

        ui.add_space(24.0);

        // Action: "Create new" pill button
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            let (r, resp) = ui.allocate_exact_size(vec2(nav_w, 32.0), Sense::click());
            let hover = resp.hovered();
            let bg = if hover { Color32::from_white_alpha(25) } else { Color32::TRANSPARENT };
            ui.painter().rect_filled(r, CornerRadius::same(16), bg);
            ui.painter().rect_stroke(
                r,
                CornerRadius::same(16),
                Stroke::new(1.0, Color32::from_rgb(215, 215, 220)),
                egui::StrokeKind::Inside,
            );
            let g = ui.painter().layout_no_wrap(tl!("Create new").to_string(), theme::semibold(13.0), Color32::WHITE);
            ui.painter().galley(pos2(r.center().x - g.size().x / 2.0, r.center().y - g.size().y / 2.0), g, Color32::WHITE);
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "Create new"));
            if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                app.run("file.newDialog", json!({})).ok();
            }
        });

        ui.add_space(10.0);

        // Action: "Open" pill button
        ui.horizontal(|ui| {
            ui.add_space(18.0);
            let (r, resp) = ui.allocate_exact_size(vec2(nav_w, 32.0), Sense::click());
            let hover = resp.hovered();
            let bg = if hover { Color32::from_white_alpha(25) } else { Color32::TRANSPARENT };
            ui.painter().rect_filled(r, CornerRadius::same(16), bg);
            ui.painter().rect_stroke(
                r,
                CornerRadius::same(16),
                Stroke::new(1.0, Color32::from_rgb(215, 215, 220)),
                egui::StrokeKind::Inside,
            );
            let g = ui.painter().layout_no_wrap(tl!("Open").to_string(), theme::semibold(13.0), Color32::WHITE);
            ui.painter().galley(pos2(r.center().x - g.size().x / 2.0, r.center().y - g.size().y / 2.0), g, Color32::WHITE);
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, "Open"));
            if resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                app.run("file.open", json!({})).ok();
            }
        });

        // Bottom link: "What's new"
        let bottom_y = rect.bottom() - 32.0;
        let whats_new_rect = Rect::from_min_size(pos2(rect.left() + 18.0, bottom_y), vec2(nav_w, 24.0));
        let resp = ui.allocate_rect(whats_new_rect, Sense::click());
        let hover = resp.hovered();
        let col = if hover { Color32::WHITE } else { Color32::from_rgb(140, 140, 145) };
        let g = ui.painter().layout_no_wrap(tl!("What's new").to_string(), egui::FontId::proportional(12.0), col);
        ui.painter().galley(whats_new_rect.left_top(), g, col);
    });
}

/// Adobe Illustrator main content area with header, hero suggestion cards, quick presets and recent files.
fn home_main(app: &mut VectorcraftApp, ui: &mut Ui, rect: Rect, _t: &Tokens) {
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(egui::Layout::top_down(egui::Align::Min)));
    egui::ScrollArea::vertical().id_salt("home-scroll").auto_shrink([false, false]).show(&mut child, |ui| {
        let max_w = (ui.available_width() - 48.0).min(960.0).max(300.0);
        let margin_x = ((ui.available_width() - max_w) / 2.0).max(24.0);

        ui.add_space(28.0);
        ui.horizontal(|ui| {
            ui.add_space(margin_x);
            ui.allocate_ui_with_layout(vec2(max_w, 0.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_width(max_w);

                // Check toggle state for suggestions
                let id = ui.make_persistent_id("home_hide_suggestions");
                let mut hide_suggestions = ui.data_mut(|d| d.get_temp::<bool>(id).unwrap_or(false));

                // 1. Header Row
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(tl!("Welcome to Illustrator"))
                            .font(theme::semibold(19.0))
                            .color(Color32::WHITE),
                    );
                    ui.add_space(6.0);
                    // (i) info icon
                    let (info_r, info_resp) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                    ui.painter().circle_stroke(info_r.center(), 7.0, Stroke::new(1.0, Color32::from_rgb(120, 120, 125)));
                    let ig = ui.painter().layout_no_wrap("i".to_string(), theme::semibold(10.0), Color32::from_rgb(160, 160, 165));
                    ui.painter().galley(pos2(info_r.center().x - ig.size().x / 2.0, info_r.center().y - ig.size().y / 2.0), ig, Color32::from_rgb(160, 160, 165));
                    info_resp.on_hover_text(tl!("Explore tutorials, tools, and presets to get started"));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let label = if hide_suggestions {
                            tl!("v Show suggestions")
                        } else {
                            tl!("^ Hide suggestions")
                        };
                        let resp = ui.add(egui::Button::new(
                            egui::RichText::new(label).size(12.0).color(Color32::from_rgb(160, 160, 165))
                        ).frame(false));
                        if resp.clicked() {
                            hide_suggestions = !hide_suggestions;
                            ui.data_mut(|d| d.insert_temp(id, hide_suggestions));
                        }
                    });
                });

                ui.add_space(14.0);

                // 2. Hero Suggestions (collapsible)
                if !hide_suggestions {
                    let total_w = ui.available_width();
                    let card1_w = ((total_w - 14.0) * 0.62).max(200.0);
                    let card2_w = (total_w - card1_w - 14.0).max(180.0);
                    let hero_h = 180.0_f32;

                    ui.horizontal(|ui| {
                        // Hero Card 1: Using the shape tools
                        let (c1_rect, _) = ui.allocate_exact_size(vec2(card1_w, hero_h), Sense::hover());
                        hero_card_shape_tools(app, ui, c1_rect);

                        ui.add_space(14.0);

                        // Hero Card 2: Get a feel for Illustrator
                        let (c2_rect, _) = ui.allocate_exact_size(vec2(card2_w, hero_h), Sense::hover());
                        hero_card_feel_illustrator(ui, c2_rect);
                    });
                    ui.add_space(20.0);
                }

                // 3. "Start a new file fast" container
                presets_section(app, ui);

                // 4. Recent Files
                recent(app, ui);
            });
        });
        ui.add_space(40.0);
    });
}

/// Hero Card 1: "First up: Using the shape tools" with custom vector illustration.
fn hero_card_shape_tools(app: &mut VectorcraftApp, ui: &mut Ui, rect: Rect) {
    if !ui.is_rect_visible(rect) {
        return;
    }
    // Card background
    ui.painter().rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(36, 36, 39));

    // Left illustration graphic (~44% width)
    let banner_w = (rect.width() * 0.44).min(rect.width() - 150.0);
    let banner_r = Rect::from_min_max(rect.left_top(), pos2(rect.left() + banner_w, rect.bottom()));
    let radius = CornerRadius { nw: 8, sw: 8, ne: 0, se: 0 };
    ui.painter().rect_filled(banner_r, radius, Color32::from_rgb(0, 130, 150));

    // 1. Tool palette on left
    let tb_r = Rect::from_min_size(banner_r.left_top() + vec2(12.0, 24.0), vec2(22.0, 84.0));
    ui.painter().rect_filled(tb_r, CornerRadius::same(3), Color32::from_rgb(30, 36, 40));
    for i in 0..4 {
        let slot = Rect::from_min_size(tb_r.left_top() + vec2(2.0, 2.0 + i as f32 * 20.0), vec2(18.0, 18.0));
        if i == 2 {
            ui.painter().rect_filled(slot, CornerRadius::same(2), Color32::from_rgb(0, 160, 180));
            ui.painter().circle_stroke(slot.center(), 4.5, Stroke::new(1.2, Color32::WHITE));
        } else {
            ui.painter().circle_filled(slot.center(), 2.0, Color32::from_white_alpha(100));
        }
    }

    // 2. Speech bubble tooltip pointing from tool
    let bubble_r = Rect::from_min_size(tb_r.right_top() + vec2(6.0, 38.0), vec2(44.0, 24.0));
    ui.painter().rect_filled(bubble_r, CornerRadius::same(4), Color32::from_rgb(60, 175, 195));
    ui.painter().line_segment(
        [bubble_r.left_top() + vec2(8.0, 8.0), bubble_r.left_top() + vec2(36.0, 8.0)],
        Stroke::new(1.5, Color32::WHITE),
    );
    ui.painter().line_segment(
        [bubble_r.left_top() + vec2(8.0, 15.0), bubble_r.left_top() + vec2(26.0, 15.0)],
        Stroke::new(1.5, Color32::from_white_alpha(200)),
    );

    // 3. Dashed bounding box with handles
    let box_min = banner_r.left_top() + vec2(tb_r.right() - banner_r.left() + 20.0, 18.0);
    let box_max = banner_r.right_bottom() - vec2(10.0, 22.0);
    if box_max.x > box_min.x && box_max.y > box_min.y {
        let box_r = Rect::from_min_max(box_min, box_max);
        let stroke = Stroke::new(1.0, Color32::from_white_alpha(180));
        let step = 6.0;
        let mut x = box_r.left();
        while x < box_r.right() {
            ui.painter().line_segment([pos2(x, box_r.top()), pos2((x + 3.0).min(box_r.right()), box_r.top())], stroke);
            ui.painter().line_segment([pos2(x, box_r.bottom()), pos2((x + 3.0).min(box_r.right()), box_r.bottom())], stroke);
            x += step;
        }
        let mut y = box_r.top();
        while y < box_r.bottom() {
            ui.painter().line_segment([pos2(box_r.left(), y), pos2(box_r.left(), (y + 3.0).min(box_r.bottom()))], stroke);
            ui.painter().line_segment([pos2(box_r.right(), y), pos2(box_r.right(), (y + 3.0).min(box_r.bottom()))], stroke);
            y += step;
        }

        let handle_stroke = Stroke::new(1.0, Color32::from_rgb(0, 130, 150));
        for p in [box_r.left_top(), box_r.right_top(), box_r.left_bottom(), box_r.right_bottom()] {
            let hr = Rect::from_center_size(p, vec2(5.0, 5.0));
            ui.painter().rect_filled(hr, CornerRadius::ZERO, Color32::WHITE);
            ui.painter().rect_stroke(hr, CornerRadius::ZERO, handle_stroke, egui::StrokeKind::Inside);
        }

        // 4. Vector circle with anchor points & bezier handles
        let circle_center = box_r.center() + vec2(-4.0, 8.0);
        let circle_radius = 24.0;
        ui.painter().circle_stroke(circle_center, circle_radius, Stroke::new(1.8, Color32::WHITE));
        let top_pt = circle_center - vec2(0.0, circle_radius);
        ui.painter().line_segment([top_pt - vec2(18.0, 0.0), top_pt + vec2(18.0, 0.0)], Stroke::new(1.0, Color32::WHITE));
        ui.painter().circle_filled(top_pt - vec2(18.0, 0.0), 2.5, Color32::WHITE);
        ui.painter().circle_filled(top_pt + vec2(18.0, 0.0), 2.5, Color32::WHITE);
        let hr = Rect::from_center_size(top_pt, vec2(5.0, 5.0));
        ui.painter().rect_filled(hr, CornerRadius::ZERO, Color32::from_rgb(0, 130, 150));
        ui.painter().rect_stroke(hr, CornerRadius::ZERO, Stroke::new(1.0, Color32::WHITE), egui::StrokeKind::Inside);
    }

    // 5. Stylized orange fluid blob on right of banner
    let blob_center = pos2(banner_r.right() - 18.0, banner_r.top() + 38.0);
    ui.painter().circle_filled(blob_center, 18.0, Color32::from_rgb(255, 140, 20));
    ui.painter().circle_filled(blob_center + vec2(10.0, 12.0), 12.0, Color32::from_rgb(255, 140, 20));

    // Right Content Area
    let right_rect = Rect::from_min_max(pos2(banner_r.right() + 18.0, rect.top() + 20.0), rect.right_bottom() - vec2(18.0, 16.0));
    ui.scope_builder(egui::UiBuilder::new().max_rect(right_rect), |ui| {
        ui.label(
            egui::RichText::new(tl!("First up: Using the shape tools"))
                .font(theme::semibold(15.0))
                .color(Color32::WHITE),
        );
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(tl!("This speedy series teaches you how to use Illustrator."))
                .size(12.5)
                .color(Color32::from_rgb(175, 175, 180)),
        );
        ui.add_space(20.0);
        ui.horizontal(|ui| {
            let (btn_r, btn_resp) = ui.allocate_exact_size(vec2(105.0, 28.0), Sense::click());
            let hover = btn_resp.hovered();
            let bg = if hover { Color32::from_white_alpha(25) } else { Color32::TRANSPARENT };
            ui.painter().rect_filled(btn_r, CornerRadius::same(14), bg);
            ui.painter().rect_stroke(
                btn_r,
                CornerRadius::same(14),
                Stroke::new(1.0, Color32::from_rgb(215, 215, 220)),
                egui::StrokeKind::Inside,
            );
            let g = ui.painter().layout_no_wrap(tl!("Make an icon").to_string(), theme::semibold(12.0), Color32::WHITE);
            ui.painter().galley(pos2(btn_r.center().x - g.size().x / 2.0, btn_r.center().y - g.size().y / 2.0), g, Color32::WHITE);
            if btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
                app.run("file.new", json!({ "width": 256.0, "height": 256.0, "units": "Pixels", "name": "Icon" })).ok();
            }

            ui.add_space(10.0);
            let link_resp = ui.add(egui::Label::new(
                egui::RichText::new(tl!("Browse more how-tos"))
                    .size(12.0)
                    .color(Color32::from_rgb(60, 160, 240)),
            ).sense(Sense::click()));
            link_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
        });
    });
}

/// Hero Card 2: "Get a feel for Illustrator" with stylized illustration.
fn hero_card_feel_illustrator(ui: &mut Ui, rect: Rect) {
    if !ui.is_rect_visible(rect) {
        return;
    }
    // Deep cosmic background
    ui.painter().rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(42, 24, 18));

    // Stylized orange sunset / glowing cosmic artwork
    let sun_center = rect.center() - vec2(0.0, 20.0);
    ui.painter().circle_filled(sun_center, 45.0, Color32::from_rgb(180, 80, 25));
    ui.painter().circle_filled(sun_center, 30.0, Color32::from_rgb(235, 130, 35));

    // Perspective vector grid lines
    let bottom_y = rect.bottom() - 40.0;
    for offset in [-60.0, -30.0, 0.0, 30.0, 60.0] {
        ui.painter().line_segment(
            [sun_center + vec2(offset * 0.2, 10.0), pos2(rect.center().x + offset * 1.5, bottom_y)],
            Stroke::new(1.0, Color32::from_white_alpha(50)),
        );
    }
    // Visor silhouette
    let visor_r = Rect::from_center_size(sun_center + vec2(0.0, 10.0), vec2(36.0, 22.0));
    ui.painter().rect_filled(visor_r, CornerRadius::same(8), Color32::from_rgb(26, 14, 10));
    ui.painter().rect_stroke(visor_r, CornerRadius::same(8), Stroke::new(1.5, Color32::from_rgb(255, 170, 70)), egui::StrokeKind::Inside);

    // Bottom gradient overlay for readability
    let overlay_r = Rect::from_min_max(pos2(rect.left(), rect.bottom() - 55.0), rect.right_bottom());
    ui.painter().rect_filled(overlay_r, CornerRadius { nw: 0, ne: 0, sw: 8, se: 8 }, Color32::from_black_alpha(180));

    // Bottom row with text and button
    ui.scope_builder(egui::UiBuilder::new().max_rect(overlay_r), |ui| {
        ui.add_space(14.0);
        ui.horizontal(|ui| {
            ui.add_space(14.0);
            ui.label(
                egui::RichText::new(tl!("Get a feel for Illustrator"))
                    .font(theme::semibold(13.0))
                    .color(Color32::WHITE),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                let (btn_r, btn_resp) = ui.allocate_exact_size(vec2(90.0, 24.0), Sense::click());
                let hover = btn_resp.hovered();
                let bg = if hover { Color32::from_white_alpha(50) } else { Color32::from_black_alpha(120) };
                ui.painter().rect_filled(btn_r, CornerRadius::same(12), bg);
                ui.painter().rect_stroke(
                    btn_r,
                    CornerRadius::same(12),
                    Stroke::new(1.0, Color32::from_rgb(220, 220, 225)),
                    egui::StrokeKind::Inside,
                );
                let g = ui.painter().layout_no_wrap(tl!("▶ Play video").to_string(), theme::semibold(11.0), Color32::WHITE);
                ui.painter().galley(pos2(btn_r.center().x - g.size().x / 2.0, btn_r.center().y - g.size().y / 2.0), g, Color32::WHITE);
                btn_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
            });
        });
    });
}

/// Adobe Illustrator "Start a new file fast" container with 6 presets.
fn presets_section(app: &mut VectorcraftApp, ui: &mut Ui) {
    let avail_w = ui.available_width();
    let container_h = 158.0_f32;
    let (container_r, _) = ui.allocate_exact_size(vec2(avail_w, container_h), Sense::hover());
    ui.painter().rect_filled(container_r, CornerRadius::same(8), Color32::from_rgb(33, 33, 36));

    ui.scope_builder(egui::UiBuilder::new().max_rect(container_r), |ui| {
        ui.add_space(14.0);
        // Header
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new(tl!("Start a new file fast"))
                    .font(theme::semibold(14.0))
                    .color(Color32::WHITE),
            );
        });

        ui.add_space(12.0);

        // 6 Preset cards horizontally
        let card_w = ((container_r.width() - 40.0) / 6.0).max(80.0);
        let card_h = 92.0_f32;

        ui.horizontal(|ui| {
            ui.add_space(20.0);

            // 1. Letter
            if preset_card(ui, card_w, card_h, "Letter", "612 × 792 pt", PresetIcon::Letter).clicked() {
                app.run("file.new", json!({ "preset": "Letter" })).ok();
            }

            // 2. Postcard
            if preset_card(ui, card_w, card_h, "Postcard", "288 × 560 pt", PresetIcon::Postcard).clicked() {
                app.run("file.new", json!({ "preset": "Postcard" })).ok();
            }

            // 3. Common
            if preset_card(ui, card_w, card_h, "Common", "1366 × 768 px", PresetIcon::Common).clicked() {
                app.run("file.new", json!({ "width": 1366.0, "height": 768.0, "units": "Pixels", "name": "Common" })).ok();
            }

            // 4. iPhone X
            if preset_card(ui, card_w, card_h, "iPhone X", "1125 × 2436 px", PresetIcon::IPhoneX).clicked() {
                app.run("file.new", json!({ "width": 1125.0, "height": 2436.0, "units": "Pixels", "name": "iPhone X" })).ok();
            }

            // 5. HDV/HDTV 1080
            if preset_card(ui, card_w, card_h, "HDV/HDTV 1080", "1920 × 1080 px", PresetIcon::Hdv).clicked() {
                if app.run("file.new", json!({ "preset": "HDTV 1080p" })).is_err() {
                    app.run("file.new", json!({ "width": 1920.0, "height": 1080.0, "units": "Pixels", "name": "HDV/HDTV 1080" })).ok();
                }
            }

            // 6. More Presets
            if preset_card(ui, card_w, card_h, "More Presets", "View All", PresetIcon::More).clicked() {
                app.run("file.newDialog", json!({})).ok();
            }
        });
    });
}

#[derive(Clone, Copy)]
enum PresetIcon {
    Letter,
    Postcard,
    Common,
    IPhoneX,
    Hdv,
    More,
}

fn preset_card(ui: &mut Ui, width: f32, height: f32, name: &str, dim: &str, icon: PresetIcon) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    if ui.is_rect_visible(rect) {
        let hover = resp.hovered();
        if hover {
            ui.painter().rect_filled(rect, CornerRadius::same(6), Color32::from_white_alpha(15));
        }

        let icon_center = pos2(rect.center().x, rect.top() + 24.0);
        let stroke_color = if hover { Color32::WHITE } else { Color32::from_rgb(205, 205, 210) };
        let stroke = Stroke::new(1.3, stroke_color);

        match icon {
            PresetIcon::Letter => {
                let w = 24.0;
                let h = 32.0;
                let fold = 7.0;
                let top_l = pos2(icon_center.x - w / 2.0, icon_center.y - h / 2.0);
                let top_fold = pos2(icon_center.x + w / 2.0 - fold, icon_center.y - h / 2.0);
                let right_fold = pos2(icon_center.x + w / 2.0, icon_center.y - h / 2.0 + fold);
                let bot_r = pos2(icon_center.x + w / 2.0, icon_center.y + h / 2.0);
                let bot_l = pos2(icon_center.x - w / 2.0, icon_center.y + h / 2.0);

                ui.painter().line_segment([top_l, top_fold], stroke);
                ui.painter().line_segment([top_fold, right_fold], stroke);
                ui.painter().line_segment([right_fold, bot_r], stroke);
                ui.painter().line_segment([bot_r, bot_l], stroke);
                ui.painter().line_segment([bot_l, top_l], stroke);

                let fold_corner = pos2(top_fold.x, right_fold.y);
                ui.painter().line_segment([top_fold, fold_corner], stroke);
                ui.painter().line_segment([fold_corner, right_fold], stroke);
            }
            PresetIcon::Postcard => {
                let w = 34.0;
                let h = 22.0;
                let r = Rect::from_center_size(icon_center, vec2(w, h));
                ui.painter().rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
                let stamp = Rect::from_min_size(pos2(r.right() - 8.0, r.top() + 3.0), vec2(5.0, 5.0));
                ui.painter().rect_stroke(stamp, CornerRadius::ZERO, stroke, egui::StrokeKind::Inside);
                ui.painter().line_segment([pos2(r.center().x, r.top() + 3.0), pos2(r.center().x, r.bottom() - 3.0)], stroke);
            }
            PresetIcon::Common => {
                let w = 34.0;
                let h = 24.0;
                let r = Rect::from_center_size(icon_center, vec2(w, h));
                ui.painter().rect_stroke(r, CornerRadius::same(3), stroke, egui::StrokeKind::Inside);
                ui.painter().line_segment([pos2(r.left(), r.top() + 6.0), pos2(r.right(), r.top() + 6.0)], stroke);
                ui.painter().circle_filled(pos2(r.left() + 4.0, r.top() + 3.0), 1.0, stroke_color);
                ui.painter().circle_filled(pos2(r.left() + 7.5, r.top() + 3.0), 1.0, stroke_color);
                ui.painter().circle_filled(pos2(r.left() + 11.0, r.top() + 3.0), 1.0, stroke_color);
                let globe_center = pos2(r.center().x, r.center().y + 3.0);
                ui.painter().circle_stroke(globe_center, 5.0, stroke);
                ui.painter().line_segment([globe_center - vec2(5.0, 0.0), globe_center + vec2(5.0, 0.0)], stroke);
            }
            PresetIcon::IPhoneX => {
                let w = 18.0;
                let h = 32.0;
                let r = Rect::from_center_size(icon_center, vec2(w, h));
                ui.painter().rect_stroke(r, CornerRadius::same(4), stroke, egui::StrokeKind::Inside);
                ui.painter().line_segment([pos2(r.center().x - 3.0, r.top() + 2.5), pos2(r.center().x + 3.0, r.top() + 2.5)], stroke);
                ui.painter().line_segment([pos2(r.center().x - 4.0, r.bottom() - 2.5), pos2(r.center().x + 4.0, r.bottom() - 2.5)], stroke);
            }
            PresetIcon::Hdv => {
                let w = 34.0;
                let h = 22.0;
                let r = Rect::from_center_size(icon_center, vec2(w, h));
                ui.painter().rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
                let p1 = icon_center + vec2(-3.0, -4.0);
                let p2 = icon_center + vec2(4.0, 0.0);
                let p3 = icon_center + vec2(-3.0, 4.0);
                ui.painter().line_segment([p1, p2], stroke);
                ui.painter().line_segment([p2, p3], stroke);
                ui.painter().line_segment([p3, p1], stroke);
            }
            PresetIcon::More => {
                let radius = 13.0;
                ui.painter().circle_stroke(icon_center, radius, stroke);
                ui.painter().circle_filled(icon_center - vec2(5.0, 0.0), 1.5, stroke_color);
                ui.painter().circle_filled(icon_center, 1.5, stroke_color);
                ui.painter().circle_filled(icon_center + vec2(5.0, 0.0), 1.5, stroke_color);
            }
        }

        // Title text
        let title_str = if matches!(icon, PresetIcon::More) {
            name.to_string()
        } else {
            format!("{name} ˅")
        };
        let title_col = if hover { Color32::WHITE } else { Color32::from_rgb(220, 220, 225) };
        let tg = ui.painter().layout_no_wrap(title_str, theme::semibold(12.5), title_col);
        ui.painter().galley(pos2(rect.center().x - tg.size().x / 2.0, rect.top() + 50.0), tg, title_col);

        // Subtitle / Dimensions
        let sub_col = Color32::from_rgb(145, 145, 150);
        let sg = ui.painter().layout_no_wrap(dim.to_string(), egui::FontId::proportional(11.0), sub_col);
        ui.painter().galley(pos2(rect.center().x - sg.size().x / 2.0, rect.top() + 68.0), sg, sub_col);
    }

    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The Home screen's Recent Files (#663): the first of File › Open Recent Files, each its name and
/// folder; a click opens it. Nothing when there are none.
fn recent(app: &mut VectorcraftApp, ui: &mut Ui) {
    const SHOWN: usize = 6;
    let files: Vec<String> = crate::io::recent_files(app).iter().take(SHOWN).cloned().collect();
    if files.is_empty() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    ui.add_space(28.0);
    ui.label(egui::RichText::new(tl!("Recent Files")).font(theme::semibold(14.0)).color(t.text));
    ui.add_space(8.0);
    for path in files {
        if recent_row(ui, &path).clicked() {
            // A failure is reported to the user there.
            let _ = crate::io::open_recent_reporting(app, &path);
        }
    }
}

/// One Recent Files row across the Home screen: history icon, name (accent) and folder (dim,
/// truncated). The whole row is one button that lights up under the pointer; its tooltip is the
/// full path.
fn recent_row(ui: &mut Ui, path: &str) -> egui::Response {
    const PAD: f32 = 6.0;
    const ICON: f32 = 16.0;
    let t = Tokens::get(ui.ctx());
    let p = std::path::Path::new(path);
    let name = p.file_name().map_or_else(|| path.to_string(), |n| n.to_string_lossy().into_owned());
    let folder = p.parent().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 26.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        if resp.hovered() {
            ui.painter().rect_filled(rect, CornerRadius::same(4), t.hover);
        }
        let icon = egui::Rect::from_center_size(egui::pos2(rect.left() + PAD + ICON / 2.0, rect.center().y), vec2(ICON, ICON));
        crate::icons::paint(ui, "history", icon, t.icon);
        let x = icon.right() + 8.0;
        let name_galley = egui::WidgetText::from(egui::RichText::new(&name).size(13.0).color(t.accent)).into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            (rect.right() - PAD - x).max(0.0),
            egui::TextStyle::Body,
        );
        let folder_x = x + name_galley.size().x + 10.0;
        ui.painter().galley(egui::pos2(x, rect.center().y - name_galley.size().y / 2.0), name_galley, t.accent);
        let room = rect.right() - PAD - folder_x;
        if room > 0.0 && !folder.is_empty() {
            let folder_galley = egui::WidgetText::from(egui::RichText::new(&folder).size(12.0).color(t.text_dim)).into_galley(
                ui,
                Some(egui::TextWrapMode::Truncate),
                room,
                egui::TextStyle::Body,
            );
            ui.painter().galley(egui::pos2(folder_x, rect.center().y - folder_galley.size().y / 2.0), folder_galley, t.text_dim);
        }
    }
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, &name));
    resp.on_hover_text(path).on_hover_cursor(egui::CursorIcon::PointingHand)
}
