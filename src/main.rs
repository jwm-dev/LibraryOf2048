use eframe::{egui, App, Frame, NativeOptions};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
mod protoboards;

// Parse protoboards.txt into map: t -> Vec<(global_id, board matrix)>
fn parse_protoboards(path: &str) -> HashMap<u32, Vec<(usize, Vec<Vec<char>>)>> {
    if !Path::new(path).exists() {
        protoboards::generate_protoboards();
    }
    
    let file = File::open(Path::new(path)).expect("Cannot open protoboards.txt");
    let reader = BufReader::new(file);
    let mut map: HashMap<u32, Vec<(usize, Vec<Vec<char>>)>> = HashMap::new();
    let mut current_t = 0;
    let mut current_id = 0;
    let mut current_board: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("read error");
        if line.contains("Boards with t =") {
            if !current_board.is_empty() {
                map.entry(current_t).or_default().push((current_id, current_board.clone()));
                current_board.clear();
            }
            if let Some(v) = line.split('=').nth(1) {
                current_t = v.trim().split_whitespace().next().unwrap().parse().unwrap();
            }
        } else if line.starts_with("Board #") {
            if !current_board.is_empty() {
                map.entry(current_t).or_default().push((current_id, current_board.clone()));
                current_board.clear();
            }
            if let Some(v) = line.split('#').nth(1) {
                current_id = v.split_whitespace().next().unwrap().parse().unwrap();
            }
        } else if line.contains('X') || line.contains('.') {
            let row: Vec<char> = line.chars().filter(|&c| c == 'X' || c == '.').collect();
            if !row.is_empty() {
                current_board.push(row);
            }
        }
    }
    if !current_board.is_empty() {
        map.entry(current_t).or_default().push((current_id, current_board));
    }
    map
}

fn count_filled(board: &Vec<Vec<char>>) -> usize {
    board.iter().flatten().filter(|&&c| c == 'X').count()
}

fn parse_base11(s: &str) -> Vec<u32> {
    if s.chars().filter(|&c| c == 'B' || c == 'b').count() > 1 {
        panic!("Invalid base-11 ID: more than one 'B'");
    }
    s.chars().map(|c| match c {
        '1'..='9' => c.to_digit(11).unwrap(),
        'A' | 'a' => 10,
        'B' | 'b' => 11,
        _ => panic!("Invalid base-11 digit"),
    }).collect()
}

fn fill_board(proto: &Vec<Vec<char>>, tiles: &[u32]) -> Vec<Vec<u32>> {
    let mut filled = vec![vec![0; 4]; 4];
    let mut iter = tiles.iter();
    for i in 0..4 {
        for j in 0..4 {
            if proto[i][j] == 'X' {
                let exp = *iter.next().unwrap();
                filled[i][j] = 2u32.pow(exp);
            }
        }
    }
    filled
}
fn tile_color(value: u32) -> egui::Color32 {
    match value {
        2    => egui::Color32::from_rgb(0xee, 0xe4, 0xda),
        4    => egui::Color32::from_rgb(0xed, 0xe0, 0xc8),
        8    => egui::Color32::from_rgb(0xf2, 0xb1, 0x79),
        16   => egui::Color32::from_rgb(0xf5, 0x95, 0x63),
        32   => egui::Color32::from_rgb(0xf6, 0x7c, 0x5f),
        64   => egui::Color32::from_rgb(0xf6, 0x5e, 0x3b),
        128  => egui::Color32::from_rgb(0xed, 0xcf, 0x72),
        256  => egui::Color32::from_rgb(0xed, 0xcc, 0x61),
        512  => egui::Color32::from_rgb(0xed, 0xc8, 0x50),
        1024 => egui::Color32::from_rgb(0xed, 0xc5, 0x3f),
        2048 => egui::Color32::from_rgb(0xed, 0xc2, 0x2e),
        _    => egui::Color32::from_rgb(0xcd, 0xc1, 0xb4), // fallback for higher tiles
    }
}

struct App2048 {
    protoboards: HashMap<u32, Vec<(usize, Vec<Vec<char>>) >>,
    t_values: Vec<u32>,
    selected_t: Option<u32>,
    global_id: String,
    local_id: String,
    current_proto: Option<Vec<Vec<char>>>,
    filled_tiles: usize,
    generated: Option<Vec<Vec<u32>>>,
    view_proto: bool,
}

impl Default for App2048 {
    fn default() -> Self {
        let protoboards = parse_protoboards("protoboards.txt");
        let mut t_values: Vec<_> = protoboards.keys().cloned().collect();
        t_values.sort();
        App2048 {
            protoboards,
            t_values,
            selected_t: None,
            global_id: String::new(),
            local_id: String::new(),
            current_proto: None,
            filled_tiles: 0,
            generated: None,
            view_proto: false,
        }
    }
}

impl App for App2048 {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Select t:");
                for &t in &self.t_values {
                    if ui.selectable_label(self.selected_t == Some(t), t.to_string()).clicked() {
                        self.selected_t = Some(t);
                        self.current_proto = None;
                        self.generated = None;
                        self.view_proto = false;
                        self.global_id.clear();
                        self.local_id.clear();
                    }
                }
            });
            if let Some(t) = self.selected_t {
                let boards = &self.protoboards[&t];
                let mut start_id = 1;
                for &k in &self.t_values {
                    if k == t { break; }
                    start_id += self.protoboards[&k].len();
                }
                let end_id = start_id + boards.len() - 1;
                ui.label(format!("Valid IDs for t={}: {}..={}", t, start_id, end_id));
                ui.horizontal(|ui| {
                    ui.label("Global ID:");
                    ui.text_edit_singleline(&mut self.global_id);
                    if ui.button("Load Protoboard").clicked() {
                        if let Ok(gid) = self.global_id.trim().parse::<usize>() {
                            if let Some((_, proto)) = boards.iter().find(|(id,_)| *id == gid) {
                                self.current_proto = Some(proto.clone());
                                self.filled_tiles = count_filled(proto);
                                self.generated = None;
                                self.view_proto = true;
                                self.local_id.clear();
                            }
                        }
                    }
                });
                if let Some(proto) = &self.current_proto {
                    ui.label(format!("Local ID length == t={}; Must use digits [1,2,3,4,5,6,7,8,9,A,B]", t));
                    ui.horizontal(|ui| {
                        ui.label("Local ID:");
                        ui.text_edit_singleline(&mut self.local_id);
                        if ui.button("Generate").clicked() {
                            if self.local_id.len() == self.filled_tiles {
                                let tiles = parse_base11(&self.local_id);
                                self.generated = Some(fill_board(proto, &tiles));
                                self.view_proto = false;
                            }
                        }
                    });
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let avail = ui.available_rect_before_wrap();
            let cell_size = avail.width().min(avail.height()) / 4.0;
            let mut y = avail.top();
            for row in 0..4 {
                let mut x = avail.left();
                for col in 0..4 {
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(x, y), egui::vec2(cell_size, cell_size)
                    );
                    ui.painter().rect_stroke(
                        rect,
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::GRAY),
                        egui::StrokeKind::Middle,
                    );
                    if self.view_proto {
                        if let Some(proto) = &self.current_proto {
                            if proto[row][col] == 'X' {
                                ui.painter().text(
                                    rect.center(), egui::Align2::CENTER_CENTER,
                                    "X",
                                    egui::FontId::proportional(cell_size * 0.5),
                                    egui::Color32::WHITE,
                                );
                            }
                        }
                    } else if let Some(board) = &self.generated {
                        let v = board[row][col];
                        if v != 0 {
                            // Add a margin so the colored tile is inset
                            let margin = cell_size * 0.03;
                            let inner_rect = egui::Rect::from_min_max(
                                rect.min + egui::vec2(margin, margin),
                                rect.max - egui::vec2(margin, margin),
                            );
                            // Draw colored background with rounded corners
                            ui.painter().rect_filled(
                                inner_rect,
                                cell_size * 0.18, // rounding radius
                                tile_color(v),
                            );

                            ui.painter().text(
                                rect.center(), egui::Align2::CENTER_CENTER,
                                v.to_string(),
                                egui::FontId::proportional(cell_size * 0.4),
                                egui::Color32::WHITE,
                            );
                        }
                    }
                    x += cell_size;
                }
                y += cell_size;
            }
        });
    }
}

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "2048 Library of Babel",
        native_options,
        Box::new(|_cc| Ok(Box::new(App2048::default()) as Box<dyn App>)),
    )
    .expect("failed to start eframe");
}
