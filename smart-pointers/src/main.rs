fn main() {
    let mut x = 5;
    {
        let y = &mut x;

        *y += 1
    }

    assert_eq!(6, x);
}
