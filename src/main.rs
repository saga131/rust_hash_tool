mod crypto;

use eframe::egui;
use crypto::{HashAlgorithm, SaltMode, calculate_hash, calculate_complex_hashes};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 800.0]) // Increased height for split view
            .with_min_inner_size([400.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rust 哈希工具箱",
        options,
        Box::new(|cc| {
            // 设置字体以支持中文
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(HashApp::default()))
        }),
    )
}

#[derive(Clone, PartialEq, Eq)]
enum CustomBlock {
    Password,
    Salt,
    Literal(String),
    Hash(HashAlgorithm, Vec<CustomBlock>),
}

impl CustomBlock {
    fn name(&self) -> String {
        match self {
            CustomBlock::Password => "$pass".to_string(),
            CustomBlock::Salt => "$salt".to_string(),
            CustomBlock::Literal(s) => format!("\"{}\"", s),
            CustomBlock::Hash(algo, _) => format!("{}(...)", algo.name()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveTool {
    Encryption,
    Inference,
}

struct HashApp {
    active_tool: ActiveTool,
    input_text: String,
    salt_text: String,
    // salt_mode: SaltMode, // Removed in favor of bulk view
    // selected_algo: HashAlgorithm, // Removed in favor of bulk view
    output_text: String,
    bulk_results: Vec<(String, String)>,
    search_query: String,
    compare_target: String,
    auto_calculate: bool,
    // 自定义加密块
    custom_blocks: Vec<CustomBlock>,
    literal_input: String,
    nested_algo_selection: HashAlgorithm,
    dragging_source: Option<usize>,
}

impl Default for HashApp {
    fn default() -> Self {
        Self {
            active_tool: ActiveTool::Encryption,
            input_text: String::new(),
            salt_text: String::new(),
            // salt_mode: SaltMode::None,
            // selected_algo: HashAlgorithm::Md5,
            output_text: String::new(),
            bulk_results: Vec::new(),
            search_query: String::new(),
            compare_target: String::new(),
            auto_calculate: true,
            custom_blocks: vec![CustomBlock::Password, CustomBlock::Salt],
            literal_input: String::new(),
            nested_algo_selection: HashAlgorithm::Md5,
            dragging_source: None,
        }
    }
}

impl HashApp {
    fn render_blocks(
        input_text: &str,
        salt_text: &str,
        ui: &mut egui::Ui,
        blocks: &mut Vec<CustomBlock>,
        changed: &mut bool,
        list_id: egui::Id,
    ) {
        let mut to_remove = None;
        let mut swap_target = None;
        
        // 从内存中获取当前列表的拖拽状态
        let mut dragging_source = ui.data_mut(|d| d.get_temp::<Option<usize>>(list_id)).unwrap_or(None);

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 4.0;

            let len = blocks.len();
            for i in 0..len {
                let color = match &blocks[i] {
                    CustomBlock::Password => egui::Color32::from_rgb(76, 151, 255), // Scratch 蓝色
                    CustomBlock::Salt => egui::Color32::from_rgb(255, 171, 25),     // Scratch 橙色
                    CustomBlock::Literal(_) => egui::Color32::from_rgb(89, 192, 89), // Scratch 绿色
                    CustomBlock::Hash(_, _) => egui::Color32::from_rgb(153, 102, 255), // Scratch 紫色
                };

                ui.horizontal(|ui| {
                    // 1. 拖动手柄 (独立感应区)
                    let _handle_id = ui.make_persistent_id(("handle", i, blocks.as_ptr()));
                    let (rect, response) = ui.allocate_at_least(egui::vec2(24.0, 30.0), egui::Sense::drag());
                    
                    // 绘制手柄视觉
                    let visuals = ui.style().interact(&response);
                    ui.painter().rect_filled(rect.shrink(2.0), 4.0, visuals.bg_fill);
                    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "☰", egui::FontId::proportional(16.0), egui::Color32::WHITE);

                    // 动态拖拽逻辑
                    if response.drag_started() {
                        dragging_source = Some(i);
                    }
                    if response.dragged() {
                        if let Some(_source_idx) = dragging_source {
                            // 检查鼠标当前位置是否在其他积木的区域内
                            if let Some(_pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                // 占位逻辑，实际交换在下方
                            }
                        }
                    }
                    if response.drag_stopped() {
                        dragging_source = None;
                    }
                    
                    // 更简单的实时交换逻辑：
                    // 如果正在拖拽某个积木，我们检查鼠标位置相对于当前积木的位置
                    if let Some(source_idx) = dragging_source {
                        if source_idx != i {
                            if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                                // 如果鼠标悬停在当前积木的手柄区域，就交换
                                if rect.contains(pointer_pos) {
                                    swap_target = Some((source_idx, i));
                                }
                            }
                        }
                    }

                    // 2. 积木主体
                    let block = &mut blocks[i];
                    match block {
                        CustomBlock::Hash(algo, inner) => {
                            // C-Block 形状实现 (全包含)
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0; // 关键修复：移除垂直间距，使三部分无缝连接

                                // 顶部栏
                                let top_frame = egui::Frame::none()
                                    .fill(color)
                                    .rounding(egui::Rounding { nw: 10.0, ne: 10.0, sw: 0.0, se: 0.0 })
                                    .inner_margin(egui::Margin::symmetric(8.0, 6.0));
                                
                                top_frame.show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(format!("计算 {}", algo.name())).color(egui::Color32::WHITE).strong());
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.button("x").clicked() { to_remove = Some(i); }
                                        });
                                    });
                                });

                                // 中间内容区 (左侧脊柱 + 内部积木)
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    
                                    // 动态计算脊柱高度：先记录起始位置
                                    let spine_start_pos = ui.cursor().min;
                                    let spine_width = 16.0;
                                    
                                    // 预留脊柱空间
                                    ui.add_space(spine_width);

                                    // 内部容器
                                    let content_response = ui.vertical(|ui| {
                                        ui.add_space(4.0);
                                        ui.indent(ui.make_persistent_id(("indent", i)), |ui| {
                                            // 递归调用时，我们需要传递 dragging_source，但要注意索引问题
                                            // 这里的 inner 是一个新的 Vec，所以索引是局部的，这简化了问题
                                            // 但我们需要一个新的 dragging_source 状态给子列表吗？
                                            // 实际上，为了简化，我们暂时只支持同级拖拽。
                                            // 如果要支持跨层级拖拽，需要更复杂的状态管理。
                                            // 这里我们传入一个临时的 None，意味着子列表内部可以拖拽，但不能跨层级
                                            // 修复：使用持久化 ID 来存储嵌套列表的拖拽状态
                                            let inner_list_id = ui.make_persistent_id(("nested_list", i, inner.as_ptr()));
                                            Self::render_blocks(input_text, salt_text, ui, inner, changed, inner_list_id);
                                            
                                            ui.horizontal(|ui| {
                                                ui.style_mut().spacing.button_padding = egui::vec2(4.0, 2.0);
                                                if ui.button("+P").on_hover_text("添加 Password").clicked() { inner.push(CustomBlock::Password); *changed = true; }
                                                if ui.button("+S").on_hover_text("添加 Salt").clicked() { inner.push(CustomBlock::Salt); *changed = true; }
                                                
                                                egui::ComboBox::new(ui.make_persistent_id(("inner_algo", i)), "")
                                                    .selected_text("添加哈希")
                                                    .show_ui(ui, |ui| {
                                                        for algo in HashAlgorithm::all() {
                                                            if ui.button(algo.name()).clicked() {
                                                                inner.push(CustomBlock::Hash(*algo, vec![]));
                                                                *changed = true;
                                                                ui.close_menu();
                                                            }
                                                        }
                                                    });
                                            });
                                        });
                                        ui.add_space(4.0);
                                    }).response;
                                    
                                    // 绘制脊柱 (高度跟随内容)
                                    let spine_rect = egui::Rect::from_min_size(
                                        spine_start_pos,
                                        egui::vec2(spine_width, content_response.rect.height())
                                    );
                                    ui.painter().rect_filled(spine_rect, 0.0, color);
                                });

                                // 底部栏 (闭合 C-Block)
                                let bottom_frame = egui::Frame::none()
                                    .fill(color)
                                    .rounding(egui::Rounding { nw: 0.0, ne: 0.0, sw: 10.0, se: 10.0 })
                                    .inner_margin(egui::Margin::symmetric(8.0, 4.0));
                                bottom_frame.show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.label(" "); // 占位高度
                                });
                            });
                        }
                        _ => {
                            // 普通积木
                            let frame = egui::Frame::none()
                                .fill(color)
                                .rounding(egui::Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 8.0));
                            
                            frame.show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(block.name()).color(egui::Color32::WHITE).strong());
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("x").clicked() { to_remove = Some(i); }
                                    });
                                });
                            });
                        }
                    }
                });
            }
        });

        if let Some(i) = to_remove {
            blocks.remove(i);
            *changed = true;
        }
        if let Some((from, to)) = swap_target {
            blocks.swap(from, to);
            dragging_source = Some(to); // 更新拖拽源索引，因为位置变了
            *changed = true;
        }
        
        // 将更新后的拖拽状态存回内存
        ui.data_mut(|d| d.insert_temp(list_id, dragging_source));
    }

    fn get_block_formula(&self, blocks: &[CustomBlock]) -> String {
        let mut parts = Vec::new();
        for block in blocks {
            match block {
                CustomBlock::Password => parts.push("$pass".to_string()),
                CustomBlock::Salt => parts.push("$salt".to_string()),
                CustomBlock::Literal(l) => parts.push(format!("\"{}\"", l)),
                CustomBlock::Hash(algo, inner) => {
                    parts.push(format!("{}({})", algo.name(), self.get_block_formula(inner)));
                }
            }
        }
        parts.join(" + ")
    }

    fn ui_encryption(&mut self, ui: &mut egui::Ui) {
        ui.heading("哈希加密工具");
        ui.separator();

        let mut changed = false;

        // 1. 公共输入区域 (Top)
        ui.group(|ui| {
            ui.label("输入内容 (Password):");
            let response = ui.add(
                egui::TextEdit::multiline(&mut self.input_text)
                    .hint_text("在此输入要计算哈希的文本...")
                    .desired_width(f32::INFINITY)
                    .desired_rows(3),
            );
            if response.changed() {
                changed = true;
            }

            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("盐值 (Salt):");
                if ui.text_edit_singleline(&mut self.salt_text).changed() {
                    changed = true;
                }
            });
        });

        ui.add_space(10.0);

        // 2. 批量计算结果 (Middle - Scrollable)
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("批量哈希结果 (Built-in)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❌").on_hover_text("清空搜索和对比").clicked() {
                        self.search_query.clear();
                        self.compare_target.clear();
                    }
                });
            });
            ui.separator();

            // 搜索和对比工具栏
            ui.horizontal(|ui| {
                ui.label("🔍 搜索:");
                ui.add(egui::TextEdit::singleline(&mut self.search_query).hint_text("筛选算法或哈希值...").desired_width(150.0));
                
                ui.add_space(10.0);
                
                ui.label("⚖️ 对比:");
                ui.add(egui::TextEdit::singleline(&mut self.compare_target).hint_text("输入目标哈希进行匹配...").desired_width(150.0));
            });
            ui.add_space(5.0);
            
            egui::ScrollArea::vertical()
                .max_height(250.0) // Limit height to allow space for custom builder
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width()); // 强制内容区域占满宽度
                    
                    let query = self.search_query.to_lowercase();
                    let target = self.compare_target.trim();

                    for (label, hash) in &self.bulk_results {
                        // 搜索过滤逻辑
                        if !query.is_empty() && !label.to_lowercase().contains(&query) && !hash.to_lowercase().contains(&query) {
                            continue;
                        }

                        // 对比匹配逻辑
                        let is_match = !target.is_empty() && hash.eq_ignore_ascii_case(target);
                        
                        let bg_color = if is_match {
                            egui::Color32::from_rgb(50, 100, 50) // 匹配成功显示深绿色背景
                        } else {
                            egui::Color32::TRANSPARENT
                        };

                        egui::Frame::none().fill(bg_color).inner_margin(2.0).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 10.0;
                                // 固定宽度的标签列
                                ui.scope(|ui| {
                                    ui.set_min_width(150.0);
                                    ui.set_max_width(150.0);
                                    let text = if is_match {
                                        egui::RichText::new(format!("✅ {}", label)).color(egui::Color32::GREEN).strong()
                                    } else {
                                        egui::RichText::new(label).strong()
                                    };
                                    ui.label(text);
                                });
                                
                                // 文本框占满剩余空间
                                let mut hash_text = hash.clone();
                                let text_edit = egui::TextEdit::singleline(&mut hash_text)
                                    .desired_width(f32::INFINITY);
                                
                                let response = ui.add(text_edit);
                                if is_match {
                                    response.highlight(); // 高亮文本框边框
                                }
                            });
                        });
                    }
                });
        });

        ui.add_space(10.0);

        // 3. 自定义加密构建器 (Bottom)
        ui.group(|ui| {
            ui.heading("自定义加密 (Custom Builder)");
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("➕ Password").clicked() {
                    self.custom_blocks.push(CustomBlock::Password);
                    changed = true;
                }
                if ui.button("➕ Salt").clicked() {
                    self.custom_blocks.push(CustomBlock::Salt);
                    changed = true;
                }
                
                ui.separator();
                ui.label("固定文本:");
                ui.text_edit_singleline(&mut self.literal_input);
                if ui.button("➕ 添加").clicked() && !self.literal_input.is_empty() {
                    self.custom_blocks.push(CustomBlock::Literal(self.literal_input.clone()));
                    self.literal_input.clear();
                    changed = true;
                }

                ui.separator();
                egui::ComboBox::new("nested_algo", "")
                    .selected_text(self.nested_algo_selection.name())
                    .show_ui(ui, |ui| {
                        for algo in HashAlgorithm::all() {
                            ui.selectable_value(&mut self.nested_algo_selection, *algo, algo.name());
                        }
                    });
                if ui.button("➕ 添加哈希块").clicked() {
                    self.custom_blocks.push(CustomBlock::Hash(self.nested_algo_selection, vec![]));
                    changed = true;
                }

                ui.separator();
                if ui.button("🗑 清空积木").clicked() {
                    self.custom_blocks.clear();
                    changed = true;
                }
            });

            ui.add_space(5.0);
            
            let frame = egui::Frame::canvas(ui.style())
                .fill(ui.visuals().faint_bg_color)
                .rounding(4.0)
                .inner_margin(10.0);

            ui.label("积木搭建区:");
            frame.show(ui, |ui| {
                ui.set_min_height(150.0);
                ui.set_width(ui.available_width());
                
                egui::ScrollArea::both().show(ui, |ui| {
                    // 使用固定的 ID 作为根列表的 ID
                    let root_list_id = ui.make_persistent_id("root_block_list");
                    Self::render_blocks(&self.input_text, &self.salt_text, ui, &mut self.custom_blocks, &mut changed, root_list_id);
                });
            });

            ui.add_space(5.0);
            let formula = self.get_block_formula(&self.custom_blocks);
            ui.horizontal(|ui| {
                ui.label("公式预览:");
                ui.code(&formula);
            });
            
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("自定义结果:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.output_text)
                        .desired_width(f32::INFINITY)
                );
            });
        });

        // Global Control
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("立即计算").clicked() {
                self.calculate();
            }
            if ui.checkbox(&mut self.auto_calculate, "实时计算").changed() {
                if self.auto_calculate {
                    changed = true;
                }
            }
        });

        if self.auto_calculate && changed {
            self.calculate();
        }
        
        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.label("Powered by Rust & egui");
        });
    }

    fn ui_inference(&mut self, ui: &mut egui::Ui) {
        ui.heading("算法推算工具");
        ui.separator();
        ui.label("此功能正在开发中...");
        ui.add_space(10.0);
        ui.label("思路：通过已知的明文和目标哈希值，遍历所有可能的算法和加盐组合进行碰撞。");
        
        ui.group(|ui| {
            ui.label("已知明文:");
            ui.text_edit_singleline(&mut String::new());
            ui.label("目标哈希值:");
            ui.text_edit_singleline(&mut String::new());
            ui.add_space(10.0);
            if ui.button("开始推算").clicked() {
                // TODO: 实现推算逻辑
            }
        });
    }

    fn calculate_blocks(&self, blocks: &[CustomBlock]) -> String {
        let mut s = String::new();
        for block in blocks {
            match block {
                CustomBlock::Password => s.push_str(&self.input_text),
                CustomBlock::Salt => s.push_str(&self.salt_text),
                CustomBlock::Literal(l) => s.push_str(l),
                CustomBlock::Hash(algo, inner_blocks) => {
                    let inner_content = self.calculate_blocks(inner_blocks);
                    s.push_str(&calculate_hash(*algo, &inner_content, "", SaltMode::None, None));
                }
            }
        }
        s
    }

    fn calculate(&mut self) {
        // 1. Bulk Calculation
        self.bulk_results = calculate_complex_hashes(&self.input_text, &self.salt_text);

        // 2. Custom Block Calculation
        self.output_text = self.calculate_blocks(&self.custom_blocks);
    }
}

impl eframe::App for HashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(170.0)
            .show(ctx, |ui| {
                ui.heading("功能菜单");
                ui.separator();
                
                ui.selectable_value(&mut self.active_tool, ActiveTool::Encryption, "🔐 加密计算");
                ui.selectable_value(&mut self.active_tool, ActiveTool::Inference, "🔍 算法推算");
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 5.0;
                        ui.label("当前版本:");
                        ui.label(egui::RichText::new("v0.5.0").color(egui::Color32::from_rgb(100, 200, 100)).strong());
                    });

                    ui.separator();
                    ui.collapsing("📢 更新日志", |ui| {
                        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new("v0.5.0").strong());
                                ui.small("• 新增批量哈希结果的搜索过滤功能");
                                ui.small("• 新增哈希值对比匹配功能 (高亮显示)");
                                ui.add_space(2.0);

                                ui.label(egui::RichText::new("v0.4.2").strong());
                                ui.small("• 优化批量哈希结果文本框宽度 (全填充)");
                                ui.small("• 完美修复哈希积木视觉连接 (无缝拼接)");
                                ui.add_space(2.0);

                                ui.label(egui::RichText::new("v0.4.1").strong());
                                ui.small("• 优化批量哈希结果显示布局");
                                ui.small("• 改进哈希积木视觉连接 (动态脊柱)");
                                ui.add_space(2.0);

                                ui.label(egui::RichText::new("v0.4.0").strong());
                                ui.small("• 新增批量哈希计算模式 (Built-in)");
                                ui.small("• 重构 UI 为分屏布局 (Built-in / Custom)");
                                ui.small("• 修复嵌套积木的拖拽排序问题");
                                ui.add_space(2.0);

                                ui.label(egui::RichText::new("v0.3.1").strong());
                                ui.small("• 修复哈希积木视觉闭合问题 (C-Block)");
                                ui.small("• 优化拖动手柄感应，解决无法拖动问题");
                                ui.add_space(2.0);

                                ui.label(egui::RichText::new("v0.3.0").strong());
                                ui.small("• 实现鼠标拖动排序积木");
                                ui.small("• 深度还原 Scratch 积木视觉风格");
                                ui.small("• 优化积木搭建区空间布局");
                                ui.add_space(2.0);

                                ui.label(egui::RichText::new("v0.2.1").strong());
                                ui.small("• 新增版本更新通知模块");
                                ui.add_space(2.0);
                                
                                ui.label(egui::RichText::new("v0.2.0").strong());
                                ui.small("• 新增 Scratch 嵌套积木模式");
                                ui.small("• 新增功能侧边栏导航架构");
                                ui.small("• 修复 ComboBox 弃用警告");
                                ui.add_space(2.0);

                                ui.label(egui::RichText::new("v0.1.0").strong());
                                ui.small("• 初始版本发布");
                                ui.small("• 支持 MD5/SHA/SM3 等基础哈希");
                            });
                        });
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tool {
                ActiveTool::Encryption => self.ui_encryption(ui),
                ActiveTool::Inference => self.ui_inference(ui),
            }
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 尝试加载 Windows 微软雅黑字体
    // 注意：在非 Windows 系统或没有该字体文件的系统上，这会失败，回退到默认字体。
    let font_paths = [
        "C:/Windows/Fonts/msyh.ttc",
        "C:/Windows/Fonts/simhei.ttf",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc", // Linux 常见路径
        "/System/Library/Fonts/PingFang.ttc", // macOS 常见路径
    ];

    let mut font_data_loaded = false;

    for path in font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "my_font".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            font_data_loaded = true;
            println!("已加载字体: {}", path);
            break;
        }
    }

    if font_data_loaded {
        // 将其设置为最高优先级
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "my_font".to_owned());
    } else {
        println!("未找到中文字体，中文可能显示乱码");
    }

    ctx.set_fonts(fonts);
}
