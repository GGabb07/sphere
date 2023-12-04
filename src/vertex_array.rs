use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};

use crate::terrgen::{gen_voxels, ChunkShape};

pub struct VertexArray {
    vao: u32,
    pos_vbo: u32,
    norm_vbo: u32,
    ebo: u32,
    num_indices: i32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut norm_vbo = 0;
        let mut ebo = 0;

        let voxels = gen_voxels();

        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

        let mut buffer = GreedyQuadsBuffer::new(voxels.len());
        greedy_quads(
            &voxels,
            &ChunkShape {},
            [0; 3],
            [65; 3],
            &faces,
            &mut buffer,
        );
        let num_indices = buffer.quads.num_quads() * 6;
        let num_vertices = buffer.quads.num_quads() * 4;
        let mut indices = Vec::with_capacity(num_indices);
        let mut positions = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);
        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
                normals.extend_from_slice(&face.quad_mesh_normals());
            }
        }

        unsafe {
            gl::CreateVertexArrays(1, &mut vao);
            assert_ne!(0, vao);
            gl::CreateBuffers(1, &mut vbo);
            assert_ne!(0, vbo);
            gl::CreateBuffers(1, &mut norm_vbo);
            assert_ne!(0, norm_vbo);
            gl::CreateBuffers(1, &mut ebo);
            assert_ne!(0, ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (4 * num_vertices * 3) as isize,
                positions.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 4 * 3, 0 as *const _);
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, norm_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (4 * num_vertices * 3) as isize,
                normals.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 4 * 3, 0 as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (4 * num_indices) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);
        }

        Self {
            vao,
            pos_vbo: vbo,
            norm_vbo,
            ebo,
            num_indices: num_indices as i32,
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.num_indices,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.pos_vbo);
            gl::DeleteBuffers(1, &self.norm_vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
