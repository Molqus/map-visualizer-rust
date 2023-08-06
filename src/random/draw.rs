use super::legacy_random::LegacyRandom;
use plotters::coord::types::*;
use plotters::prelude::*;

fn draw(data: [(f64, i32); 32]) {
    let root_area: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
        BitMapBackend::new("test.png", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx: ChartContext<'_, BitMapBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordi32>> =
        ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("Scatter Demo", ("sans-serif", 40))
            .build_cartesian_2d(0.0..512.0, 0..128)
            .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(data.iter().map(|point| Circle::new(*point, 5, &BLUE)))
        .unwrap();
}

pub fn draw_data(bpm: f64, idx: i32, cs: f64, length: i32) -> (Vec<f64>, f64, f64) {
    const RNG_SEED: u32 = 1337;
    let mut test_rand: LegacyRandom = LegacyRandom::new(RNG_SEED);

    let ms_per_beat: f64 = 60.0 * 1000.0 / bpm;
    let mut offset_y: f64 = ms_per_beat * length as f64;
    let mut banana_counts: i32 = 1;
    while offset_y > 100.0 {
        offset_y /= 2.0;
        banana_counts *= 2;
    }

    for _ in 0..idx {
        test_rand.next();
    }

    let mut data: Vec<f64> = Vec::new();
    for _ in 0..banana_counts {
        let t: f64 = test_rand.next_double() * 100.0;
        data.push(t);
        test_rand.next();
        test_rand.next();
        test_rand.next();
    }

    // 64 * (1.0 - 0.7 * (cs - 5) / 5) / 2
    let fruits_radius: f64 = (54.4 - 4.48 * cs) * 100.0 / 640.0;
    // draw(data);
    (data, offset_y, fruits_radius)
}
