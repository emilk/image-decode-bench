use web_time::{Duration, Instant};

const IMAGES: [&[u8]; 1] = [include_bytes!("../images/1.jpg")];

#[derive(Default)]
pub struct ImageDecodeBench {
    image_decode_time: Duration,
    zune_decode_time: Duration,
}

impl ImageDecodeBench {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut slf = Self::default();
        slf.measure();
        slf
    }

    pub fn measure(&mut self) {
        for _ in 0..2 {
            self.measure_image();
            self.measure_zune();
        }
    }

    fn measure_image(&mut self) {
        let start = Instant::now();
        for jpeg_bytes in IMAGES {
            let image = image::load_from_memory(jpeg_bytes).unwrap();
            assert!(image.width() > 0);
        }
        self.image_decode_time = start.elapsed();
    }

    fn measure_zune(&mut self) {
        let start = Instant::now();
        for jpeg_bytes in IMAGES {
            let mut decoder = zune_jpeg::JpegDecoder::new(jpeg_bytes);
            let pixels = decoder.decode().unwrap();
            assert!(!pixels.is_empty());
        }
        self.zune_decode_time = start.elapsed();
    }
}

impl eframe::App for ImageDecodeBench {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            ui.monospace(format!(
                "image: {:.1} ms / image",
                1e3 * self.image_decode_time.as_secs_f32() / IMAGES.len() as f32,
            ));
            ui.monospace(format!(
                "zune:  {:.1} ms / image",
                1e3 * self.zune_decode_time.as_secs_f32() / IMAGES.len() as f32,
            ));

            if ui.button("Measure again").clicked() {
                self.measure();
            }
        });
    }
}
