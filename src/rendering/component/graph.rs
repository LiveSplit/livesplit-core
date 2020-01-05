use {
    crate::{
        component::graph::State,
        layout::LayoutState,
        rendering::{
            mesh::{basic_builder, fill_builder, stroke_builder},
            Backend, Mesh, RenderContext,
        },
        settings::Gradient,
    },
    lyon::tessellation::{
        basic_shapes::{fill_circle, fill_polyline, stroke_polyline},
        FillOptions, FillTessellator, StrokeOptions,
    },
    std::iter,
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    [width, height]: [f32; 2],
    component: &State,
    _layout_state: &LayoutState,
) {
    let old_transform = context.transform;
    context.scale(height);
    let width = width / height;

    const GRID_LINE_WIDTH: f32 = 0.015;
    const LINE_WIDTH: f32 = 0.025;
    const CIRCLE_RADIUS: f32 = 0.035;

    context.render_rectangle(
        [0.0, 0.0],
        [width, component.middle],
        &Gradient::Plain(component.top_background_color),
    );
    context.render_rectangle(
        [0.0, component.middle],
        [width, 1.0],
        &Gradient::Plain(component.bottom_background_color),
    );

    for &y in &component.horizontal_grid_lines {
        context.render_rectangle(
            [0.0, y - GRID_LINE_WIDTH],
            [width, y + GRID_LINE_WIDTH],
            &Gradient::Plain(component.grid_lines_color),
        );
    }

    for &x in &component.vertical_grid_lines {
        context.render_rectangle(
            [width * x - GRID_LINE_WIDTH, 0.0],
            [width * x + GRID_LINE_WIDTH, 1.0],
            &Gradient::Plain(component.grid_lines_color),
        );
    }

    let mut mesh = Mesh::new();

    let len = if component.is_live_delta_active {
        let p1 = &component.points[component.points.len() - 2];
        let p2 = &component.points[component.points.len() - 1];

        fill_polyline(
            [
                [p1.x, component.middle],
                [p1.x, p1.y],
                [p2.x, p2.y],
                [p2.x, component.middle],
            ]
            .iter()
            .map(|&[x, y]| [width * x, y].into()),
            &mut FillTessellator::new(),
            &FillOptions::tolerance(0.005),
            &mut fill_builder(&mut mesh),
        )
        .unwrap();

        let partial_fill_mesh = context.create_mesh(&mesh);
        context.render_mesh(&partial_fill_mesh, component.partial_fill_color);
        context.free_mesh(partial_fill_mesh);

        mesh.clear();

        component.points.len() - 1
    } else {
        component.points.len()
    };

    fill_polyline(
        iter::once([0.0, component.middle].into())
            .chain(
                component.points[..len]
                    .iter()
                    .map(|p| [width * p.x, p.y].into()),
            )
            .chain(iter::once(
                [width * component.points[len - 1].x, component.middle].into(),
            )),
        &mut FillTessellator::new(),
        &FillOptions::tolerance(0.005),
        &mut fill_builder(&mut mesh),
    )
    .unwrap();

    let fill_mesh = context.create_mesh(&mesh);
    context.render_mesh(&fill_mesh, component.complete_fill_color);
    context.free_mesh(fill_mesh);

    for points in component.points.windows(2) {
        mesh.clear();

        let p1 = [width * points[0].x, points[0].y].into();
        let p2 = [width * points[1].x, points[1].y].into();

        stroke_polyline(
            iter::once(p1).chain(iter::once(p2)),
            false,
            &StrokeOptions::default().with_line_width(LINE_WIDTH),
            &mut stroke_builder(&mut mesh),
        )
        .unwrap();

        let color = if points[1].is_best_segment {
            component.best_segment_color
        } else {
            component.graph_lines_color
        };

        let line_mesh = context.create_mesh(&mesh);
        context.render_mesh(&line_mesh, color);
        context.free_mesh(line_mesh);
    }

    for (i, point) in component.points.iter().enumerate().skip(1) {
        if i != component.points.len() - 1 || !component.is_live_delta_active {
            mesh.clear();

            fill_circle(
                [width * point.x, point.y].into(),
                CIRCLE_RADIUS,
                &FillOptions::tolerance(0.005),
                &mut basic_builder(&mut mesh),
            )
            .unwrap();

            let color = if point.is_best_segment {
                component.best_segment_color
            } else {
                component.graph_lines_color
            };

            let circle_mesh = context.create_mesh(&mesh);
            context.render_mesh(&circle_mesh, color);
            context.free_mesh(circle_mesh);
        }
    }

    context.transform = old_transform;
}
