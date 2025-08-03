impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        //linear solution by reorganizing the connections data.
        let mut visited = vec![false; n as usize];
        //this runs off edges being a mapping of city to all of that city's connections
        let mut edges = Vec::with_capacity(n as usize);
        edges.resize(n as usize, Vec::new());
        for c in connections{
            edges[c[0] as usize].push(c[1]);
            edges[c[1] as usize].push(-c[0]);
        }

        Self::dfs(0, &edges, &mut visited)
    }

    fn dfs(cur: i32, edges: &Vec<Vec<i32>>, visited: &mut Vec<bool>) -> i32{
        let mut count = 0;
        visited[cur as usize] = true;
        for city in edges[cur as usize].iter(){
            if visited[city.abs() as usize]{ continue; }
            if *city > 0{
                count += 1;
            }
            count += Self::dfs(city.abs(), edges, visited);
        }

        count
    }
}