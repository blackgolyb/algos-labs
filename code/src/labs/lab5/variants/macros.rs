#[macro_export]
macro_rules! impl_display_for_table {
    ($table:ident) => {
        impl<T: fmt::Display + Clone> fmt::Display for $table<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let (rows, columns) = self.get_size();
                for i in 0..rows {
                    for j in 0..columns {
                        write!(f, "{} ", self.get(i, j))?;
                    }
                    write!(f, "\n")?;
                }
                Ok(())
            }
        }
    };
}
