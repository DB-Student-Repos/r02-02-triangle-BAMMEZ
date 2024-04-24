#[derive(Debug)]
pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let [a, b, c] = sides;
        // Check if it's a valid triangle
        if a == 0 || b == 0 || c == 0 || a + b <= c || a + c <= b || b + c <= a {
            None // Not a valid triangle
        } else {
            Some(Triangle(a, b, c))
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let &Triangle(a, b, c) = self;
        a == b && b == c
    }

    pub fn is_isosceles(&self) -> bool {
        let &Triangle(a, b, c) = self;
        a == b || b == c || a == c
    }

    pub fn is_scalene(&self) -> bool {
        let &Triangle(a, b, c) = self;
        a != b && b != c && a != c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equilateral_triangles() {
        assert!(Triangle::build([2, 2, 2]).unwrap().is_equilateral());
    }

    #[test]
    #[ignore]
    fn test_not_equilateral_triangles() {
        assert!(!Triangle::build([2, 3, 4]).unwrap().is_equilateral());
    }

    #[test]
    fn test_isosceles_triangles() {
        assert!(Triangle::build([5, 5, 3]).unwrap().is_isosceles());
    }

    #[test]
    #[ignore]
    fn test_not_isosceles_triangles() {
        assert!(!Triangle::build([2, 3, 4]).unwrap().is_isosceles());
    }

    #[test]
    fn test_scalene_triangles() {
        assert!(Triangle::build([3, 4, 5]).unwrap().is_scalene());
    }

    #[test]
    #[ignore]
    fn test_not_scalene_triangles() {
        assert!(!Triangle::build([2, 2, 3]).unwrap().is_scalene());
    }

    #[test]
    fn test_invalid_triangles() {
        assert!(Triangle::build([0, 0, 0]).is_none());
        assert!(Triangle::build([3, 3, 7]).is_none());
    }
}

