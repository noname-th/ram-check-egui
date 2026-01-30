use ::egui::{Align2, Color32, CornerRadius, Id, Vec2, ViewportCommand};

use crate::system_info::MemoryInfo;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub const WIN_WIDTH: f32 = 340.0;

pub struct App {
    memory_info: MemoryInfo,
    last_update: Instant,
    countdown_timer: Option<Instant>,
    countdown_duration: Duration,
    error_message: Option<String>,

    first_render: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // โหลดฟอนต์ภาษาไทย
        let mut fonts = egui::FontDefinitions::default();

        // activate NotoSansThaiLooped
        fonts.font_data.insert(
            "NotoSanseThaiLooped".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/font/NotoSansThaiLooped-VariableFont_wdth,wght.ttf"
            ))),
        );

        // กำหนดฟอนต์เป็นฟอนต์หลักสำหรับ Proportional
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("NotoSanseThaiLooped".to_owned());
        cc.egui_ctx.set_fonts(fonts);

        let mut app = Self {
            memory_info: MemoryInfo::default(),
            last_update: Instant::now(),
            countdown_timer: None,
            countdown_duration: Duration::from_secs(10),
            error_message: None,

            first_render: true,
        };

        if let Err(e) = app.memory_info.update() {
            app.error_message = Some(e);
        }

        // เริ่มนับถอยหลังถ้ามีปัญหา
        if app.memory_info.has_problem() {
            // เริ่มนับถอยหลังจากเวลาปัจจุบัน
            app.countdown_timer = Some(Instant::now());
        }

        app
    }

    fn update_memory_info(&mut self) {
        // อัปเดตข้อมูล RAM ทุก ๆ 1 วินาที
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            if let Err(e) = self.memory_info.update() {
                self.error_message = Some(e);
            } else {
                self.error_message = None;
            }
            self.last_update = Instant::now();

            // ตรวจสอบว่ามีปัญหาหรือไม่
            if self.memory_info.has_problem() && self.countdown_timer.is_none() {
                self.countdown_timer = Some(Instant::now());
            } else if !self.memory_info.has_problem() {
                self.countdown_timer = None;
            }
        }
    }

    fn get_countdown_progress(&self) -> f32 {
        if let Some(start_time) = self.countdown_timer {
            let elapsed = start_time.elapsed().as_secs_f32();
            let total = self.countdown_duration.as_secs_f32();
            (elapsed / total).min(1.0)
        } else {
            0.0
        }
    }

    fn get_countdown_remaining(&self) -> u64 {
        if let Some(start_time) = self.countdown_timer {
            let elapsed = start_time.elapsed();
            if elapsed < self.countdown_duration {
                (self.countdown_duration - elapsed).as_secs()
            } else {
                0
            }
        } else {
            self.countdown_duration.as_secs()
        }
    }

    fn execute_fix_action(&mut self) {
        self.memory_info.fix_ram_issue();
    }
}

impl eframe::App for App {
    // ตั้งค่าพื้นหลังเป็นโปร่งใส
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Color32::TRANSPARENT.to_normalized_gamma_f32()
    }

    // ฟังก์ชันหลักสำหรับอัปเดต UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_memory_info();

        // ตรวจสอบว่านับถอยหลังเสร็จหรือยัง
        if let Some(start_time) = self.countdown_timer {
            if start_time.elapsed() >= self.countdown_duration {
                self.execute_fix_action();
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }

        // รีเพนต์หน้าต่างทุก 100 มิลลิวินาที
        ctx.request_repaint_after(Duration::from_millis(100));

        // สร้างหน้าต่างหลัก
        egui::Window::new("main_window")
            .title_bar(false)
            .collapsible(false)
            .movable(true)
            .anchor(Align2::CENTER_TOP, Vec2::ZERO)
            .max_width(WIN_WIDTH)
            .auto_sized()
            .resizable(false)
            .frame(
                egui::Frame::default()
                    .corner_radius(10.0)
                    .fill(ctx.style().visuals.window_fill()),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // กรอบหน้าต่างหลัก
                    egui::Frame::new()
                        .fill(ui.style().visuals.window_fill())
                        .stroke(ui.style().visuals.window_stroke())
                        .corner_radius(10.0)
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            // TitleBar
                            ui.vertical_centered(|ui| {
                                let title_rect = ui.clip_rect().with_max_y(40.0);

                                ui.painter().rect_filled(
                                    title_rect,
                                    CornerRadius {
                                        nw: 10.0 as u8,
                                        ne: 10.0 as u8,
                                        sw: 0.0 as u8,
                                        se: 0.0 as u8,
                                    },
                                    ui.style().visuals.window_stroke.color,
                                );
                                ui.style_mut().interaction.selectable_labels = false;
                                ui.heading("🖥  RAM Status Monitor");

                                // ทำให้ TitleBar สามารถลากย้ายหน้าต่างได้
                                let response = ui.interact(
                                    title_rect,
                                    Id::new("main_window_drag"),
                                    egui::Sense::drag(),
                                );
                                if response.dragged() {
                                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                                }
                            });
                            ui.add_space(10.0);

                            // ContentArea
                            ui.vertical(|ui| {
                                // InfoSection
                                ui.horizontal(|ui| {
                                    // AvatarIcon
                                    let status = if self.memory_info.has_problem() {
                                        ("⚠", egui::Color32::from_rgb(255, 165, 0))
                                    } else {
                                        ("✅", egui::Color32::from_rgb(46, 204, 113))
                                    };

                                    ui.label(
                                        egui::RichText::new(status.0).color(status.1).size(40.0),
                                    );
                                    ui.add_space(15.0);

                                    // InfoTexts
                                    ui.vertical(|ui| {
                                        // InfoHeader
                                        let header_text = if self.memory_info.has_problem() {
                                            "RAM มีปัญหา"
                                        } else {
                                            "RAM ทำงานปกติ"
                                        };
                                        ui.label(
                                            egui::RichText::new(header_text)
                                                .size(20.0)
                                                .color(status.1)
                                                .strong(),
                                        );

                                        // InfoDetail
                                        ui.label(format!(
                                            "RAM ที่ใช้งานได้: {:.2} GB / {:.2} GB",
                                            self.memory_info.total_visible_gb(),
                                            self.memory_info.total_installed_gb()
                                        ));
                                    });
                                });

                                ui.add_space(20.0);

                                // RestartSection (แสดงเฉพาะเมื่อมีปัญหา)
                                if self.memory_info.has_problem() {
                                    ui.vertical(|ui| {
                                        // RestartLabel
                                        let remaining = self.get_countdown_remaining();
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "จะดำเนินการรีสตาร์ทใน {} วินาที",
                                                remaining
                                            ))
                                            .size(16.0)
                                            .color(egui::Color32::from_rgb(231, 76, 60)),
                                        );

                                        ui.add_space(10.0);

                                        // ProgressBar
                                        let progress = self.get_countdown_progress();
                                        let progress_bar = egui::ProgressBar::new(progress)
                                            .show_percentage()
                                            .animate(true);

                                        ui.add(progress_bar);
                                    });

                                    ui.add_space(10.0);
                                }

                                // ButtonRow
                                ui.horizontal(|ui| {
                                    ui.add_space(ui.available_width() / 2.0 - 25.0);

                                    if self.countdown_timer.is_some() {
                                        // Button1 - ยกเลิก
                                        let cancel_button_response = ui
                                            .add_sized([50.0, 40.0], egui::Button::new("❌ ยกเลิก"));
                                        if cancel_button_response.clicked() {
                                            ctx.send_viewport_cmd(ViewportCommand::Close);
                                        }

                                        ui.add_space(10.0);

                                        // Button2 - ดำเนินการทันที
                                        let continue_button_response = ui.add_sized(
                                            [50.0, 40.0],
                                            egui::Button::new("⏩ รีสตาร์ททันที"),
                                        );
                                        if continue_button_response.clicked() {
                                            self.execute_fix_action();
                                            ctx.send_viewport_cmd(ViewportCommand::Close);
                                        }

                                        // โฟกัสปุ่มอัตโนมัติ
                                        ui.memory_mut(|mem| {
                                            if mem.focused().is_none() {
                                                mem.request_focus(cancel_button_response.id);
                                            }
                                        });
                                    } else {
                                        // Button - ปิดหน้าต่าง
                                        let close_button_response = ui.add_sized(
                                            [60.0, 40.0],
                                            egui::Button::new("✅ ปิดหน้าต่าง"),
                                        );
                                        if close_button_response.clicked() {
                                            ctx.send_viewport_cmd(ViewportCommand::Close);
                                        }
                                        // โฟกัสปุ่มอัตโนมัติ
                                        ui.memory_mut(|mem| {
                                            if mem.focused().is_none() {
                                                mem.request_focus(close_button_response.id);
                                            }
                                        });
                                    }
                                });
                            });

                            let full_size = ctx.used_size();

                            if self.first_render {
                                // กำหนดขนาดหน้าต่างให้พอดีกับเนื้อหาเมื่อรันครั้งแรก
                                if full_size.y >= ui.min_size().y && full_size.x >= ui.min_size().x
                                {
                                    ctx.send_viewport_cmd(ViewportCommand::InnerSize(
                                        ui.clip_rect().size(),
                                    ));
                                    self.first_render = false;
                                }

                                // โฟกัสหน้าต่างเมื่อรันครั้งแรก
                                ctx.send_viewport_cmd(ViewportCommand::Focus);
                                ctx.send_viewport_cmd(ViewportCommand::RequestUserAttention(
                                    ::egui::UserAttentionType::Critical,
                                ));
                            }
                        });
                });

                // แสดง error ถ้ามี
                if let Some(error) = &self.error_message {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
                }
            });
    }
}
