use nota_text_query::{
    MatchEvidence, Occurrence, Query, QueryTerm, SearchOutcome, SearchText, WordDistance,
};

#[test]
fn word_query_matches_case_insensitive_normalized_text() {
    let text = SearchText::new("Launch the SPIRIT search surface.");
    let query = Query::contains(QueryTerm::word("spirit"));

    let outcome = query.find_in(&text);

    assert_eq!(
        outcome,
        SearchOutcome::matched(MatchEvidence::Contains(
            nota_text_query::ContainsEvidence::new(
                QueryTerm::word("spirit"),
                vec![Occurrence::new(2, 2)]
            )
        ))
    );
}

#[test]
fn phrase_query_matches_adjacent_normalized_words() {
    let text = SearchText::new("Readable, typed search language.");
    let query = Query::contains(QueryTerm::phrase(vec![
        "typed".to_owned(),
        "search".to_owned(),
    ]));

    let outcome = query.find_in(&text);

    assert!(matches!(
        outcome,
        SearchOutcome::Matched(MatchEvidence::Contains(_))
    ));
}

#[test]
fn all_of_requires_every_child_query() {
    let text = SearchText::new("typed query language");
    let query = Query::all_of(vec![
        Query::contains(QueryTerm::word("typed")),
        Query::contains(QueryTerm::word("language")),
    ]);

    let outcome = query.find_in(&text);

    assert!(matches!(
        outcome,
        SearchOutcome::Matched(MatchEvidence::AllOf(_))
    ));
}

#[test]
fn any_of_reports_matching_children_in_query_order() {
    let text = SearchText::new("search language");
    let query = Query::any_of(vec![
        Query::contains(QueryTerm::word("missing")),
        Query::contains(QueryTerm::word("search")),
        Query::contains(QueryTerm::word("language")),
    ]);

    let outcome = query.find_in(&text);

    let SearchOutcome::Matched(MatchEvidence::AnyOf(evidence)) = outcome else {
        panic!("expected any-of evidence");
    };
    assert_eq!(evidence.matches.len(), 2);
    assert!(matches!(evidence.matches[0], MatchEvidence::Contains(_)));
    assert!(matches!(evidence.matches[1], MatchEvidence::Contains(_)));
}

#[test]
fn not_matches_when_child_query_does_not_match() {
    let text = SearchText::new("public reusable search");
    let query = Query::negated(Query::contains(QueryTerm::word("spirit")));

    let outcome = query.find_in(&text);

    assert_eq!(outcome, SearchOutcome::matched(MatchEvidence::Not));
}

#[test]
fn near_matches_terms_within_word_gap() {
    let text = SearchText::new("alpha one two beta gamma beta");
    let query = Query::near(
        QueryTerm::word("alpha"),
        QueryTerm::word("beta"),
        WordDistance::new(2),
    );

    let outcome = query.find_in(&text);

    let SearchOutcome::Matched(MatchEvidence::Near(evidence)) = outcome else {
        panic!("expected near evidence");
    };
    assert_eq!(evidence.pairs.len(), 1);
    assert_eq!(evidence.pairs[0].left, Occurrence::new(0, 0));
    assert_eq!(evidence.pairs[0].right, Occurrence::new(3, 3));
    assert_eq!(evidence.pairs[0].gap, WordDistance::new(2));
}
