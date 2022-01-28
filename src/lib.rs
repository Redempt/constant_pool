#[macro_export]
macro_rules! constant_pool {
    (
        $(#[$outermeta:meta])*
        $vis:vis $name:ident : $typ:ident {
            $(
                $(#[$innermeta:meta])*
                $cname:ident {
                    $($field:ident : $fieldval:expr),*
                }
            ),* $(,)?
        }
    ) => {
        $(#[$outermeta])*
        $vis mod $name {
            use super::*;
            $(
                $(#[$innermeta])*
                $vis const $cname: $typ = $typ {
                    name: stringify!($cname),
                    $(
                        $field: $fieldval
                    ),*
                };
            )*
            $vis static values: &[&$typ] = &[
                $(
                    &$name::$cname
                ),*
            ];
            $vis fn by_name(name: &str) -> Option<&'static $typ> {
                match name {
                    $(
                        stringify!($cname) => Some(&$name::$cname),
                    )*
                    _ => None,
                }
            }
        }
    };
    (
        $(#[$outermeta:meta])*
        $vis:vis $name:ident : $typ:ident {
            $(
                $(#[$innermeta:meta])*
                $cname:ident (
                    $($fieldval:expr),*
                )
            ),* $(,)?
        }
    ) => {
        $(#[$outermeta])*
        $vis mod $name {
            use super::*;
            $(
                $(#[$innermeta])*
                $vis const $cname: $typ = $typ (
                    stringify!($cname),
                    $(
                        $fieldval
                    ),*
                );
            )*
            $vis static values: &[&$typ] = &[
                $(
                    &$name::$cname
                ),*
            ];
            $vis fn by_name(name: &str) -> Option<&'static $typ> {
                match name {
                    $(
                        stringify!($cname) => Some(&$name::$cname),
                    )*
                    _ => None,
                }
            }
        }
    }
}