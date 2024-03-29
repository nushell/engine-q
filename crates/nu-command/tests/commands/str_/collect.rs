use nu_test_support::{nu, pipeline};

#[test]
fn test_1() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        echo 1..5 | into string | str collect
        "#
        )
    );

    assert_eq!(actual.out, "12345");
}

#[test]
fn test_2() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        echo [a b c d] | str collect "<sep>"
        "#
        )
    );

    assert_eq!(actual.out, "a<sep>b<sep>c<sep>d");
}

#[test]
fn construct_a_path() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        echo [sample txt] | str collect "."
        "#
        )
    );

    assert_eq!(actual.out, "sample.txt");
}

#[test]
fn sum_one_to_four() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
        1..4 | each { $it } | into string | str collect "+" | math eval
        "#
        )
    );

    assert!(actual.out.contains("10"));
}
