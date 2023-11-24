use eframe::egui;
use eframe::egui::{Align, Layout, ScrollArea, ViewportCommand};
use egui_extras::{Column, TableBuilder};
use exercise_05::count_words;
use std::sync::mpsc::TryRecvError;

fn main() {
    let options = eframe::NativeOptions::default();

    let (tx_text, rx_text) = std::sync::mpsc::channel();
    let (tx_result, rx_result) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        while let Ok(text) = rx_text.recv() {
            tx_result.send(count_words(text, 1000)).unwrap()
        }
    });

    eframe::run_native(
        "Word count App",
        options,
        Box::new(|_cc| Box::new(WordCountApp::new(tx_text, rx_result))),
    )
    .unwrap();
}

struct WordCountApp {
    text: String,
    word_count: Option<Vec<(String, u32)>>,
    waiting_for_result: bool,
    tx_text: std::sync::mpsc::Sender<String>,
    rx_result: std::sync::mpsc::Receiver<Vec<(String, u32)>>,
}

impl WordCountApp {
    pub fn new(
        tx_text: std::sync::mpsc::Sender<String>,
        rx_result: std::sync::mpsc::Receiver<Vec<(String, u32)>>,
    ) -> Self {
        Self {
            text: "".to_string(),
            word_count: None,
            waiting_for_result: false,
            tx_text,
            rx_result,
        }
    }
}

impl eframe::App for WordCountApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                match self.rx_result.try_recv() {
                    Ok(result) => {
                        ctx.request_repaint();
                        self.waiting_for_result = false;
                        self.word_count = Some(result);
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => {
                        ctx.send_viewport_cmd(ViewportCommand::Close)
                    }
                }

                ui.heading("Input text");

                ScrollArea::both()
                    .id_source("text_scroll")
                    .max_height(200.)
                    .show(ui, |ui| {
                        ui.text_edit_multiline(&mut self.text);
                    });

                ui.add_space(5.);

                if ui.button("Count words").clicked() {
                    self.tx_text.send(self.text.clone()).unwrap();
                    self.waiting_for_result = true;
                }

                ui.add_space(5.);
                ui.separator();

                let text_style = egui::TextStyle::Body;
                let text_height = ui.text_style_height(&text_style);

                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(Column::initial(100.).at_least(70.))
                    .column(Column::remainder().at_least(40.))
                    .resizable(true)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Count");
                        });
                        header.col(|ui| {
                            ui.heading("Word");
                        });
                    })
                    .body(|body| {
                        body.rows(
                            text_height,
                            self.word_count.as_ref().map(|wc| wc.len()).unwrap_or(0),
                            |row_index, mut row| {
                                if let Some(ref word_count) = self.word_count {
                                    let (word, count) = &word_count[row_index];

                                    row.col(|ui| {
                                        ui.label(count.to_string());
                                    });
                                    row.col(|ui| {
                                        ui.label(word);
                                    });
                                }
                            },
                        )
                    });

                if self.waiting_for_result {
                    ctx.request_repaint();
                }
            });
        });
    }
}
