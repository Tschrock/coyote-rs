pub trait Plural {
    fn plural<A: AsRef<str>>(&self, singular: A, plural: A) -> A;
}

impl Plural for usize {
    fn plural<A: AsRef<str>>(&self, singular: A, plural: A) -> A {
        if *self == 1 {
            singular
        } else {
            plural
        }
    }
}

impl<T> Plural for Vec<T> {
    fn plural<A: AsRef<str>>(&self, singular: A, plural: A) -> A {
        self.len().plural(singular, plural)
    }
}
