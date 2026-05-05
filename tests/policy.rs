use signal_dev_doc_deck::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 62, capacity: 73, latency: 23, risk: 22, weight: 6 };
    assert_eq!(score(signal), 14);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 89, capacity: 80, latency: 15, risk: 12, weight: 4 };
    assert_eq!(score(signal), 153);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 68, capacity: 90, latency: 14, risk: 12, weight: 6 };
    assert_eq!(score(signal), 130);
    assert_eq!(classify(signal), "review");
}
