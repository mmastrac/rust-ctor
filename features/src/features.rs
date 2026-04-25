
/// A macro that generates the appropriate feature extraction macros.
#[macro_export]
#[doc(hidden)]
macro_rules! __declare_features {
    ( $($input:tt)* ) => {
        $crate::__perform!(
            ($($input)*),
            $crate::__chain[
                $crate::__parse_feature_input,
                $crate::__parallel[
                    $crate::__identity,
                    $crate::__feature_square,
                ],
                // (params) (all) (crate) (attr) (both) (feature_square)
                $crate::__parallel[
                    $crate::__chain[
                        $crate::__pick[0 2 3 5],
                        $crate::__make_macros[$],
                    ],
                    $crate::__chain[
                        $crate::__pick[4],
                        $crate::__make_docs[@crate],
                    ],
                    $crate::__chain[
                        $crate::__pick[3],
                        $crate::__make_docs[@attr],
                    ],
                ],
            ]
        );
    };
}

/// Parses a single, generic item decorated with an attribute macro into the
/// main attribute features, other meta items, and the original item.
#[macro_export]
#[doc(hidden)]
macro_rules! __parse_item {
    ( @entry next=$next:path[$next_args:tt], input=($($item:tt)*), args=[$macro_name:path]) => {
        $crate::__chain!(@entry next=$next[$next_args], input=($($item)*), args=[
            $crate::__parallel[
                // Split meta from item and process them separately
                $crate::__chain[
                    $crate::__split_meta,
                    // (meta) (item)
                    $crate::__separate[
                        // (meta)
                        $crate::__parallel[
                            // Generate crate features
                            $crate::__call[$macro_name[$macro_name => @crate]],
                            // Extract meta items
                            $crate::__chain[
                                // (meta)
                                $crate::__call[$macro_name[$macro_name => @self]],
                                // (self)(other)
                                $crate::__separate[
                                    // (self)
                                    $crate::__call[$macro_name[$macro_name => @meta]],
                                    // (other)
                                    $crate::__brace[()],
                                ],
                            ],
                        ],
                        // (item)
                        $crate::__identity,
                    ],
                ],
            ],
            // Assembles the final parsed item
            // input: features #[other_meta] item
            $crate::__finish_item,
        ]);
    };
}

/// Finishes the item parsing by collecting the features, meta, and item.
#[macro_export]
#[doc(hidden)]
macro_rules! __finish_item {
    ( @entry next=$next:path[$next_args:tt], input=(
        $($feature:ident = $feature_value:tt,)*
        ($(#[$other_meta:meta])*)
        $($item:tt)*
    ) ) => {
        $next ! ( $next_args, (
            features = ($($feature = $feature_value,)*),
            meta = ($(#[$other_meta])*),
            item = ($($item)*)
        ) );
    };
}

/// Splits the item input into meta and item.
#[macro_export]
#[doc(hidden)]
macro_rules! __split_meta {
    ( @entry next=$next:path[$next_args:tt], input=($($item:tt)*) ) => {
        $crate::__split_meta!(@loop meta=(), rest=($($item)*), next=[$next[$next_args]]);
    };

    ( @loop meta=($($metas:tt)*), rest=(#$meta:tt $($item:tt)*), next=$next:tt ) => {
        $crate::__split_meta!(@loop meta=($($metas)* #$meta), rest=($($item)*), next=$next);
    };

    ( @loop meta=($($meta:tt)*), rest=($item:item), next=[$next:path[$next_args:tt]]) => {
        $next ! ( $next_args, (($($meta)*) ($item)) );
    };

    ( $($input:tt)* ) => {
        const _: () = { compile_error!(concat!("Unexpected input: ", stringify!($($input)*))); };
    };
}

/// Parses a "pretty" feature specification into something easier to work with.
#[macro_export]
#[doc(hidden)]
macro_rules! __parse_feature_input {
    ( @entry next=$next:path[$next_args:tt], input=(
        $macro_name:ident: $macro_parse:ident => $macro_impl:ident;

        $(
            $( #[doc = $doc:literal] )*
            $feature:ident {
                $(
                    feature: $feature_name:literal = $feature_include_macro:ident;
                )?
                $(
                    attr: $attr:tt;
                    $( default $default:tt )?
                )?
            };
        )*
    )) => {
        $next ! ( $next_args, (
            (
                $macro_name
                $macro_parse
                $macro_impl
            )
            (
                $(
                    $feature,
                )*
            )
            (
                $(
                    $( $feature {
                        name = $feature_name;
                        macro = $feature_include_macro;
                    } )?
                )*
            )
            (
                $(
                    $( $feature {
                        attr = $attr;
                        $( default $default )?
                    } )?
                )*
            )
            (
                $(
                    $feature {
                        $(
                            name = $feature_name;
                            macro = $feature_include_macro;
                        )?
                        $(
                            attr = $attr;
                            $( default $default )?
                        )?
                    }
                )*
            )
        ) );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __feature_square {
    ( @entry next=$next:path[$next_args:tt], input=(
        $params:tt
        ($($all:ident,)*)
        $($rest:tt)*
    ) ) => {
        $crate::__feature_square!( @loop accum=[] queue=[$($all)*] mult=[$($all)*] final=[$next[$next_args]] );
    };
    ( @loop accum=[$($accum:tt)*] queue=[] mult=$mult:tt final=[$next:path[$next_args:tt]] ) => {
        $next ! ( $next_args, (($($accum)*)) );
    };
    ( @loop accum=[$($accum:tt)*] queue=[$first:ident $($rest:ident)*] mult=[$($mult:ident)*] final=$final:tt ) => {
        $crate::__feature_square!( @loop accum=[$($accum)* ($first $($mult)*)] queue=[$($rest)*] mult=[$($mult)*] final=$final );
    };
    ( $($input:tt)* ) => {
        const _: () = { compile_error!(concat!("Unexpected input: ", stringify!($($input)*))); };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __extract_features {
    ( @entry macro=$macro_parse:path, next=$next:tt, features=$features:tt, all_features=$all_features:tt) => {
        $crate::__extract_features!(@loop accum=(), macro=$macro_parse, next=$next, features=$features, all_features=$all_features);
    };
    ( @loop accum=$accum:tt, macro=$macro_parse:path, next=($next:path[$next_args:tt]), features=(), all_features=$all_features:tt) => {
        $next ! ( $next_args, $accum );
    };
    ( @loop accum=$accum:tt, macro=$macro_parse:path, next=$next:tt, features=($feature:ident $($feature_rest:tt)*), all_features=$all_features:tt) => {
        $macro_parse!(@extract next=__extract_features[(@cont accum=$accum, macro=$macro_parse, next=$next, features=($($feature_rest)*), all_features=$all_features)] $feature $all_features);
    };
    ( (@cont accum=($($accum:tt)*), $($args:tt)*), ($name:ident = $output:tt $($extra:tt)+) ) => {
        $crate::__extract_features!((@cont accum=($($accum)*), $($args)*), ($name = $($extra)+));
    };
    ( (@cont accum=($($accum:tt)*), $($args:tt)*), ($name:ident = $output:tt) ) => {
        $crate::__extract_features!(@loop accum=($($accum)* $name = $output ,), $($args)*);
    };
    ( (@cont accum=($($accum:tt)*), $($args:tt)*), ($name:ident = ) ) => {
        $crate::__extract_features!(@loop accum=($($accum)* $name = () ,), $($args)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __extract_meta {
    ( @entry macro=$macro_parse:path, next=$next:tt, meta=$meta:tt) => {
        $crate::__extract_meta!(@loop accum=(), meta=$meta, macro=$macro_parse, next=$next);
    };
    ( @loop accum=$accum:tt, meta=(), macro=$macro_parse:path, next=($next:path[$next_args:tt])) => {
        $next ! ( $next_args, $accum );
    };
    ( @loop accum=$accum:tt, meta=($meta:ident $( = $($eq_args:tt)* )? $( ( $($args:tt)* ) )? $(, $($rest:tt)+ )?), macro=$macro_parse:path, next=$next:tt) => {
        $macro_parse!(@meta next=$crate::__extract_meta[(@cont accum=$accum, meta=($($($rest)*)?), macro=$macro_parse, next=$next)] $meta $( = $($eq_args)* )? $(($($args)*))?);
    };
    ( (@cont accum=($($accum:tt)*), $($args:tt)*), ($($output:tt)*) ) => {
        $crate::__extract_meta!(@loop accum=($($accum)* $($output)* ,), $($args)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __extract_self {
    ( @entry macro=$macro_parse:path, next=$next:tt, input=$input:tt) => {
        $crate::__extract_self!(@loop self=(), other=(), input=$input, macro=$macro_parse, next=$next);
    };
    ( @loop self=$self:tt, other=$other:tt, input=(), macro=$macro_parse:path, next=($next:path[$next_args:tt])) => {
        $next ! ( $next_args, ( $self $other ) );
    };
    ( @loop self=$self:tt, other=$other:tt, input=(# $meta:tt $($rest:tt)*), macro=$macro_parse:path, next=$next:tt) => {
        $macro_parse!(@self next=$crate::__extract_self[(@cont self=$self, other=$other, input=($($rest)*), macro=$macro_parse, next=$next)], input=(# $meta));
    };
    ( (@cont self=$self:tt, other=$other:tt, $($args:tt)*), (self = ($($output:tt)*)) ) => {
        $crate::__extract_self!(@loop self=($($output)*), other=$other, $($args)*);
    };
    ( (@cont self=$self:tt, other=($($other:tt)*), $($args:tt)*), (other = ($($output:tt)*)) ) => {
        $crate::__extract_self!(@loop self=$self, other=($($other)* $($output)*), $($args)*);
    };
}

/// Generates the various utility macros used to parse the actual macro under
/// the hood.
#[macro_export]
#[doc(hidden)]
macro_rules! __make_macros {
    ( @entry next=$next:path[$next_args:tt], input=(
        (
            $macro_name:ident
            $macro_parse:ident
            $macro_impl:ident
        )
        (
            $(
                $feature:ident {
                    name = $name:literal;
                    macro = $feature_macro:ident;
                }
            )*
        )
        (
            $(
                $feature_attr:ident {
                    attr = [($( $attr:tt )*) => ($( $attr_output:tt )*)];
                    $( default $default:tt )?
                }
            )*
        )
        (
            $(
                ($feature_sq_1:ident $($feature_sq_2:ident)*)
            )*
        )
    ), args=[$dollar:tt]) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! $macro_parse {
            // @extract takes the full or partial feature tuple and extracts one
            // feature at a time. If present, the next macro receives
            // (name = $feature_value:tt), otherwise ().
            $(
                (@extract next=$next_macro:path[$next_macro_args:tt] $feature_sq_1 
                    (
                        $dollar (
                        $(
                            $dollar ( $feature_sq_2 = $dollar $feature_sq_2:tt)? 
                        )* ,
                        )*
                    )) => {
                    $next_macro ! ( $next_macro_args, (
                        $feature_sq_1 = $dollar ( $dollar ( $dollar $feature_sq_1 )? )*
                    ) );
                };
            )*

            (@extract next=$next_macro:path[$next_macro_args:tt] $dollar feature:ident $dollar ($dollar rest:tt)*) => {
                const _: () = { compile_error!(concat!("Unexpected input: ", stringify!($dollar feature))); };
            };

            // @meta extracts the meta items from the proc-macro attribute.
            $(
                (@meta next=$next_macro:path[$next_macro_args:tt] $($attr)*) => {
                    $next_macro ! ( $next_macro_args, ( $feature_attr = $($attr_output)* ) );
                };
            )*

            (@meta next=$next_macro:path[$next_macro_args:tt] $meta:ident $dollar ($dollar rest:tt)*) => {
                const _: () = { compile_error!(concat!("Unexpected meta attribute: ", stringify!($meta))); };
            };

            // @self extracts the self-attribute from a list of attributes.
            (@self next=$next_macro:path [$next_macro_args:tt], input=(# [ $macro_name ])) => {
                $next_macro ! ( $next_macro_args, ( self = (()) ) );
            };
            (@self next=$next_macro:path [$next_macro_args:tt], input=(# [ $macro_name $args:tt ])) => {
                $next_macro ! ( $next_macro_args, ( self = ($args) ) );
            };
            (@self next=$next_macro:path [$next_macro_args:tt], input=$input:tt) => {
                $next_macro ! ( $next_macro_args, ( other = $input ) );
            };

            // Extracts all features specified in $all_features and passes them
            // to the next macro.
            (@entry next=$next_macro:path [$next_macro_args:tt],
                input=$all_features:tt,
                args=[$macro:path => @extract $features:tt]) => {
                $crate::__extract_features!(@entry macro=$macro, next=($next_macro [$next_macro_args]),
                    features=$features, all_features=$all_features);
            };

            // Extracts features from enabled crate features.
            (@entry next=$next_macro:path [$next_macro_args:tt],
                input=$all_features:tt,
                args=[$macro:path => @crate]) => {
                $crate::__chain!(@entry next=$next_macro[$next_macro_args], input=(), args=[
                    $($macro_name::$feature_macro,)*
                ]);
            };
            
            // Extracts features from meta items.
            (@entry next=$next_macro:path [$next_macro_args:tt],
                input=($meta:tt),
                args=[$macro:path => @meta]) => {
                $crate::__extract_meta!(@entry macro=$macro, next=($next_macro [$next_macro_args]),
                    meta=$meta);
            };

            // Extracts the self-attribute from a list of attributes.
            (@entry next=$next_macro:path [$next_macro_args:tt],
                input=($dollar ( # $dollar attr:tt )*),
                args=[$macro:path => @self]) => {
                $crate::__extract_self!(@entry macro=$macro, next=($next_macro [$next_macro_args]), input=($dollar ( # $dollar attr )*));
            };

            (@entry $dollar ($rest:tt)*) => {
                const _: () = { compile_error!(concat!("Unexpected input: ", stringify!($dollar ($rest)*))); };
            };
        }

        mod $macro_name {
            $(
                #[doc(hidden)]
                #[macro_export]
                #[cfg(feature = $name)]
                macro_rules! $feature_macro {
                    (@entry next=$next_macro:path [$next_macro_args:tt], input=($dollar ($input:tt)*)) => {
                        $next_macro ! ( $next_macro_args, ($dollar ($input)* $feature = $feature, ) );
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                #[cfg(not(feature = $name))]
                macro_rules! $feature_macro {
                    (@entry next=$next_macro:path [$next_macro_args:tt], input=($dollar ($input:tt)*)) => {
                        $next_macro ! ( $next_macro_args, ($dollar ($input)* ) );
                    };
                }

                #[allow(unused_imports)]
                pub use $feature_macro;
            )*

            #[allow(unused_imports)]
            pub use $macro_parse;
        }

        $next ! ( $next_args, () );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __make_docs {
    ( @entry next=$next:path[$next_args:tt], input=((
        $(
            $feature:ident {
                $(
                    name = $feature_name:literal;
                    macro = $feature_include_macro:ident;
                )?
                $(
                    attr = [($( $attr:tt )*) => ($( $attr_output:tt )*)];
                    $( default $default:tt )?
                )?
            }
        )*
    )), args=[@crate] ) => {
        $next ! ( $next_args, (
            $(
                $(
                    #[doc = concat!("<code>", stringify!($feature_name), "</code>: ")]
                    #[doc = "\n"]
                )?
            )*
            const _: () = {  };
        ) );
    };

    ( @entry next=$next:path[$next_args:tt], input=(
        $(
            $input:tt
        )*
    ), args=[@attr] ) => {
        $next ! ( $next_args, () );
    };

    ( $($input:tt)* ) => {
        const _: () = { compile_error!(concat!("Unexpected input: ", stringify!($($input)*))); };
    };
}
