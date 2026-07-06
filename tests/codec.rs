use nota_text_query::{Query, QueryTerm, WordDistance};

#[cfg(feature = "nota-text")]
use nota::NotaSource;

#[test]
fn query_round_trips_through_rkyv_bytes() {
    let query = Query::all_of(vec![
        Query::contains(QueryTerm::word("typed")),
        Query::near(
            QueryTerm::word("search"),
            QueryTerm::phrase(vec!["language".to_owned(), "surface".to_owned()]),
            WordDistance::new(3),
        ),
    ]);

    let bytes = query.to_rkyv_bytes().unwrap();
    let decoded = Query::from_rkyv_bytes(bytes.as_slice()).unwrap();

    assert_eq!(decoded, query);
}

#[test]
#[cfg(feature = "nota-text")]
fn query_decodes_from_readable_nota_text() {
    let source = NotaSource::new(
        "(AllOf [(Contains (Word (typed))) (Near ((Word (search)) (Word (language)) 3))])",
    );

    let query: Query = source.parse().unwrap();

    assert_eq!(
        query,
        Query::all_of(vec![
            Query::contains(QueryTerm::word("typed")),
            Query::near(
                QueryTerm::word("search"),
                QueryTerm::word("language"),
                WordDistance::new(3)
            ),
        ])
    );
}
