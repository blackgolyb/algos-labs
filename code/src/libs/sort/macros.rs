#[macro_export]
macro_rules! init_sort {
    ($name:ident) => {
        pub struct $name {
            logger: Logger,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    logger: Logger::new(),
                }
            }
        }

        type SortArgs<'a, T> = (&'a mut Vec<T>, &'a mut $name);

        impl SortLogging for $name {
            fn logger(&mut self) -> &mut Logger {
                &mut self.logger
            }
        }
    };
}

#[macro_export]
macro_rules! sort {
    (
        $name:ident<$type:ident> $sort:expr
    ) => {
        init_sort!($name);

        impl Sort<$type> for $name {
            fn sort(&mut self, vec: &mut Vec<$type>) {
                self.logger.start();
                if !vec.is_empty() {
                    $sort((vec, self));
                }
                self.logger.end();
            }
        }
    };
    (
        $name:ident $(+ $aditional_trait:ident)* $sort:expr
    ) => {
        init_sort!($name);

        impl<T> Sort<T> for $name
        where
            T: Ord $( + $aditional_trait)*,
        {
            fn sort(&mut self, vec: &mut Vec<T>) {
                self.logger.start();
                if !vec.is_empty() {
                    $sort((vec, self));
                }
                self.logger.end();
            }
        }
    };
}
