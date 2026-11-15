use crate::math::{Mat4, Vec3, Vec4};

use super::Vertex;

#[derive(Debug, Clone, PartialEq)]
pub enum MeshError {
    IndexCountNotMultipleOfThree,
    InvalidIndex { index: u32, vertex_count: usize },
    InvalidHomogeneousCoordinate { vertex_index: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Result<Self, MeshError> {
        if indices.len() % 3 != 0 {
            return Err(MeshError::IndexCountNotMultipleOfThree);
        }

        for &index in &indices {
            if index as usize >= vertices.len() {
                return Err(MeshError::InvalidIndex {
                    index,
                    vertex_count: vertices.len(),
                });
            }
        }

        Ok(Self { vertices, indices })
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn transform_positions(&mut self, matrix: Mat4) -> Result<(), MeshError> {
        let mut transformed = Vec::with_capacity(self.vertices.len());

        for (index, vertex) in self.vertices.iter().enumerate() {
            let position = vertex.position();

            let homogeneous = matrix * Vec4::new(position.x(), position.y(), position.z(), 1.0);

            let w = homogeneous.w();
            if w == 0.0 || !w.is_finite() {
                return Err(MeshError::InvalidHomogeneousCoordinate {
                    vertex_index: index,
                });
            }

            let result = Vec3::new(
                homogeneous.x() / w,
                homogeneous.y() / w,
                homogeneous.z() / w,
            );
            if !result.is_finite() {
                return Err(MeshError::InvalidHomogeneousCoordinate {
                    vertex_index: index,
                });
            }

            transformed.push(result);
        }

        for (vertex, position) in self.vertices.iter_mut().zip(transformed) {
            vertex.set_position(position);
        }

        Ok(())
    }

    pub fn transformed(&self, matrix: Mat4) -> Result<Self, MeshError> {
        let mut result = self.clone();
        result.transform_positions(matrix)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Mat4, Vec2, Vec3};

    mod new {
        use super::*;

        #[test]
        fn accepts_valid_input() {
            let cases = [
                (
                    "triangle",
                    vec![
                        Vertex::new(Vec3::ZERO, Vec3::Z, Vec2::ZERO),
                        Vertex::new(Vec3::X, Vec3::Z, Vec2::X),
                        Vertex::new(Vec3::Y, Vec3::Z, Vec2::Y),
                    ],
                    vec![0, 1, 2],
                ),
                ("empty", vec![], vec![]),
            ];

            for (name, vertices, indices) in cases {
                let mesh = Mesh::new(vertices.clone(), indices.clone()).unwrap_or_else(|err| {
                    panic!("Test case '{name}' should produce a mesh: {err:?}")
                });

                assert_eq!(mesh.vertex_count(), vertices.len());
                assert_eq!(mesh.index_count(), indices.len());
                assert_eq!(mesh.triangle_count(), indices.len() / 3);
                assert_eq!(mesh.is_empty(), vertices.len() == 0)
            }
        }

        #[test]
        fn rejects_invalid_input() {
            let cases = [
                (
                    "index count not multiple of three",
                    vec![Vertex::default(), Vertex::default()],
                    vec![0, 1],
                    Err(MeshError::IndexCountNotMultipleOfThree),
                ),
                (
                    "index equal to vertices count",
                    vec![Vertex::default(), Vertex::default()],
                    vec![0, 1, 2],
                    Err(MeshError::InvalidIndex {
                        index: 2,
                        vertex_count: 2,
                    }),
                ),
                (
                    "index greater than vertices count",
                    vec![Vertex::default(), Vertex::default()],
                    vec![0, 1, 3],
                    Err(MeshError::InvalidIndex {
                        index: 3,
                        vertex_count: 2,
                    }),
                ),
            ];

            for (name, vertices, indices, want_err) in cases {
                let result = Mesh::new(vertices, indices);

                assert_eq!(
                    result, want_err,
                    "Test case '{name} should return error: got {result:?}, want {want_err:?}"
                );
            }
        }
    }

    mod transform {
        use super::*;

        #[test]
        fn applies_valid_transformation() {
            let cases = [
                (
                    "scaling by 2.0",
                    Vec3::new(1.0, 2.0, 3.0),
                    Mat4::scaling(Vec3::new(2.0, 2.0, 2.0)),
                    Vec3::new(2.0, 4.0, 6.0),
                ),
                (
                    "translation by (5, -1, 2)",
                    Vec3::ONE,
                    Mat4::translation(Vec3::new(5.0, -1.0, 2.0)),
                    Vec3::new(6.0, 0.0, 3.0),
                ),
            ];

            for (name, initial_pos, matrix, want_pos) in cases {
                let mut mesh = create_mesh(name, initial_pos);

                let result = mesh.transform_positions(matrix);

                assert!(
                    result.is_ok(),
                    "Test case '{name}' transformation failed: {result:?}"
                );

                let got_pos = mesh.vertices()[0].position();
                assert_eq!(
                    got_pos, want_pos,
                    "Test case '{name}' position mismatch: got {got_pos:?}, want {want_pos:?}"
                );
            }
        }

        #[test]
        fn reject_invalid_homogeneous_coordinates() {
            let cases = [(
                "w coordinate is zero",
                Vec3::ONE,
                Mat4::ZERO,
                Err(MeshError::InvalidHomogeneousCoordinate { vertex_index: 0 }),
            )];

            for (name, initial_pos, matrix, want_err) in cases {
                let mut mesh = create_mesh(name, initial_pos);

                let result = mesh.transform_positions(matrix);

                assert_eq!(
                    result, want_err,
                    "Test case '{name}' shoud return error: got {result:?}, want {want_err:?}"
                );
            }
        }

        fn create_mesh(name: &str, initial_pos: Vec3) -> Mesh {
            let vertices = vec![Vertex::new(initial_pos, Vec3::Z, Vec2::ZERO)];
            let indices = vec![0, 0, 0];

            let mesh = Mesh::new(vertices, indices)
                .unwrap_or_else(|err| panic!("Test case '{name}' failed to create mesh: {err:?}"));

            mesh
        }
    }

    mod tranformed {
        use super::*;

        #[test]
        fn returns_new_transformed_mesh_immutably() {
            let vertices = vec![Vertex::new(Vec3::X, Vec3::Z, Vec2::ZERO)];
            let indices = vec![0, 0, 0];

            let mesh = Mesh::new(vertices, indices).unwrap();
            let matrix = Mat4::scaling(Vec3::new(3.0, 3.0, 3.0));

            let transformed_mesh = mesh.transformed(matrix).unwrap();

            assert_eq!(mesh.vertices[0].position().x(), 1.0);
            assert_eq!(transformed_mesh.vertices[0].position().x(), 3.0);
        }
    }

    mod clear {
        use super::*;

        #[test]
        fn resets_mesh_state() {
            let vertices = vec![
                Vertex::new(Vec3::ZERO, Vec3::Z, Vec2::ZERO),
                Vertex::new(Vec3::X, Vec3::Z, Vec2::ZERO),
                Vertex::new(Vec3::Y, Vec3::Z, Vec2::ZERO),
            ];
            let indices = vec![0, 1, 2];

            let mut mesh = Mesh::new(vertices, indices).unwrap();
            assert!(!mesh.is_empty());

            mesh.clear();

            assert!(mesh.is_empty());
            assert_eq!(mesh.vertex_count(), 0);
            assert_eq!(mesh.index_count(), 0);
            assert_eq!(mesh.triangle_count(), 0);
            assert!(mesh.vertices().is_empty());
            assert!(mesh.indices().is_empty());
        }
    }
}
