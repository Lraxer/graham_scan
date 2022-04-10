use graham_scan::graham;
use graham_scan::readgraph;
use graham_scan::save_edge;

fn main() {
    let input_filename = "randGraph.txt";
    let output_filename = "edge.txt";

    let mut point_vec = readgraph::open_read_graph(input_filename).unwrap();
    graham::convert_axis(&mut point_vec);

    let edge_vec = graham::scan(&point_vec).unwrap();

    save_edge::save_edge(output_filename, &edge_vec).unwrap();
}
