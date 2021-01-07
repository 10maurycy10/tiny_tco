# tiny_tco

A tiny dirt simple no_std tail call optimization library.

## how to use

    // y is the acoumulator for the value
    let fact = tco(|(x,y): (i32,i32)|
    	if (x == 0) {
    		// if we have reached 0 return computed value
    		TCO::Ret(y)
    	} else {
    		// reduce x by 1, and multiplyx value by x
    		TCO::Rec((x-1,y*x))
    	},
    );
    assert_eq!(fact((3,1)),6);