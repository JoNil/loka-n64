use eframe::{
    egui::{self, RichText, Separator},
    epaint::Vec2,
};
use vu_emu::{instruction, regs::*, Vu};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 960.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Vu Emu",
        options,
        Box::new(|_cc| {
            let mut vu = Vu::default();

            vu.load_fix_point(v1.e0(), v2.e0(), 3 << 15);
            vu.load_fix_point(v1.e2(), v2.e2(), 5 << 15);
            vu.load_fix_point(v1.e4(), v2.e4(), 7 << 15);
            vu.load_fix_point(v1.e6(), v2.e6(), 9 << 15);

            vu.load_fix_point(v3.e0(), v4.e0(), 3 << 16);
            vu.load_fix_point(v3.e2(), v4.e2(), 4 << 16);
            vu.load_fix_point(v3.e4(), v4.e4(), 5 << 16);
            vu.load_fix_point(v3.e6(), v4.e6(), 6 << 16);

            Box::new(VuEmuGui::new(vu))
        }),
    )
}

#[derive(Copy, Clone, Debug)]
enum RegisterDisplay {
    Dec,
    Hex,
    Fix,
}

impl Default for RegisterDisplay {
    fn default() -> Self {
        Self::Hex
    }
}

#[derive(Default)]
struct VuEmuGui {
    vu: Vu,
    display_type: [RegisterDisplay; 32],
    code: String,
}

impl VuEmuGui {
    fn new(vu: Vu) -> Self {
        Self {
            vu,
            display_type: [RegisterDisplay::default(); 32],
            code: "vmudl v5,v1,v3
vmadm v6,v2,v3
vmadn v7,v1,v4
vmadh v8,v2,v4"
                .to_string(),
        }
    }
}

impl eframe::App for VuEmuGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.expand_to_include_x(frame.info().window_info.size.x);

                ui.horizontal_top(|ui| {
                    egui::Grid::new("registers")
                        .striped(true)
                        .min_col_width(0.0)
                        .show(ui, |ui| {
                            let vu = run(&self.vu, &self.code);

                            for (i, reg) in vu.registers.iter().enumerate() {
                                ui.label(RichText::new(format!("v{i}")).monospace());

                                for j in 0..8 {
                                    let v = &reg[(2 * j)..=(2 * j + 1)];
                                    let v = u16::from_be_bytes(v.try_into().unwrap());

                                    ui.add(Separator::default().vertical());

                                    match self.display_type[i] {
                                        RegisterDisplay::Dec => {
                                            if v == 0 {
                                                ui.label(RichText::new("     ").monospace());
                                            } else {
                                                ui.label(
                                                    RichText::new(format!("{:5}", v)).monospace(),
                                                );
                                            }
                                        }
                                        RegisterDisplay::Hex => {
                                            if v == 0 {
                                                ui.label(RichText::new("     ").monospace());
                                            } else {
                                                ui.label(
                                                    RichText::new(format!("{:5x}", v)).monospace(),
                                                );
                                            }
                                        }
                                        RegisterDisplay::Fix => {
                                            if v == 0 {
                                                ui.label(RichText::new("     ").monospace());
                                            } else {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{:.3}",
                                                        v as f64 / (1 << 16) as f64
                                                    ))
                                                    .monospace(),
                                                );
                                            }
                                        }
                                    }
                                }

                                ui.add(Separator::default().vertical());

                                if ui
                                    .button(
                                        RichText::new(format!("{:?}", self.display_type[i]))
                                            .monospace(),
                                    )
                                    .clicked()
                                {
                                    self.display_type[i] = match self.display_type[i] {
                                        RegisterDisplay::Dec => RegisterDisplay::Hex,
                                        RegisterDisplay::Hex => RegisterDisplay::Fix,
                                        RegisterDisplay::Fix => RegisterDisplay::Dec,
                                    }
                                }

                                ui.end_row();
                            }
                        });

                    ui.add_sized(
                        ui.available_size() - Vec2::new(7.0, 0.0),
                        egui::TextEdit::multiline(&mut self.code)
                            .code_editor()
                            .frame(false),
                    );
                });
            });
        });
    }
}

fn run(vu: &Vu, code: &str) -> Vu {
    let mut vu = vu.clone();

    for line in code.lines() {
        let line = line.trim();
        if let Some(instruction) = instruction::parse(line) {
            instruction::execute(&mut vu, instruction);
        }
    }

    vu
}
