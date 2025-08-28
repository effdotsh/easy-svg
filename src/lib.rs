mod generated;
pub use generated::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        let rect = Rect::new(100.0, 50.0)
            .fill("red".to_string())
            .stroke("black".to_string());

        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 50.0);
        assert_eq!(rect.fill, Some("red".to_string()));
        println!("{}", rect);
    }
}
