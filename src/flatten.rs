trait Flatten {
    type Output;
    fn flatten(self) -> Self::Output;
}

impl<A, B> Flatten for (A, (B, ())) {
    type Output = (A, B);
    fn flatten(self) -> Self::Output {
        (self.0, self.1.flatten())
    }
}

impl<A> Flatten for (A, ()) {
    type Output = A;
    fn flatten(self) -> Self::Output {
        self.0
    }
}
