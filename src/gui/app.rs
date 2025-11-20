use crate::config::Config;
use crate::gui::layouts::Layout;
use crate::matrix::{MatrixRouter, Route};
use crate::ndi::{NdiDiscovery, NdiSource};
use anyhow::Result;
use eframe::egui;
use log::{error, info};
use std::sync::{Arc, Mutex};

/// View state for each matrix view slot
#[derive(Clone, Debug)]
struct ViewSlot {
    /// The output name this slot represents
    output_name: String,
    /// Currently assigned input (if any)
    assigned_input: Option<String>,
    /// Whether this view is selected
    selected: bool,
}

/// Main GUI application state
pub struct MatrixViewerApp {
    /// Current layout configuration
    layout: Layout,
    /// Matrix router
    router: Arc<Mutex<MatrixRouter>>,
    /// NDI discovery service
    discovery: Arc<NdiDiscovery>,
    /// Available NDI sources
    available_sources: Vec<NdiSource>,
    /// View slots for the matrix
    view_slots: Vec<ViewSlot>,
    /// Show layout selection panel
    show_layout_panel: bool,
    /// Show routing panel
    show_routing_panel: bool,
    /// Selected source for routing (index in available_sources)
    selected_source_idx: Option<usize>,
    /// Selected view slot for routing
    selected_view_idx: Option<usize>,
    /// Manual input name for creating placeholder routes
    manual_input_name: String,
}

impl MatrixViewerApp {
    /// Create a new matrix viewer application
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        // Configure egui style
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        cc.egui_ctx.set_style(style);

        // Initialize matrix router
        let mut router = MatrixRouter::new();
        for output in &config.matrix.outputs {
            router.add_output(output.clone());
        }

        // Create view slots
        let view_slots: Vec<ViewSlot> = config
            .matrix
            .outputs
            .iter()
            .map(|output| ViewSlot {
                output_name: output.clone(),
                assigned_input: None,
                selected: false,
            })
            .collect();

        // Initialize NDI discovery
        let discovery = Arc::new(NdiDiscovery::new());

        Self {
            layout: config.gui.default_layout,
            router: Arc::new(Mutex::new(router)),
            discovery,
            available_sources: Vec::new(),
            view_slots,
            show_layout_panel: true,
            show_routing_panel: true,
            selected_source_idx: None,
            selected_view_idx: None,
            manual_input_name: String::new(),
        }
    }

    /// Update available sources from discovery
    fn update_sources(&mut self) {
        self.available_sources = self.discovery.get_sources();

        // Auto-resolve placeholder routes when matching sources appear
        if let Ok(mut router) = self.router.lock() {
            for source in &self.available_sources {
                // Add newly discovered sources to router
                router.add_input(source.clone());
            }
        }
    }

    /// Create or update a route (including placeholder routes)
    fn create_route(&mut self, input: String, output: String) {
        if let Ok(mut router) = self.router.lock() {
            // Try to add input to router if it's a discovered source
            if let Some(source) = self
                .available_sources
                .iter()
                .find(|s| s.name == input || s.url == input)
            {
                router.add_input(source.clone());
            }

            // Create the route (placeholder if source doesn't exist yet)
            let result = if router.input_exists(&input) {
                router.route(&input, &output)
            } else {
                router.route_placeholder(&input, &output)
            };

            if let Err(e) = result {
                error!("Failed to create route: {}", e);
            } else {
                // Update view slot
                if let Some(slot) = self.view_slots.iter_mut().find(|s| s.output_name == output) {
                    slot.assigned_input = Some(input.clone());
                }
                info!("Route created: {} -> {}", input, output);
            }
        }
    }

    /// Remove a route
    fn remove_route(&mut self, output: &str) {
        if let Ok(mut router) = self.router.lock() {
            router.unroute(output);
            if let Some(slot) = self.view_slots.iter_mut().find(|s| s.output_name == output) {
                slot.assigned_input = None;
            }
            info!("Route removed for output: {}", output);
        }
    }

    /// Draw the matrix view area
    fn draw_matrix_view(&mut self, ui: &mut egui::Ui) {
        let available_rect = ui.available_rect_before_wrap();
        let rects = self.layout.calculate_view_rects();

        // Limit view slots to the number supported by the layout
        let num_views = self.layout.view_count().min(self.view_slots.len());

        for (i, (x, y, w, h)) in rects.iter().enumerate().take(num_views) {
            let rect = egui::Rect::from_min_size(
                available_rect.min
                    + egui::vec2(available_rect.width() * x, available_rect.height() * y),
                egui::vec2(
                    available_rect.width() * w - 4.0,
                    available_rect.height() * h - 4.0,
                ),
            );

            let view_slot = &self.view_slots[i];

            // Draw view rectangle
            let response = ui.allocate_rect(rect, egui::Sense::click());

            let fill_color = if view_slot.selected {
                egui::Color32::from_rgb(60, 80, 100)
            } else {
                egui::Color32::from_rgb(40, 40, 50)
            };

            ui.painter().rect_filled(rect, 4.0, fill_color);
            ui.painter().rect_stroke(
                rect,
                4.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 120)),
            );

            // Draw label
            let label_text = if let Some(input) = &view_slot.assigned_input {
                // Check if this is a placeholder route (input doesn't exist)
                let is_placeholder = if let Ok(router) = self.router.lock() {
                    !router.input_exists(input)
                } else {
                    false
                };

                if is_placeholder {
                    format!("{}\n← {} (no feed)", view_slot.output_name, input)
                } else {
                    format!("{}\n← {}", view_slot.output_name, input)
                }
            } else {
                format!("{}\n(No input)", view_slot.output_name)
            };

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label_text,
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );

            // Handle click
            if response.clicked() {
                self.selected_view_idx = Some(i);
                // Toggle selection
                self.view_slots[i].selected = !self.view_slots[i].selected;
            }
        }
    }

    /// Draw the layout selection panel
    fn draw_layout_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Layout");
        ui.separator();

        for layout in Layout::all() {
            let is_selected = self.layout == layout;
            if ui.selectable_label(is_selected, layout.name()).clicked() {
                self.layout = layout;
                info!("Layout changed to: {}", layout.name());
            }
        }
    }

    /// Draw the routing panel
    fn draw_routing_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Routing Control");
        ui.separator();

        // Refresh sources button
        if ui.button("🔄 Refresh Sources").clicked() {
            self.update_sources();
        }

        ui.add_space(10.0);

        // Available sources
        ui.label(format!(
            "Available Sources ({})",
            self.available_sources.len()
        ));
        ui.separator();

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for (idx, source) in self.available_sources.iter().enumerate() {
                    let is_selected = self.selected_source_idx == Some(idx);
                    if ui.selectable_label(is_selected, &source.name).clicked() {
                        self.selected_source_idx = Some(idx);
                    }
                }
            });

        ui.add_space(10.0);

        // Route button for selected source
        ui.horizontal(|ui| {
            let can_route = self.selected_source_idx.is_some() && self.selected_view_idx.is_some();

            if ui
                .add_enabled(can_route, egui::Button::new("➡ Route Selected"))
                .clicked()
            {
                if let (Some(source_idx), Some(view_idx)) =
                    (self.selected_source_idx, self.selected_view_idx)
                {
                    if let (Some(source), Some(view)) = (
                        self.available_sources.get(source_idx),
                        self.view_slots.get(view_idx),
                    ) {
                        self.create_route(source.url.clone(), view.output_name.clone());
                        self.selected_source_idx = None;
                        self.view_slots[view_idx].selected = false;
                    }
                }
            }
        });

        ui.add_space(10.0);
        ui.separator();

        // Manual input name entry for placeholder routes
        ui.label("Or enter input name manually:");
        ui.horizontal(|ui| {
            ui.label("Input name:");
            ui.text_edit_singleline(&mut self.manual_input_name);
        });

        ui.horizontal(|ui| {
            let can_create_placeholder =
                !self.manual_input_name.is_empty() && self.selected_view_idx.is_some();

            if ui
                .add_enabled(
                    can_create_placeholder,
                    egui::Button::new("➡ Create Placeholder Route"),
                )
                .clicked()
            {
                if let Some(view_idx) = self.selected_view_idx {
                    if let Some(view) = self.view_slots.get(view_idx) {
                        self.create_route(self.manual_input_name.clone(), view.output_name.clone());
                        self.manual_input_name.clear();
                        self.view_slots[view_idx].selected = false;
                    }
                }
            }
        });

        ui.add_space(10.0);

        // Current routes
        ui.label("Current Routes");
        ui.separator();

        let routes: Vec<Route> = if let Ok(router) = self.router.lock() {
            router.get_all_routes()
        } else {
            Vec::new()
        };

        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                for route in &routes {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} ← {}", route.output, route.input));
                        if ui.button("❌").clicked() {
                            self.remove_route(&route.output);
                        }
                    });
                }

                if routes.is_empty() {
                    ui.label("No routes configured");
                }
            });
    }
}

impl eframe::App for MatrixViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update sources periodically
        self.update_sources();

        // Top panel - menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("View", |ui| {
                    if ui
                        .checkbox(&mut self.show_layout_panel, "Layout Panel")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                    if ui
                        .checkbox(&mut self.show_routing_panel, "Routing Panel")
                        .clicked()
                    {
                        ui.close_menu();
                    }
                });

                ui.separator();
                ui.label(format!("Current Layout: {}", self.layout.name()));
            });
        });

        // Left panel - layout selection
        if self.show_layout_panel {
            egui::SidePanel::left("layout_panel")
                .default_width(200.0)
                .show(ctx, |ui| {
                    self.draw_layout_panel(ui);
                });
        }

        // Right panel - routing control
        if self.show_routing_panel {
            egui::SidePanel::right("routing_panel")
                .default_width(300.0)
                .show(ctx, |ui| {
                    self.draw_routing_panel(ui);
                });
        }

        // Central panel - matrix view
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_matrix_view(ui);
        });

        // Request repaint for smooth updates
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

/// Run the GUI application
pub fn run_gui(config: Config) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.gui.window_width, config.gui.window_height])
            .with_min_inner_size([800.0, 600.0])
            .with_title("RusTV - NDI Matrix Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "RusTV",
        options,
        Box::new(|cc| {
            let app = MatrixViewerApp::new(cc, config);

            // Start async initialization in background
            let discovery = Arc::clone(&app.discovery);
            tokio::spawn(async move {
                if let Err(e) = discovery.start().await {
                    error!("Failed to start NDI discovery: {}", e);
                }
            });

            Ok(Box::new(app))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run GUI: {}", e))
}
