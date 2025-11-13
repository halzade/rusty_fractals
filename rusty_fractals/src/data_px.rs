use crate::pixel_states::DomainElementState;
use crate::pixel_states::DomainElementState::{
    ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort,
    HibernatedDeepBlack,
};
use image::Rgb;
use std::sync::RwLock;

pub struct DataPx {
    is_alive: RwLock<bool>,
    data: RwLock<Data>,
}

#[derive(Clone, Copy)]
pub struct Data {
    origin_re: f64,
    origin_im: f64,
    value: u32,
    /* Element state is decided by calculation result.
     * Alternatively: If all it's neighbours finished too long, it is going to be
     * created as HibernatedBlack and its origin won't seed any calculation path.
     */
    state: DomainElementState,
    quad: f64,
    color: Option<Rgb<u8>>,
}

impl DataPx {
    pub fn add_v1(&self) {
        self.data.write().unwrap().value += 1;
    }

    pub fn set_v(&self, value: u32) {
        self.data.write().unwrap().value = value;
    }

    pub fn set_qsv(&self, quad: f64, state: DomainElementState, value: u32) {
        let mut d = self.data.write().unwrap();
        d.quad = quad;
        d.state = state;
        d.value = value;
    }

    pub fn set_qs(&self, quad: f64, state: DomainElementState) {
        let mut d = self.data.write().unwrap();
        d.quad = quad;
        d.state = state;
    }

    pub fn set_c(&self, color: Rgb<u8>) {
        self.data.write().unwrap().color = Some(color);
    }

    pub fn get_vsqc(&self) -> (u32, DomainElementState, f64, Option<Rgb<u8>>) {
        let d = self.data.read().unwrap();
        (d.value, d.state, d.quad, d.color)
    }

    pub fn get_vsc(&self) -> (u32, DomainElementState, Option<Rgb<u8>>) {
        let d = self.data.read().unwrap();
        (d.value, d.state, d.color)
    }

    pub fn get_vs(&self) -> (u32, DomainElementState) {
        let d = self.data.read().unwrap();
        (d.value, d.state)
    }

    pub fn get_sri(&self) -> (DomainElementState, f64, f64) {
        let d = self.data.read().unwrap();
        (d.state, d.origin_re, d.origin_im)
    }

    pub fn get_ri(&self) -> (f64, f64) {
        let d = self.data.read().unwrap();
        (d.origin_re, d.origin_im)
    }

    pub fn get_v(&self) -> u32 {
        self.data.read().unwrap().value
    }

    pub fn get_s(&self) -> DomainElementState {
        self.data.read().unwrap().state
    }

    pub fn get_c(&self) -> Option<Rgb<u8>> {
        self.data.read().unwrap().color
    }

    pub fn is_alive(&self) -> bool {
        *self.is_alive.read().expect("e3")
    }

    pub fn is_active_new(&self) -> bool {
        self.data.read().unwrap().state == ActiveNew
    }

    pub fn is_finished_too_short(&self) -> bool {
        self.data.read().unwrap().state == FinishedTooShort
    }

    pub fn is_finished_too_long(&self) -> bool {
        self.data.read().unwrap().state == FinishedTooLong
    }

    pub fn is_hibernated(&self) -> bool {
        let s = self.data.read().unwrap().state;
        s == FinishedTooShort || s == HibernatedDeepBlack
    }

    pub fn is_finished_success_any(&self) -> bool {
        let s = self.data.read().unwrap().state;
        s == FinishedSuccessPast || s == FinishedSuccess
    }

    pub fn is_finished_success_past(&self) -> bool {
        self.data.read().unwrap().state == FinishedSuccessPast
    }

    pub fn past(&self) {
        if self.data.read().unwrap().state == FinishedSuccess {
            self.data.write().unwrap().state = FinishedSuccessPast;
        }
    }

    pub fn has_worse_state_then(&self, other: &DataPx) -> bool {
        self.data
            .read()
            .unwrap()
            .state
            .cmp(&other.data.read().unwrap().state)
            .is_gt()
    }

    pub fn set_finished_state(&mut self, state: DomainElementState) {
        self.data.write().unwrap().state = state;
    }

    pub fn reset(&self, origin_re: f64, origin_im: f64, state: DomainElementState) {
        // is alive
        *self.is_alive.write().expect("e4") = true;
        // data
        let mut d = self.data.write().unwrap();
        d.origin_re = origin_re;
        d.origin_im = origin_im;
        d.value = 0;
        d.state = state;
        d.quad = 0.0;
        d.color = None;
    }

    pub fn override_by(&self, master: &DataPx) {
        // data
        let m = master.data.read().unwrap();
        let mut d = self.data.write().unwrap();
        d.origin_re = m.origin_re;
        d.origin_im = m.origin_im;
        d.value = m.value;
        d.state = m.state;
        d.quad = m.quad;
        d.color = m.color;

        // is alive
        *self.is_alive.write().expect("e2") = true;
        *master.is_alive.write().expect("e1") = false;
    }

    pub fn kill(&self) {
        *self.is_alive.write().expect("e5") = false;
    }
}

pub fn init(origin_re: f64, origin_im: f64, state: DomainElementState) -> DataPx {
    DataPx {
        is_alive: RwLock::new(true),
        data: RwLock::new(Data {
            origin_re,
            origin_im,
            value: 0,
            state,
            quad: 0.0,
            color: None,
        }),
    }
}

pub fn init_trivial() -> DataPx {
    init(0.0, 0.0, ActiveNew)
}

#[cfg(test)]
mod tests {
    use crate::data_px::init_trivial;
    use crate::pixel_states::DomainElementState::{
        ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort,
    };
    use image::Rgb;
    const GOLD: Rgb<u8> = Rgb([255, 215, 0]);

    #[test]
    fn test_add_v1() {
        let p = init_trivial();

        p.add_v1();
        assert_eq!(p.data.read().unwrap().value, 1);
    }

    #[test]
    fn test_set_v() {
        let p = init_trivial();

        p.set_v(7);
        assert_eq!(p.data.read().unwrap().value, 7);
    }

    #[test]
    fn test_set_qsv() {
        let p = init_trivial();

        p.set_qsv(2.2, FinishedSuccessPast, 8);
        let d = p.data.read().unwrap();
        assert_eq!(d.quad, 2.2);
        assert_eq!(d.state, FinishedSuccessPast);
        assert_eq!(d.value, 8);
    }

    #[test]
    fn test_set_qs() {
        let p = init_trivial();

        p.set_qs(3.1, FinishedTooShort);
        let d = p.data.read().unwrap();
        assert_eq!(d.quad, 3.1);
        assert_eq!(d.state, FinishedTooShort);
    }

    #[test]
    fn test_set_c() {
        let p = init_trivial();

        p.set_c(GOLD);
        assert_eq!(p.data.read().unwrap().color, Some(GOLD))
    }

    #[test]
    fn test_get_vsqc() {
        let p = init_trivial();

        let (rv, rs, rq, rc) = p.get_vsqc();
        assert_eq!(rv, 0);
        assert_eq!(rs, ActiveNew);
        assert_eq!(rq, 0.);
        assert_eq!(rc, None);
    }

    #[test]
    fn test_get_vsc() {
        let p = init_trivial();

        let (rv, rs, rc) = p.get_vsc();
        assert_eq!(rv, 0);
        assert_eq!(rs, ActiveNew);
        assert_eq!(rc, None);
    }

    #[test]
    fn test_get_vs() {
        let p = init_trivial();

        let (rv, rs) = p.get_vs();
        assert_eq!(rv, 0);
        assert_eq!(rs, ActiveNew);
    }

    #[test]
    fn test_get_sri() {
        let p = init_trivial();

        let (rs, rr, ri) = p.get_sri();
        assert_eq!(rs, ActiveNew);
        assert_eq!(rr, 0.0);
        assert_eq!(ri, 0.0);
    }

    #[test]
    fn test_get_ri() {
        let p = init_trivial();

        let (rr, ri) = p.get_ri();
        assert_eq!(rr, 0.0);
        assert_eq!(ri, 0.0);
    }

    #[test]
    fn test_get_v() {
        let p = init_trivial();
        assert_eq!(p.get_v(), 0);
    }

    #[test]
    fn test_get_s() {
        let p = init_trivial();
        assert_eq!(p.get_s(), ActiveNew);
    }

    #[test]
    fn test_is_alive() {
        let p = init_trivial();
        assert!(p.is_alive());
    }

    #[test]
    fn test_is_active_new() {
        let p = init_trivial();
        assert!(p.is_active_new());
    }

    #[test]
    fn test_is_finished_too_short() {
        let p = init_trivial();
        assert_eq!(p.is_finished_too_short(), false);
    }

    #[test]
    fn test_is_finished_too_long() {
        let p = init_trivial();
        assert_eq!(p.is_finished_too_long(), false);
    }

    #[test]
    fn test_is_hibernated() {
        let p = init_trivial();
        assert_eq!(p.is_hibernated(), false);
    }

    #[test]
    fn test_is_finished_success_any() {
        let p = init_trivial();
        assert_eq!(p.is_finished_success_any(), false);
    }

    #[test]
    fn test_is_finished_success_past() {
        let p = init_trivial();
        assert_eq!(p.is_finished_success_past(), false);
    }

    #[test]
    fn test_past() {
        let p = init_trivial();
        p.data.write().unwrap().state = FinishedSuccess;

        p.past();
        assert_eq!(p.data.read().unwrap().state, FinishedSuccessPast);
    }

    #[test]
    fn test_has_worse_state_then() {
        let p = init_trivial();
        let q = init_trivial();

        assert_eq!(p.has_worse_state_then(&q), false);
    }

    #[test]
    fn test_set_finished_state() {
        let mut p = init_trivial();

        p.set_finished_state(FinishedTooLong);
        assert_eq!(p.data.read().unwrap().state, FinishedTooLong);
    }

    #[test]
    fn test_reset() {
        let p = init_trivial();

        p.reset(1.1, 2.2, FinishedSuccessPast);
        let d = p.data.read().unwrap();
        assert_eq!(d.origin_re, 1.1);
        assert_eq!(d.origin_im, 2.2);
        assert_eq!(d.state, FinishedSuccessPast);
    }

    #[test]
    fn test_override_by() {
        let p = init_trivial();
        let q = init_trivial();
        p.override_by(&q);
        assert_eq!(p.is_alive(), true);
        assert_eq!(q.is_alive(), false);
    }

    #[test]
    fn test_kill() {
        let p = init_trivial();
        p.kill();
        assert_eq!(p.is_alive(), false);
    }
}
