//! Utility macro for converting priority values to literals.

/// Convert a priority value to a literal if 0 < N < 1000, otherwise pass through the value.
#[doc(hidden)]
#[macro_export]
macro_rules! __priority_to_literal {
    ($n:path,$a:tt,0) => {
        $n!($a, "000");
    };
    ($n:path,$a:tt,1) => {
        $n!($a, "001");
    };
    ($n:path,$a:tt,2) => {
        $n!($a, "002");
    };
    ($n:path,$a:tt,3) => {
        $n!($a, "003");
    };
    ($n:path,$a:tt,4) => {
        $n!($a, "004");
    };
    ($n:path,$a:tt,5) => {
        $n!($a, "005");
    };
    ($n:path,$a:tt,6) => {
        $n!($a, "006");
    };
    ($n:path,$a:tt,7) => {
        $n!($a, "007");
    };
    ($n:path,$a:tt,8) => {
        $n!($a, "008");
    };
    ($n:path,$a:tt,9) => {
        $n!($a, "009");
    };
    ($n:path,$a:tt,10) => {
        $n!($a, "010");
    };
    ($n:path,$a:tt,11) => {
        $n!($a, "011");
    };
    ($n:path,$a:tt,12) => {
        $n!($a, "012");
    };
    ($n:path,$a:tt,13) => {
        $n!($a, "013");
    };
    ($n:path,$a:tt,14) => {
        $n!($a, "014");
    };
    ($n:path,$a:tt,15) => {
        $n!($a, "015");
    };
    ($n:path,$a:tt,16) => {
        $n!($a, "016");
    };
    ($n:path,$a:tt,17) => {
        $n!($a, "017");
    };
    ($n:path,$a:tt,18) => {
        $n!($a, "018");
    };
    ($n:path,$a:tt,19) => {
        $n!($a, "019");
    };
    ($n:path,$a:tt,20) => {
        $n!($a, "020");
    };
    ($n:path,$a:tt,21) => {
        $n!($a, "021");
    };
    ($n:path,$a:tt,22) => {
        $n!($a, "022");
    };
    ($n:path,$a:tt,23) => {
        $n!($a, "023");
    };
    ($n:path,$a:tt,24) => {
        $n!($a, "024");
    };
    ($n:path,$a:tt,25) => {
        $n!($a, "025");
    };
    ($n:path,$a:tt,26) => {
        $n!($a, "026");
    };
    ($n:path,$a:tt,27) => {
        $n!($a, "027");
    };
    ($n:path,$a:tt,28) => {
        $n!($a, "028");
    };
    ($n:path,$a:tt,29) => {
        $n!($a, "029");
    };
    ($n:path,$a:tt,30) => {
        $n!($a, "030");
    };
    ($n:path,$a:tt,31) => {
        $n!($a, "031");
    };
    ($n:path,$a:tt,32) => {
        $n!($a, "032");
    };
    ($n:path,$a:tt,33) => {
        $n!($a, "033");
    };
    ($n:path,$a:tt,34) => {
        $n!($a, "034");
    };
    ($n:path,$a:tt,35) => {
        $n!($a, "035");
    };
    ($n:path,$a:tt,36) => {
        $n!($a, "036");
    };
    ($n:path,$a:tt,37) => {
        $n!($a, "037");
    };
    ($n:path,$a:tt,38) => {
        $n!($a, "038");
    };
    ($n:path,$a:tt,39) => {
        $n!($a, "039");
    };
    ($n:path,$a:tt,40) => {
        $n!($a, "040");
    };
    ($n:path,$a:tt,41) => {
        $n!($a, "041");
    };
    ($n:path,$a:tt,42) => {
        $n!($a, "042");
    };
    ($n:path,$a:tt,43) => {
        $n!($a, "043");
    };
    ($n:path,$a:tt,44) => {
        $n!($a, "044");
    };
    ($n:path,$a:tt,45) => {
        $n!($a, "045");
    };
    ($n:path,$a:tt,46) => {
        $n!($a, "046");
    };
    ($n:path,$a:tt,47) => {
        $n!($a, "047");
    };
    ($n:path,$a:tt,48) => {
        $n!($a, "048");
    };
    ($n:path,$a:tt,49) => {
        $n!($a, "049");
    };
    ($n:path,$a:tt,50) => {
        $n!($a, "050");
    };
    ($n:path,$a:tt,51) => {
        $n!($a, "051");
    };
    ($n:path,$a:tt,52) => {
        $n!($a, "052");
    };
    ($n:path,$a:tt,53) => {
        $n!($a, "053");
    };
    ($n:path,$a:tt,54) => {
        $n!($a, "054");
    };
    ($n:path,$a:tt,55) => {
        $n!($a, "055");
    };
    ($n:path,$a:tt,56) => {
        $n!($a, "056");
    };
    ($n:path,$a:tt,57) => {
        $n!($a, "057");
    };
    ($n:path,$a:tt,58) => {
        $n!($a, "058");
    };
    ($n:path,$a:tt,59) => {
        $n!($a, "059");
    };
    ($n:path,$a:tt,60) => {
        $n!($a, "060");
    };
    ($n:path,$a:tt,61) => {
        $n!($a, "061");
    };
    ($n:path,$a:tt,62) => {
        $n!($a, "062");
    };
    ($n:path,$a:tt,63) => {
        $n!($a, "063");
    };
    ($n:path,$a:tt,64) => {
        $n!($a, "064");
    };
    ($n:path,$a:tt,65) => {
        $n!($a, "065");
    };
    ($n:path,$a:tt,66) => {
        $n!($a, "066");
    };
    ($n:path,$a:tt,67) => {
        $n!($a, "067");
    };
    ($n:path,$a:tt,68) => {
        $n!($a, "068");
    };
    ($n:path,$a:tt,69) => {
        $n!($a, "069");
    };
    ($n:path,$a:tt,70) => {
        $n!($a, "070");
    };
    ($n:path,$a:tt,71) => {
        $n!($a, "071");
    };
    ($n:path,$a:tt,72) => {
        $n!($a, "072");
    };
    ($n:path,$a:tt,73) => {
        $n!($a, "073");
    };
    ($n:path,$a:tt,74) => {
        $n!($a, "074");
    };
    ($n:path,$a:tt,75) => {
        $n!($a, "075");
    };
    ($n:path,$a:tt,76) => {
        $n!($a, "076");
    };
    ($n:path,$a:tt,77) => {
        $n!($a, "077");
    };
    ($n:path,$a:tt,78) => {
        $n!($a, "078");
    };
    ($n:path,$a:tt,79) => {
        $n!($a, "079");
    };
    ($n:path,$a:tt,80) => {
        $n!($a, "080");
    };
    ($n:path,$a:tt,81) => {
        $n!($a, "081");
    };
    ($n:path,$a:tt,82) => {
        $n!($a, "082");
    };
    ($n:path,$a:tt,83) => {
        $n!($a, "083");
    };
    ($n:path,$a:tt,84) => {
        $n!($a, "084");
    };
    ($n:path,$a:tt,85) => {
        $n!($a, "085");
    };
    ($n:path,$a:tt,86) => {
        $n!($a, "086");
    };
    ($n:path,$a:tt,87) => {
        $n!($a, "087");
    };
    ($n:path,$a:tt,88) => {
        $n!($a, "088");
    };
    ($n:path,$a:tt,89) => {
        $n!($a, "089");
    };
    ($n:path,$a:tt,90) => {
        $n!($a, "090");
    };
    ($n:path,$a:tt,91) => {
        $n!($a, "091");
    };
    ($n:path,$a:tt,92) => {
        $n!($a, "092");
    };
    ($n:path,$a:tt,93) => {
        $n!($a, "093");
    };
    ($n:path,$a:tt,94) => {
        $n!($a, "094");
    };
    ($n:path,$a:tt,95) => {
        $n!($a, "095");
    };
    ($n:path,$a:tt,96) => {
        $n!($a, "096");
    };
    ($n:path,$a:tt,97) => {
        $n!($a, "097");
    };
    ($n:path,$a:tt,98) => {
        $n!($a, "098");
    };
    ($n:path,$a:tt,99) => {
        $n!($a, "099");
    };
    ($n:path,$a:tt,$priority:literal) => {
        $n!($a, $priority);
    };
}
