#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // 在 release 模式下使用 windows subsystem

use eframe::egui;
mod chars;

struct PostChecklist {
    items: Vec<(String, bool)>,
    current_step: usize, // 当前可操作的步骤
}

impl Default for PostChecklist {
    fn default() -> Self {
        Self {
            items: vec![
                ("更新本地库".to_string(), false),
                ("新建一个分支来工作".to_string(), false),
                ("撰写文章，包括代码和图形".to_string(), false),
                ("中途多次提交commit和把本地代码push到服务器".to_string(), false),
                ("把文章的draft标志设置为false".to_string(), false),
                ("commit所有工作".to_string(), false),
                ("把分支切换到主分支".to_string(), false),
                ("合并工作分支".to_string(), false),
                ("push代码库，使得github action能够渲染新撰写的文章".to_string(), false),
            ],
            current_step: 0,
        }
    }
}

impl PostChecklist {
    fn update_current_step(&mut self) {
        // 找到第一个未完成的步骤
        self.current_step = self.items.iter()
            .position(|(_, checked)| !checked)
            .unwrap_or(self.items.len());
    }

    fn uncheck_following_steps(&mut self, from_step: usize) {
        // 取消从指定步骤开始的所有后续步骤
        for (_, checked) in self.items.iter_mut().skip(from_step) {
            *checked = false;
        }
        // 更新当前步骤
        self.update_current_step();
    }
}

impl eframe::App for PostChecklist {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut steps_to_uncheck = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("GitHub Pages 文章发布清单");
                ui.add_space(20.0);

                // 先计算当前步骤
                let current_step = self.current_step;
                
                // 然后遍历项目
                for (i, (item, checked)) in self.items.iter_mut().enumerate() {
                    let can_check = i == current_step || i < current_step;
                    let old_checked = *checked;
                    
                    ui.horizontal(|ui| {
                        let text = format!("{}. {}", i + 1, item);
                        let checkbox = egui::Checkbox::new(checked, text);
                        ui.add_enabled(can_check, checkbox);
                    });
                    ui.add_space(5.0);

                    // 如果选项状态从选中变为未选中，记录需要取消的步骤
                    if old_checked && !*checked {
                        steps_to_uncheck = Some(i + 1);
                    }
                }

                ui.add_space(20.0);
                if ui.button("重置所有选项").clicked() {
                    for (_, checked) in &mut self.items {
                        *checked = false;
                    }
                    self.current_step = 0;
                }
            });
        });

        // 在循环外部处理需要取消的步骤
        if let Some(step) = steps_to_uncheck {
            self.uncheck_following_steps(step);
        }

        // 检查是否有步骤状态发生变化
        let mut step_changed = false;
        for (i, (_, checked)) in self.items.iter().enumerate() {
            if i == self.current_step && *checked {
                step_changed = true;
                break;
            }
        }

        // 如果有步骤状态变化，更新当前步骤
        if step_changed {
            self.update_current_step();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("GitHub Pages 文章发布清单")
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "GitHub Pages 文章发布清单",
        options,
        Box::new(|cc| {
            // 设置中文字体
            let mut fonts = egui::FontDefinitions::default();
            
            // 添加自定义字体
            fonts.font_data.insert(
                "custom_font".to_owned(),
                egui::FontData::from_static(include_bytes!("../fonts/custom_font.ttf")),
            );

            // 将字体添加到所有字体族
            fonts.families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "custom_font".to_owned());

            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx.set_pixels_per_point(1.5);
            Box::new(PostChecklist::default())
        }),
    )
}