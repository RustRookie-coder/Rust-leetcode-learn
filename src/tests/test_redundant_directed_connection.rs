#[test]
pub fn test_redundant_directed_connection() {
    let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]];
    let res = redundant_redirected_connection(edges);
    println!("res: {:?}", res)
}

fn redundant_redirected_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let len = edges.len();
    let mut father: Vec<i32> = Vec::with_capacity(len + 1);
    let mut unionset: Vec<usize> = Vec::with_capacity(len + 1);

    let mut triangle: Option<(i32, i32, i32)> = None;
    let mut cycle: Option<(i32, i32)> = None;

    for i in 0..=len {
        father.push(i as i32);
        unionset.push(i);
    }

    for e in edges.iter() {
        let r = e[0];
        let t = e[1];
        let ru = r as usize;
        let tu = t as usize;

        if father[tu] != t { triangle = Some((father[tu], r, t)) } else {
            father[tu] = r;
            let mut rx = ru;
            let mut tx = tu;
            while unionset[rx] != rx { rx = unionset[rx]; }
            while unionset[tx] != tx { tx = unionset[tx]; }

            if rx == tx {
                cycle = Some((r, t));
            } else {
                unionset[tx] = rx;
            }
        }
    }
    if let Some((a, b, c)) = triangle {
        if let Some(_) = cycle { vec![a, c] } else { vec![b, c] }
    } else {
        if let Some((r, t)) = cycle { vec![r, t]} else { panic!() }
    }
}
// pub fn redundant_redirected_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
//     let n = edges.len();
//     struct ConUnionFind {
//         anc: Vec<i32>,
//     }
//
//     impl ConUnionFind {
//         fn new(n: usize) -> ConUnionFind {
//             let mut anc: Vec<i32> = (0..n).map(|x| x as i32).collect();
//             ConUnionFind { anc }
//         }
//
//         fn find(&mut self, x: i32) -> i32 {
//             if self.anc[x as usize] != x {
//                 self.anc[x as usize] = self.find(self.anc[x as usize]);
//             }
//             self.anc[x as usize]
//         }
//
//         fn union(&mut self, from: i32, to: i32) {
//             let idx = self.find(from) as usize;
//             self.anc[idx] = self.find(to);
//         }
//     }
//
//     let mut uf = ConUnionFind::new(n + 1);
//     let mut parent: Vec<_> = (0..=1000).collect();
//
//     let mut conflict_edge: Option<Vec<i32>> = None;
//     let mut cycle_edge: Option<Vec<i32>> = None;
//
//     for edge in edges {
//         let from = edge[0];
//         let to = edge[1];
//
//         if parent[to as usize] != to {
//             conflict_edge = Some(edge.clone());
//         } else {
//             parent[to as usize] = from;
//             if uf.find(to) == uf.find(from) {
//                 cycle_edge = Some(edge.clone());
//             } else {
//                 uf.union(from, to);
//             }
//         }
//     }
//
//     if conflict_edge.is_none() {
//         conflict_edge.unwrap_or_else(|| vec![0])
//     } else if cycle_edge.is_some() {
//         let conflict_edge_vec = conflict_edge.as_ref().unwrap();
//         vec![parent[conflict_edge_vec[1] as usize], conflict_edge_vec[1]]
//     } else {
//         conflict_edge.unwrap()
//     }
// }