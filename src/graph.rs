use std::io;
use std::str;
use std::num;

use crate::matrix::Matrix;

pub struct Graph {
    pub adjacency_list: Vec<Vec<usize>>
}

impl Graph {
    pub fn from_adjacency_list(adj_list: Vec<Vec<usize>>) -> Self {
        Self {
            adjacency_list: adj_list
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn edges_count(&self) -> usize {
        self.adjacency_list.iter()
            .map(|adjacents| adjacents.len())
            .sum()
    }

    pub fn outdegree(&self, vertex: usize) -> usize {
        self.adjacency_list[vertex].len()
    }

    pub fn indegree(&self, vertex: usize) -> usize {
        self.adjacency_list
            .iter()
            .map(|adjacents| {
                adjacents
                    .iter()
                    .filter(|&&v| v == vertex)
                    .count()
            })
            .sum::<usize>()
    }

    pub fn degree(&self, vertex: usize) -> usize {
        self.outdegree(vertex) + self.indegree(vertex)
    }

    pub fn to_adjacency_matrix(&self) -> Matrix<usize> {
        let mut mat = Matrix::new(0, self.vertex_count(), self.vertex_count());

        for (i, vertexes) in self.adjacency_list.iter().enumerate() {
            for &vertex in vertexes.iter() {
                // элемент матрицы - количество ребер инцидентных вершинам i j
                mat[(i, vertex)] += 1;
                /*if i != vertex {
                    mat[(vertex, i)] += 1;
                }*/
            }
        }
        mat
    }

    pub fn to_incidence_matrix(&self) -> Matrix<i8> {
        let n_edges = self.edges_count();
        let mut mat = Matrix::new(0, self.vertex_count(), n_edges);
        let mut edge_id = 0;
        for (i, vertexes) in self.adjacency_list.iter().enumerate() {
            for &vertex in vertexes.iter() {
                mat[(i, edge_id)] = -1;
                mat[(vertex, edge_id)] = 1;
                /*if vertex != i {
                    mat[(i, edge_id)] = -1;
                    mat[(vertex, edge_id)] = 1;
                } else {
                    mat[(i, edge_id)] = 2;
                }*/
                edge_id += 1;
            }
        }
        mat
    }

    pub fn to_dot<W: std::io::Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "digraph {{")?;
        for (i, adjacents) in self.adjacency_list.iter().enumerate() {
            for &j in adjacents.iter() {
                writeln!(w, "  {} -> {};", i + 1, j + 1)?;
            }
        }
        writeln!(w, "}}")?;
        Ok(())
    }
}

impl str::FromStr for Graph {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let adj_list: Vec<Vec<usize>> =
            s
            .lines()
            .map(|line| {
                line
                    .split(' ')
                    .filter(|num_str| !num_str.is_empty())
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect()
            }).collect();


        Ok(Self::from_adjacency_list(adj_list))
    }
}