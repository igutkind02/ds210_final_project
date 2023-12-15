
use std::fs::File;
use std::io::{self, BufRead};

pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex)>;


// FUNCTION TO READ TEXT FILE AND RETURN A LIST OF EDGES
pub fn read_file(_path: &str) -> Vec<(Vertex, Vertex)>{
    let mut list_edges: Vec<(Vertex, Vertex)> = Vec::new();
    let file = File::open("t_twitter.txt").expect("Could not open file");

    let mut buf_reader = std::io::BufReader::new(file).lines();
    buf_reader.next();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let line_str = line_str.replace("[","");
        let line_str = line_str.replace("]","");
        let line_str = line_str.replace(" ","");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x = v[0].parse::<i128>().unwrap();
        let y = v[1].parse::<i128>().unwrap();
        list_edges.push((x as Vertex, y as Vertex));
    }
    return list_edges;
}