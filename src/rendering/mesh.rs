use lyon::tessellation::{
    geometry_builder::{
        vertex_builder, BasicGeometryBuilder, FillGeometryBuilder, StrokeGeometryBuilder,
    },
    math::Point,
    FillAttributes, StrokeAttributes, VertexBuffers,
};

/// The vertex types describes a single point of a mesh used to form triangles.
/// It uses a C compatible layout such that it can be directly uploaded to a GPU.
#[repr(C)]
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Vertex {
    /// The x coordinate of the vertex.
    pub x: f32,
    /// The y coordinate of the vertex.
    pub y: f32,
    /// The u coordinate of the vertex, which corresponds to the x axis in
    /// texture space.
    pub u: f32,
    /// The v coordinate of the vertex, which corresponds to the y axis in
    /// texture space.
    pub v: f32,
}

/// A mesh supplied to the backend that will eventually be rendered out.
pub struct Mesh {
    pub(in crate::rendering) buffers: VertexBuffers<Vertex, u16>,
}

impl Mesh {
    pub(in crate::rendering) fn new() -> Self {
        Self {
            buffers: VertexBuffers::new(),
        }
    }

    pub(in crate::rendering) fn clear(&mut self) {
        self.buffers.vertices.clear();
        self.buffers.indices.clear();
    }

    pub(in crate::rendering) fn vertices_mut(&mut self) -> &mut [Vertex] {
        &mut self.buffers.vertices
    }

    /// The vertices that make up the mesh.
    pub fn vertices(&self) -> &[Vertex] {
        &self.buffers.vertices
    }

    /// The indices describe the actual triangles that make up the mesh. Each
    /// chunk of three indices pointing into the `vertices` makes up a triangle.
    pub fn indices(&self) -> &[u16] {
        &self.buffers.indices
    }
}

pub fn stroke_builder(mesh: &mut Mesh) -> impl StrokeGeometryBuilder + '_ {
    vertex_builder::<_, _, (), _>(
        &mut mesh.buffers,
        |p: Point, _: StrokeAttributes<'_, '_>| Vertex {
            x: p.x,
            y: p.y,
            u: 0.0,
            v: 0.0,
        },
    )
}

pub fn fill_builder(mesh: &mut Mesh) -> impl FillGeometryBuilder + '_ {
    vertex_builder::<_, _, (), _>(&mut mesh.buffers, |p: Point, _: FillAttributes<'_>| {
        Vertex {
            x: p.x,
            y: p.y,
            u: 0.0,
            v: 0.0,
        }
    })
}

pub fn basic_builder(mesh: &mut Mesh) -> impl BasicGeometryBuilder + '_ {
    vertex_builder::<_, _, (), _>(&mut mesh.buffers, |p: Point| Vertex {
        x: p.x,
        y: p.y,
        u: 0.0,
        v: 0.0,
    })
}

pub fn rectangle() -> Mesh {
    let mut buffers = VertexBuffers::new();

    buffers.vertices = vec![
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

    buffers.indices = vec![0, 1, 2, 2, 3, 0];

    Mesh { buffers }
}
