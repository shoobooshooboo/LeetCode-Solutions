use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        //This problem is secretely a directed-weighted-graph problem in disguise. don't let it fool you.
        let mut graph = HashMap::<String, HashMap<String, f64>>::new();

        //make the actual graph
        for (eq, result) in equations.into_iter().zip(values.into_iter()){
            graph.entry(eq[0].clone()).or_insert(HashMap::new()).insert(eq[1].clone(), result);
            graph.entry(eq[1].clone()).or_insert(HashMap::new()).insert(eq[0].clone(), 1.0 / result);
        }

        let mut results = Vec::new();
        for q in queries.iter(){
            if !graph.contains_key(&q[0]) || !graph.contains_key(&q[1]){ 
                results.push(-1.0); 
                continue;
            }
            let mut visited = HashSet::new();
            results.push(Self::dfs(&graph, &q[0], &q[1], &mut visited).unwrap_or(-1.0));
        }

        results
    }

    fn dfs(graph: &HashMap<String, HashMap<String, f64>>, cur: &String, target: &String, visited: &mut HashSet<String>) -> Option<f64>{
        let paths = &graph[cur];

        //if this graph node can see the target, we can stop recursing
        if let Some(weight) = paths.get(target){
            return Some(*weight);
        }

        //now just a regular dfs
        visited.insert(cur.clone());
        for (next, weight) in paths{
            if visited.contains(next){
                continue;
            }
            if let Some(w) = Self::dfs(graph, next, target, visited){
                return Some(*weight * w);
            }
        }

        None
    }
}