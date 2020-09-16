

mod matrix;
mod graph;

use graph::Graph;


fn main() {
    let adj_list = vec![
        vec![1,2],
        vec![1,4],
        vec![4],
        vec![2,3,3],
        vec![0,2,3],
        vec![0,4,5],
    ];

    println!("Adjacency list:");
    for adjacents in adj_list.iter() {
        println!("{:?}", adjacents);
    }
    println!();

    let g: Graph = "
        1 2
        1 4
        4
        2 3 3
        0 2 3
        0 4 5
    ".parse().unwrap();

    let g = Graph::from_adjacency_list(adj_list);
    let adj_mat = g.to_adjacency_matrix();
    println!("Матрица смежности:\n{}", adj_mat);

    let inc_mat = g.to_incidence_matrix();
    println!("Матрица инцидентности:\n{}", inc_mat);

    println!("   Vertex    Degree  Indegree Outdegree");
    for v in 0..g.vertex_count() {
        println!("{:>9} {:>9} {:>9} {:>9}", v, g.degree(v), g.indegree(v), g.outdegree(v));
    }

    let path_len = 3;
    println!("A^k:\n{}", adj_mat.degree(path_len));

    let mut c_m1 = adj_mat.clone();
    for i in 2..g.vertex_count() {
        c_m1 += &adj_mat.degree(i);
    }
    let mut c = c_m1.clone();
    c += &adj_mat.degree(g.vertex_count());
    println!("Cn-1:\n{}", c_m1);
    println!("Cn:\n{}", c);

    let mut fout = std::fs::File::create("out.dot").unwrap();
    g.to_dot(&mut fout).unwrap();
}
