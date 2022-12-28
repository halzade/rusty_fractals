use MandelbrotPixelState::{ActiveNew, Finished};

pub struct DomainElement {
    pub origin_re: f64,
    pub origin_im: f64,
    pub value: u32,
    pub quad: f64,
    // Element state is decided by calculation result. Alternatively: If all it's neighbours finished too long, it is going to be created as HibernatedBlack and its origin won't seed any calculation path.
    pub state: MandelbrotPixelState,
}

impl DomainElement {
    pub fn is_active_new(&self) -> bool {
        self.state == ActiveNew
    }

    public boolean isFinishedTooShort() {
    return state == FinishedTooShort;
    }

    public boolean isHibernated() {
    return state == FinishedTooShort || state == HibernatedDeepBlack;
    }

    public boolean isFinishedSuccessAny() {
    return state == FinishedSuccessPast || state == FinishedSuccess;
    }

    public boolean isFinishedSuccessPast() {
    return state == FinishedSuccessPast;
    }


    pub fn set_finished_state(&mut self, iterator: u32, q: f64) {
        self.state = Finished;
        self.quad = q;
        if iterator < 1 {
            self.value = 1;
            return;
        }
        if iterator == fractal::ITERATION_MAX {
            self.value = 0;
            return;
        }
        self.value = iterator;
    }

    public void setFinishedState(int iterator, int pathLength) {
    if (iterator == ITERATION_MAX) {
    state = FinishedTooLong;
    Stats.newElementsTooLong++;
    return;
    }
    if (pathLength < ITERATION_min) {
    state = FinishedTooShort;
    Stats.newElementsTooShort++;
    return;
    }
    state = FinishedSuccess;
    Stats.newElementsLong++;
    }

    pub fn set_average_with(&mut self, e: DomainElement) {
        self.value = (((self.value + e.value) as f64) / 2.0) as u32;
    }
}

pub fn init(re: f64, im: f64) -> DomainElement {
    DomainElement {
        origin_re: re,
        origin_im: im,
        value: 0,
        quad: 0.0,
        state: ActiveNew,
    }
}

public static MaskMandelbrotElement activeNew(double re, double im) {
return new MaskMandelbrotElement(re, im, ActiveNew);
}

public static MaskMandelbrotElement hibernatedDeepBlack(double re, double im) {
return new MaskMandelbrotElement(re, im, HibernatedDeepBlack);
}

public MaskMandelbrotPixelState state() {
return state;
}

public void past() {
if (state == FinishedSuccess) {
state = FinishedSuccessPast;
}
}

// Returns a negative integer, zero, or a positive integer as this object is less than, equal to, or greater than the specified object
@Override
public int compareTo(MaskMandelbrotElement e) {
if (this == e) return 0;
return this.state.compareTo(e.state);
}

public boolean hasWorseStateThen(MaskMandelbrotElement e) {
return this.compareTo(e) > 0;
}

public void goodPath() {
state = GoodPath;
}


#[test]
pub fn is_active_new() {}

#[test]
pub fn set_finished_state() {}

#[test]
fn test_set_average_with() {
    let mut me = DomainElement {
        origin_re: 0.0,
        origin_im: 0.0,
        value: 10,
        quad: 0.0,
        state: ActiveNew,
    };
    let other = DomainElement {
        origin_re: 0.0,
        origin_im: 0.0,
        value: 3,
        quad: 0.0,
        state: ActiveNew,
    };

    me.set_average_with(other);
    assert_eq!(me.value, 55);
}
