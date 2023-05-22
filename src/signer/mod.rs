use std::sync::Arc;

use crate::{server::AppState, *};
use cloudfront_sign::*;
// const CLOUDFRONT_DOMAIN: &'static str = "https://ieco-vod.sadhguru.org";
pub const CLOUDFRONT_KEY_PAIR_ID: &'static str = "K3IS5FDKXVZ2O1";
pub const CLOUDFRONT_PRIVATE_KEY: &'static str = "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA0qVsUVZ5fJeE8aAMW2tqIJPw4Kfx/HiTV0drEK8SfdFB+g1Q
yzle7g9AdDp068/sSp7xIPh47ud9bl22/Pwt6vc5+Gb/oapbIejLjeBbDwoin7Mc
zkrGPiY1dxGmUjPo77UyHctSe4HsDYaJDKdyibUOdif6WXQkJCxDHTX3i/Xo607+
yNKcfx3TJ00DOKmW0F7gtpM/TK3xvKm7FAcFWPsJ0At0gcEs5myLTcArGNhEuKzk
KG3nTI/gGQoHO4XxkHiDeQo5VhhRYoNamW/u1J8UZVyq9cYlI/bGt6+a0ApGowqR
28YLEhnkbc+gf9QmBqEvuIFDdH3slYqC+XfBMwIDAQABAoIBAEnwPuEeFW6iUMvv
febpAUuGIlr5GHuiZzWPoAkvKrJ3LU10NxPeg2/Ucq9ZhYjwZuSafdEw5mIVZ3XB
92U9r3B2bya+IKTCg1KXwDuAsPXKfPTJjibrIELYbpPo4hSgm/boBztmDwHDTPRC
cQC8Ffw4TWc0CTA1ECqSTsyGEOpTN1tnbctKY+5kSsZWWN2HtZKJDOvYLoxSHx8w
Eg9C5C8poxg8UeMhApec/Gc/cC+4cfs0+1pn7U1JQ+5h3KOc5eH8t22+QeUWA9ni
NLo+wdh6weTCHUW1ZM/Mc7E88XiVX/oM33UAeuQXu3CdJts5HqrADP3OpEuA3ubh
MzFsocECgYEA/P8FGf22xuLY4SPvArMK6kPXn+kcyHGu+AeLOeIL79o7S07+eKW2
86jN5jo5TwpY1MaqMwI0+PNKXARUDWKl1q2SHmD/LsGnGlKh/wqlC4p6G1NgOLQi
bbVbnPllVvIl8CKagz9zInvi+rZODuj5YRX7bNyt1RkIl8OiiNdNvfsCgYEA1SWu
QmurLZscBiYTXGZ0IO2VjFU32dlXblHRQk3szRetx6c0qqwQ7RIALMCZVtXp2e/i
qG5unkEpMQWOXpDhEVQGkBErUWivnQAOEK4i2LQriNsZq16tb+KBqYYyBPOVRll6
gEdNhgbC3sM06+ds19wkDN1jUkohIOtd9euAvCkCgYBnvUZhPvjImtIll291Iudx
4fmzt4m2427Eg4tiY79TTaAMpEqSPBlF1kz/f5ZqQ6rjLMczfU8tWc1+58NzbaTk
321QCaP5jyyvDX0hsEK29cRWGh0Vgz+uzrPPLTCRs189FWNYOBdE55nOBh0B3iQL
3FCpdd/NxqMQYAbwp6BYKwKBgB+Op6Ev3bUv3NNQOdeZV6I0asrPJyi61AYCWRAP
B3tKATGpINQQC6V938bAcTN5Ellw+cbkKr6FeTrHzs9Tde8h2KLl19sCu9Vr16lC
jug0rKljfwZvSxBgob/enXM8OSKkRs3NQL4SLDHdBu3SYrwG3NOGx5i8XvFHx66M
QpAZAoGBAPKC5+I6ynPzynMsFknItHjLSyFRm1tr48oPxujLCmSTvi70zExkyjLl
GP1zuDFC25kmXwchIJqMS2rUtMGb5Azoc+aUVk7dmdxFPKzAjBBZ14USOqJdhpB9
HwUUl4zVVqbPyft3HwzaQqlLp6etMwpEEIYoXWfcD2YWg0SvSx07
-----END RSA PRIVATE KEY-----";
pub fn sign(url: &String, app_state: &Arc<AppState>) -> String {
    get_signed_url(&url, &app_state.sign_options).unwrap()
}
