use lyon::tessellation::{
    geometry_builder::{vertex_builder, VertexConstructor},
    BuffersBuilder, FillVertex, StrokeVertex, VertexBuffers,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
}

pub type Mesh = VertexBuffers<Vertex, u16>;

pub fn stroke_builder(
    mesh: &mut Mesh,
) -> BuffersBuilder<'_, Vertex, u16, StrokeVertex, impl VertexConstructor<StrokeVertex, Vertex>> {
    vertex_builder(mesh, |v: StrokeVertex| Vertex {
        x: v.position.x,
        y: v.position.y,
        u: 0.0,
        v: 0.0,
    })
}

pub fn fill_builder(
    mesh: &mut Mesh,
) -> BuffersBuilder<'_, Vertex, u16, FillVertex, impl VertexConstructor<FillVertex, Vertex>> {
    vertex_builder(mesh, |v: FillVertex| Vertex {
        x: v.position.x,
        y: v.position.y,
        u: 0.0,
        v: 0.0,
    })
}

pub fn rectangle() -> Mesh {
    let mut mesh = Mesh::new();

    mesh.vertices = vec![
        Vertex {
            x: 0.0,
            y: 0.0,
            u: 0.0,
            v: 0.0,
        },
        Vertex {
            x: 1.0,
            y: 0.0,
            u: 1.0,
            v: 0.0,
        },
        Vertex {
            x: 1.0,
            y: 1.0,
            u: 1.0,
            v: 1.0,
        },
        Vertex {
            x: 0.0,
            y: 1.0,
            u: 0.0,
            v: 1.0,
        },
    ];

    mesh.indices = vec![0, 1, 2, 2, 3, 0];

    mesh
}
