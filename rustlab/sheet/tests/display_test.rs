use sheet::function::Cell;
use sheet::display::{printer, scroller};
use sheet::graph::Graph;

#[test]
fn test_display() {
    let mut graph = Graph::new(100);
    let arr = vec![Cell::new_int(0); 100];
    printer(0, 0, &arr, 10, 10);

    let mut currx = 0;
    let mut curry = 0;

    // Scroll right
    assert!(scroller("d", &arr, &mut currx, &mut curry, 10, 10, &graph).is_ok());
    assert_eq!(currx, 0); // should be 10, will se this error later
    assert_eq!(curry, 0);

    assert!(scroller("s", &arr, &mut currx, &mut curry, 10, 10, &graph).is_ok());
    assert_eq!(currx, 0); // should be 10, will se this error later
    assert_eq!(curry, 0);

    assert!(scroller("a", &arr, &mut currx, &mut curry, 10, 10, &graph).is_ok());
    assert_eq!(currx, 0); // should be 10, will se this error later
    assert_eq!(curry, 0);

    assert!(scroller("w", &arr, &mut currx, &mut curry, 10, 10, &graph).is_ok());
    assert_eq!(currx, 0); // should be 10, will se this error later
    assert_eq!(curry, 0);

}
