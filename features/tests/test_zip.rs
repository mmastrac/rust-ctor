use ::features::perform::*;
use ::features::*;

__test!(__zip: (
    (1 2 3 4 ) (a b c d ) ("a" "b" "c" "d" )
) => ((1 a "a")(2 b "b")(3 c "c")(4 d "d")));
