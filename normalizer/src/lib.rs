mod queries; 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_test() {
        use oxrdf::{Dataset, GraphName, NamedNode, Quad};
        use spareval::{QueryEvaluator, QueryResults};
        use spargebra::SparqlParser;

        let ex = NamedNode::new("http://example.com").unwrap();
        let dataset = Dataset::from_iter([Quad::new(
            ex.clone(),
            ex.clone(),
            ex.clone(),
            GraphName::DefaultGraph,
        )]);
        let query = SparqlParser::new()
            .parse_query("SELECT * WHERE { ?s ?p ?o }")
            .unwrap();
        let evaluator = QueryEvaluator::new();
        let results = evaluator.prepare(&query).execute(&dataset);
        if let QueryResults::Solutions(solutions) = results.unwrap() {
            let solutions = solutions.collect::<Result<Vec<_>, _>>().unwrap();
            assert_eq!(solutions.len(), 1);
            assert_eq!(solutions[0]["s"], ex.into());
        };
    }
}
