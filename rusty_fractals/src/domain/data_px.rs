use crate::image::pixel_states::DomainElementState;
use crate::image::pixel_states::DomainElementState::{
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
struct Data {
    origin_re: f64,
    origin_im: f64,
    value: u64,
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
        if let Ok(mut d) = self.data.write() {
            d.value += 1;
        }
    }

    pub fn set_v(&self, value: u64) {
        if let Ok(mut d) = self.data.write() {
            d.value = value;
        }
    }

    pub fn set_qsv(&self, quad: f64, state: DomainElementState, value: u64) {
        if let Ok(mut d) = self.data.write() {
            d.quad = quad;
            d.state = state;
            d.value = value;
        }
    }

    pub fn set_qs(&self, quad: f64, state: DomainElementState) {
        if let Ok(mut d) = self.data.write() {
            d.quad = quad;
            d.state = state;
        }
    }

    pub fn set_c(&self, color: Rgb<u8>) {
        if let Ok(mut d) = self.data.write() {
            d.color = Some(color);
        }
    }

    pub fn get_vsqc(&self) -> (u64, DomainElementState, f64, Option<Rgb<u8>>) {
        self.data.read().map_or((0, ActiveNew, 0.0, None), |d| (d.value, d.state, d.quad, d.color))
    }

    pub fn get_vsc(&self) -> (u64, DomainElementState, Option<Rgb<u8>>) {
        self.data.read().map_or((0, ActiveNew, None), |d| (d.value, d.state, d.color))
    }

    pub fn get_vs(&self) -> (u64, DomainElementState) {

        // TODO thes must be always set, throw instead
        self.data.read().map_or((0, ActiveNew), |d| (d.value, d.state))
    }

    pub fn get_sri(&self) -> (DomainElementState, f64, f64) {
        self.data.read().map_or((ActiveNew, 0.0, 0.0), |d| (d.state, d.origin_re, d.origin_im))
    }

    pub fn get_ri(&self) -> (f64, f64) {
        self.data.read().map_or((0.0, 0.0), |d| (d.origin_re, d.origin_im))
    }

    pub fn get_v(&self) -> u64 {
        self.data.read().map(|d| d.value).unwrap_or(0)
    }

    pub fn get_s(&self) -> DomainElementState {
        self.data.read().map(|d| d.state).unwrap_or(ActiveNew)
    }

    pub fn get_c(&self) -> Option<Rgb<u8>> {
        self.data.read().map(|d| d.color).unwrap_or(None)
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive.read().map(|a| *a).unwrap_or(false)
    }

    pub fn is_active_new(&self) -> bool {
        self.data.read().map(|d| d.state == ActiveNew).unwrap_or(false)
    }

    pub fn is_finished_too_short(&self) -> bool {
        self.data.read().map(|d| d.state == FinishedTooShort).unwrap_or(false)
    }

    pub fn is_finished_too_long(&self) -> bool {
        self.data.read().map(|d| d.state == FinishedTooLong).unwrap_or(false)
    }

    pub fn is_hibernated(&self) -> bool {
        self.data.read().is_ok_and(|d| d.state == FinishedTooShort || d.state == HibernatedDeepBlack)
    }

    pub fn is_finished_success_any(&self) -> bool {
        self.data.read().is_ok_and(|d| d.state == FinishedSuccessPast || d.state == FinishedSuccess)
    }

    pub fn is_finished_success_past(&self) -> bool {
        self.data.read().map(|d| d.state == FinishedSuccessPast).unwrap_or(false)
    }

    pub fn past(&self) {
        if let Ok(d) = self.data.read()
            && d.state == FinishedSuccess {
            drop(d);
            if let Ok(mut d_write) = self.data.write() {
                d_write.state = FinishedSuccessPast;
            }
        }
    }

    pub fn has_worse_state_then(&self, other: &Self) -> bool {
        if let (Ok(d1), Ok(d2)) = (self.data.read(), other.data.read()) {
            d1.state.cmp(&d2.state).is_gt()
        } else {
            false
        }
    }

    pub fn set_finished_state(&mut self, state: DomainElementState) {
        if let Ok(mut d) = self.data.write() {
            d.state = state;
        }
    }

    pub fn reset(&self, origin_re: f64, origin_im: f64, state: DomainElementState) {
        // is alive
        if let Ok(mut alive) = self.is_alive.write() {
            *alive = true;
        }
        // data
        if let Ok(mut d) = self.data.write() {
            d.origin_re = origin_re;
            d.origin_im = origin_im;
            d.value = 0;
            d.state = state;
            d.quad = 0.0;
            d.color = None;
        }
    }

    pub fn override_by(&self, master: &Self) {
        // data
        if let (Ok(m), Ok(mut s)) = (master.data.read(), self.data.write()) {
            s.origin_re = m.origin_re;
            s.origin_im = m.origin_im;
            s.value = m.value;
            s.state = m.state;
            s.quad = m.quad;
            s.color = m.color;
            drop(m);
            drop(s);

            // is alive
            if let Ok(mut alive) = self.is_alive.write() {
                *alive = true;
            }
            if let Ok(mut m_alive) = master.is_alive.write() {
                *m_alive = false;
            }
        }
    }

    pub fn kill(&self) {
        if let Ok(mut alive) = self.is_alive.write() {
            *alive = false;
        }
    }
}

pub const fn init(origin_re: f64, origin_im: f64, state: DomainElementState) -> DataPx {
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

pub const fn init_trivial() -> DataPx {
    init(0.0, 0.0, ActiveNew)
}

#[cfg(test)]
mod tests {
    use crate::domain::data_px::init_trivial;
    use crate::image::pixel_states::DomainElementState::{
        ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort,
    };
    use image::Rgb;
    const GOLD: Rgb<u8> = Rgb([255, 215, 0]);

    #[test]
    fn test_add_v1() {
        let p = init_trivial();

        p.add_v1();
        assert_eq!(p.get_v(), 1);
    }

    #[test]
    fn test_set_v() {
        let p = init_trivial();

        p.set_v(7);
        assert_eq!(p.get_v(), 7);
    }

    #[test]
    fn test_set_qsv() {
        let p = init_trivial();

        p.set_qsv(2.2, FinishedSuccessPast, 8);
        let (_, rs, rq, _) = p.get_vsqc();
        assert_eq!(rq, 2.2);
        assert_eq!(rs, FinishedSuccessPast);
        assert_eq!(p.get_v(), 8);
    }

    #[test]
    fn test_set_qs() {
        let p = init_trivial();

        p.set_qs(3.1, FinishedTooShort);
        let (_, rs, rq, _) = p.get_vsqc();
        assert_eq!(rq, 3.1);
        assert_eq!(rs, FinishedTooShort);
    }

    #[test]
    fn test_set_c() {
        let p = init_trivial();

        p.set_c(GOLD);
        assert_eq!(p.get_c(), Some(GOLD))
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
        assert!(!p.is_finished_too_short());
    }

    #[test]
    fn test_is_finished_too_long() {
        let p = init_trivial();
        assert!(!p.is_finished_too_long());
    }

    #[test]
    fn test_is_hibernated() {
        let p = init_trivial();
        assert!(!p.is_hibernated());
    }

    #[test]
    fn test_is_finished_success_any() {
        let p = init_trivial();
        assert!(!p.is_finished_success_any());
    }

    #[test]
    fn test_is_finished_success_past() {
        let p = init_trivial();
        assert!(!p.is_finished_success_past());
    }

    #[test]
    fn test_past() {
        let p = init_trivial();
        if let Ok(mut d) = p.data.write() {
            d.state = FinishedSuccess;
        }

        p.past();
        assert_eq!(p.get_s(), FinishedSuccessPast);
    }

    #[test]
    fn test_has_worse_state_then() {
        let p = init_trivial();
        let q = init_trivial();

        assert!(!p.has_worse_state_then(&q));
    }

    #[test]
    fn test_set_finished_state() {
        let mut p = init_trivial();

        p.set_finished_state(FinishedTooLong);
        assert_eq!(p.get_s(), FinishedTooLong);
    }

    #[test]
    fn test_reset() {
        let p = init_trivial();

        p.reset(1.1, 2.2, FinishedSuccessPast);
        let (rs, rr, ri) = p.get_sri();
        assert_eq!(rr, 1.1);
        assert_eq!(ri, 2.2);
        assert_eq!(rs, FinishedSuccessPast);
    }

    #[test]
    fn test_override_by() {
        let p = init_trivial();
        let q = init_trivial();
        p.override_by(&q);
        assert!(p.is_alive());
        assert!(!q.is_alive());
    }

    #[test]
    fn test_kill() {
        let p = init_trivial();
        p.kill();
        assert!(!p.is_alive());
    }
}
