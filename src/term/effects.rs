use crate::AppEvent;
use ratatui::layout::Rect;
use ratatui::style::Color;
use std::sync::mpsc;
use std::time::Duration;
use tachyonfx::fx::*;
use tachyonfx::widget::EffectTimelineRects;
use tachyonfx::Interpolation::{BounceOut, CircIn, ExpoInOut, ExpoOut, QuadOut};
use tachyonfx::{CellFilter, Effect};

pub(super) fn transition_fx(
    screen: Rect,
    sender: mpsc::Sender<AppEvent>,
    fx_in: Effect,
) -> Effect {

    // refresh buffer after transitioning out
    let update_inspected_effect = effect_fn_buf((), 1, move |_, _, _| {
        sender.send(AppEvent::RefreshAufBuffer).unwrap();
    });

    sequence(&[
        out_fx_1(screen),
        update_inspected_effect,
        fx_in,
    ])
}

pub(super) fn effect_in(idx: u8, areas: EffectTimelineRects) -> Effect {
    match idx {
        0 => effect_in_1(areas),
        1 => effect_in_2(areas),
        _ => effect_in_3(areas),
    }
}

pub(super) fn effect_in_1(areas: EffectTimelineRects) -> Effect {
    parallel(&[
        tree_fx_1(areas.tree),
        chart_fx_1(areas.chart),
        cell_filter_and_area_fx(areas.cell_filter, areas.areas, areas.legend),
    ])
}

pub(super) fn effect_in_2(areas: EffectTimelineRects) -> Effect {
    parallel(&[
        tree_fx_1(areas.tree),
        chart_fx_2(areas.chart),
        cell_filter_and_area_fx(areas.cell_filter, areas.areas, areas.legend),
    ])
}

pub(super) fn effect_in_3(areas: EffectTimelineRects) -> Effect {
    parallel(&[
        tree_fx_1(areas.tree),
        chart_fx_3(areas.chart),
        cell_filter_and_area_fx(areas.cell_filter, areas.areas, areas.legend),
    ])
}

pub(super) fn out_fx_1(area: Rect) -> Effect {
    let step = Duration::from_millis(100);
    let bg = Color::Black;

    with_duration(step * 7, parallel(&[
        never_complete(dissolve((step * 5, ExpoInOut))),
        never_complete(fade_to_fg(bg, (5 * step, BounceOut))),
    ]).with_area(area))
}

fn tree_fx_1(area: Rect) -> Effect {
    let step = Duration::from_millis(100);
    let bg = Color::Black;

    parallel(&[
        coalesce((step * 5, ExpoInOut))
            .with_cell_selection(CellFilter::Text),
        sweep_in(Direction::UpToDown, 1, bg, step * 3),
    ]).with_area(area)
}

fn chart_fx_1(area: Rect) -> Effect {
    let step = Duration::from_millis(100);
    let bg = Color::Black;

    prolong_start(step * 4, sweep_in(Direction::RightToLeft, 5, bg, step * 3))
        .with_area(area)
}

fn chart_fx_2(area: Rect) -> Effect {
    let step = Duration::from_millis(100);
    let color1 = Color::from_u32(0x102020);
    let color2 = Color::from_u32(0x204040);

    parallel(&[
        parallel(&[
            timed_never_complete(step * 10, fade_to(Color::Black, Color::Black, 0)),
            timed_never_complete(step * 10, fade_to(color1, color1, (step * 5, QuadOut))),
        ]),
        sequence(&[
            sleep(step * 10),
            parallel(&[
                slide_in(Direction::DownToUp, 15, color2, step * 5),
                fade_from_fg(color1, (step * 10, ExpoOut)),
            ]),
        ])
    ]).with_area(area)
}

fn chart_fx_3(area: Rect) -> Effect {
    let step = Duration::from_millis(100);
    let bg = Color::Black;

    let hsl_shift = [0.0, -100.0, -50.0];

    parallel(&[
        hsl_shift_fg(hsl_shift, (15 * step, CircIn)).reversed(),
        sweep_in(Direction::LeftToRight, 80, bg, step * 15),
    ]).with_area(area)
}

pub fn cell_filter_and_area_fx(
    cell_filter_column: Rect,
    area_column: Rect,
    legend: Rect
) -> Effect {
    let d = Duration::from_millis(500);

    parallel(&[
        prolong_start(d, sweep_in(Direction::DownToUp, 1, Color::Black, (d, QuadOut)))
            .with_area(cell_filter_column),
        prolong_start(d * 2, sweep_in(Direction::UpToDown, 1, Color::Black, (d, QuadOut)))
            .with_area(area_column),
        prolong_start(d * 3,  fade_from_fg(Color::Black, (700, QuadOut)))
            .with_area(legend),
    ])
}
}