use dioxus::prelude::*;
use std::f32::consts::PI;
use std::fmt::Write;

/// Render a "black hole" style icon with curly spiral traits around a center dot.
///
/// The spiral arms are generated procedurally to create a swirling/curly appearance.
/// We keep only the shorter horizontal curls (left/right), cutting 66% of their length.
/// This function is pure and has no side effects beyond producing the SVG element.
#[component]
pub fn Knowledge() -> Element {
    let cx = 24.0;
    let cy = 24.0;
    let inner_r = 3.0;
    let outer_r = 20.0;
    let turns = 1.35;
    let steps = 64;
    let horizontal_outer = scaled_outer(inner_r, outer_r, 0.66);
    let d0 = spiral_path(cx, cy, inner_r, horizontal_outer, turns, 0.0, steps);
    let d2 = spiral_path(cx, cy, inner_r, horizontal_outer, turns, PI, steps);
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "15 15 18 18",
            circle {
                cx: "24",
                cy: "24",
                r: "2",
                fill: "currentColor",
            }
            g { transform: "rotate(90 24 24)",
                path {
                    d: "{d0}",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "0.8",
                    stroke_linecap: "round",
                    opacity: "0.9",
                }
                path {
                    d: "{d2}",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "0.6",
                    stroke_linecap: "round",
                    opacity: "0.7",
                }
            }
        }
    }
}
/// Compute a shortened outer radius by cutting a fraction of the radial span.
///
/// - inner_r, outer_r: original inner and outer radius
/// - cut_fraction: fraction in [0, 1] to cut from the original radial span
///
/// Returns: inner_r + (outer_r - inner_r) * (1 - cut_fraction), clamped.
///
/// This function is pure and has no side effects.
fn scaled_outer(inner_r: f32, outer_r: f32, cut_fraction: f32) -> f32 {
    let cut = cut_fraction.clamp(0.0, 1.0);
    let span = (outer_r - inner_r).max(0.0);
    inner_r + span * (1.0 - cut)
}
/// Build an SVG path string for an Archimedean spiral segment.
///
/// - cx, cy: center
/// - inner_r, outer_r: start/end radius
/// - turns: number of rotations from inner to outer radius
/// - phase: angular offset in radians
/// - steps: number of line segments used to approximate the curve
fn spiral_path(
    cx: f32,
    cy: f32,
    inner_r: f32,
    outer_r: f32,
    turns: f32,
    phase: f32,
    steps: usize,
) -> String {
    let mut d = String::new();
    let n = steps.max(1);
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let angle = phase + t * turns * 2.0 * PI;
        let r = inner_r + t * (outer_r - inner_r);
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        if i == 0 {
            let _ = write!(d, "M {:.2} {:.2}", x, y);
        } else {
            let _ = write!(d, " L {:.2} {:.2}", x, y);
        }
    }
    d
}
