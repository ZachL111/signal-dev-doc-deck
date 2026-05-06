use signal_dev_doc_deck::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 74, slack: 23, drag: 32, confidence: 66 };
    assert_eq!(review_score(case), 141);
    assert_eq!(review_lane(case), "ship");
}
