use std::process::{Command};

mod matrix;
mod graph;

use graph::Graph;


fn main() {
    let g: Graph = "
        1 2
        1 4
        4
        2 3 3
        0 2 3
        0 4 5
    ".parse().expect("wrong adjacency list representation");

    let adj_mat = g.to_adjacency_matrix();
    println!("Матрица смежности:\n{}", adj_mat);

    let inc_mat = g.to_incidence_matrix();
    println!("Матрица инцидентности:\n{}", inc_mat);

    println!("   Vertex    Degree  Indegree Outdegree");
    for v in 0..g.vertex_count() {
        println!("{:>9} {:>9} {:>9} {:>9}", v, g.degree(v), g.indegree(v), g.outdegree(v));
    }

    let path_len = 3;
    println!("A^k:\n{}", adj_mat.pow(path_len));

    let mut c_m1 = adj_mat.clone();
    for i in 2..g.vertex_count() {
        c_m1 += &adj_mat.pow(i);
    }
    let mut c = c_m1.clone();
    c += &adj_mat.pow(g.vertex_count());
    println!("Cn-1:\n{}", c_m1);
    println!("Cn:\n{}", c);

    let dot_file = "/tmp/out.dot";
    let svg_file = dot_file.to_string() + ".svg";

    let mut fout = std::fs::File::create(dot_file).unwrap();
    g.to_dot(&mut fout).unwrap();

    Command::new("dot").args(&[dot_file, "-Tsvg", "-Osvg"]).output().unwrap();
    Command::new("xdg-open").args(&[svg_file]).output().unwrap();
}
