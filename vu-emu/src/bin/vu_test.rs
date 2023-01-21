use eframe::egui::{self, RichText};
use vu_emu::{regs::*, Vu};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 200.0)),
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

            println!("{vu}");

            vu.vmudl(v5, v1, v3);
            vu.vmadm(v6, v2, v3);
            vu.vmadn(v7, v1, v4);
            vu.vmadh(v8, v2, v4);

            println!("{vu}");
            println!("{}", vu.asm());

            Box::new(VuEmuGui::new(vu))
        }),
    )
}

#[derive(Copy, Clone)]
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
    display_type: [RegisterDisplay; 16],
}

impl VuEmuGui {
    fn new(vu: Vu) -> Self {
        Self {
            vu,
            display_type: [RegisterDisplay::default(); 16],
        }
    }
}

impl eframe::App for VuEmuGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .max_width(f32::INFINITY)
                .stick_to_right(true)
                .show(ui, |ui| {
                    ui.expand_to_include_x(frame.info().window_info.size.x);
                    egui::Grid::new("registers")
                        .num_columns(9)
                        .spacing([12.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            for (i, reg) in self.vu.registers.iter().enumerate() {
                                ui.label(RichText::new(format!("v{i}")).monospace());

                                for j in 0..8 {
                                    let v = &reg[(2 * j)..=(2 * j + 1)];
                                    let v = u16::from_be_bytes(v.try_into().unwrap());

                                    if v == 0 {
                                        ui.label(RichText::new("|").monospace());
                                    } else {
                                        ui.label(RichText::new(format!("| {:4x}", v)).monospace());
                                    }
                                }

                                ui.end_row();
                            }
                        });

                    // });
                });
        });
    }
}
