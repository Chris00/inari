/*
 *
 * Unit tests from libieeep1788 for recommended interval boolean operations
 * (Original author: Marco Nehmeier)
 * converted into portable ITL format by Oliver Heimlich.
 *
 * Copyright 2013-2015 Marco Nehmeier (nehmeier@informatik.uni-wuerzburg.de)
 * Copyright 2015-2017 Oliver Heimlich (oheim@posteo.de)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
//Language imports
#![rustfmt::skip]
#![allow(unused_attributes, unused_imports)]

//Test library imports

//Arithmetic library imports

//Preamble
use crate::util::*;
use hexf::*;
type D = inari::Decoration;
type DI = inari::DecoratedInterval;
type I = inari::Interval;

#[test]
fn minimal_is_common_interval_test() {
    assert!(n2i(-27.0, -27.0).is_common_interval());
    assert!(n2i(-27.0, 0.0).is_common_interval());
    assert!(n2i(0.0, 0.0).is_common_interval());
    assert!(n2i(-0.0, -0.0).is_common_interval());
    assert!(n2i(-0.0, 0.0).is_common_interval());
    assert!(n2i(0.0, -0.0).is_common_interval());
    assert!(n2i(5.0, 12.4).is_common_interval());
    assert!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).is_common_interval());
    assert_eq!(I::entire().is_common_interval(), false);
    assert_eq!(I::empty().is_common_interval(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).is_common_interval(), false);
    assert_eq!(n2i(0.0, f64::INFINITY).is_common_interval(), false);
}

#[test]
fn minimal_is_common_interval_dec_test() {
    assert!(nd2di(-27.0, -27.0, D::Com).is_common_interval());
    assert!(nd2di(-27.0, 0.0, D::Com).is_common_interval());
    assert!(nd2di(0.0, 0.0, D::Com).is_common_interval());
    assert!(nd2di(-0.0, -0.0, D::Com).is_common_interval());
    assert!(nd2di(-0.0, 0.0, D::Com).is_common_interval());
    assert!(nd2di(0.0, -0.0, D::Com).is_common_interval());
    assert!(nd2di(5.0, 12.4, D::Com).is_common_interval());
    assert!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Com).is_common_interval());
    assert!(nd2di(-27.0, -27.0, D::Trv).is_common_interval());
    assert!(nd2di(-27.0, 0.0, D::Def).is_common_interval());
    assert!(nd2di(0.0, 0.0, D::Dac).is_common_interval());
    assert!(nd2di(-0.0, -0.0, D::Trv).is_common_interval());
    assert!(nd2di(-0.0, 0.0, D::Def).is_common_interval());
    assert!(nd2di(0.0, -0.0, D::Dac).is_common_interval());
    assert!(nd2di(5.0, 12.4, D::Def).is_common_interval());
    assert!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Trv).is_common_interval());
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).is_common_interval(), false);
    assert_eq!(DI::nai().is_common_interval(), false);
    assert_eq!(DI::empty().is_common_interval(), false);
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Trv).is_common_interval(), false);
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Def).is_common_interval(), false);
}

#[test]
fn minimal_is_singleton_test() {
    assert!(n2i(-27.0, -27.0).is_singleton());
    assert!(n2i(-2.0, -2.0).is_singleton());
    assert!(n2i(12.0, 12.0).is_singleton());
    assert!(n2i(17.1, 17.1).is_singleton());
    assert!(n2i(-0.0, -0.0).is_singleton());
    assert!(n2i(0.0, 0.0).is_singleton());
    assert!(n2i(-0.0, 0.0).is_singleton());
    assert!(n2i(0.0, -0.0).is_singleton());
    assert_eq!(I::empty().is_singleton(), false);
    assert_eq!(I::entire().is_singleton(), false);
    assert_eq!(n2i(-1.0, 0.0).is_singleton(), false);
    assert_eq!(n2i(-1.0, -0.5).is_singleton(), false);
    assert_eq!(n2i(1.0, 2.0).is_singleton(), false);
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023")).is_singleton(), false);
    assert_eq!(n2i(-1.0, f64::INFINITY).is_singleton(), false);
}

#[test]
fn minimal_is_singleton_dec_test() {
    assert!(nd2di(-27.0, -27.0, D::Def).is_singleton());
    assert!(nd2di(-2.0, -2.0, D::Trv).is_singleton());
    assert!(nd2di(12.0, 12.0, D::Dac).is_singleton());
    assert!(nd2di(17.1, 17.1, D::Com).is_singleton());
    assert!(nd2di(-0.0, -0.0, D::Def).is_singleton());
    assert!(nd2di(0.0, 0.0, D::Com).is_singleton());
    assert!(nd2di(-0.0, 0.0, D::Def).is_singleton());
    assert!(nd2di(0.0, -0.0, D::Dac).is_singleton());
    assert_eq!(DI::empty().is_singleton(), false);
    assert_eq!(DI::nai().is_singleton(), false);
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).is_singleton(), false);
    assert_eq!(nd2di(-1.0, 0.0, D::Dac).is_singleton(), false);
    assert_eq!(nd2di(-1.0, -0.5, D::Com).is_singleton(), false);
    assert_eq!(nd2di(1.0, 2.0, D::Def).is_singleton(), false);
    assert_eq!(nd2di(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023"), D::Dac).is_singleton(), false);
    assert_eq!(nd2di(-1.0, f64::INFINITY, D::Trv).is_singleton(), false);
}

#[test]
fn minimal_is_member_test() {
    assert!(n2i(-27.0, -27.0).contains(-27.0));
    assert!(n2i(-27.0, 0.0).contains(-27.0));
    assert!(n2i(-27.0, 0.0).contains(-7.0));
    assert!(n2i(-27.0, 0.0).contains(0.0));
    assert!(n2i(0.0, 0.0).contains(-0.0));
    assert!(n2i(0.0, 0.0).contains(0.0));
    assert!(n2i(-0.0, -0.0).contains(0.0));
    assert!(n2i(-0.0, 0.0).contains(0.0));
    assert!(n2i(0.0, -0.0).contains(0.0));
    assert!(n2i(5.0, 12.4).contains(5.0));
    assert!(n2i(5.0, 12.4).contains(6.3));
    assert!(n2i(5.0, 12.4).contains(12.4));
    assert!(I::entire().contains(0.0));
    assert!(I::entire().contains(5.0));
    assert!(I::entire().contains(6.3));
    assert!(I::entire().contains(12.4));
    assert!(I::entire().contains(hexf64!("0x1.fffffffffffffp+1023")));
    assert!(I::entire().contains(hexf64!("-0x1.fffffffffffffp+1023")));
    assert!(I::entire().contains(hexf64!("0x1.0000000000000p-1022")));
    assert!(I::entire().contains(hexf64!("-0x1.0000000000000p-1022")));
    assert_eq!(n2i(-27.0, 0.0).contains(-71.0), false);
    assert_eq!(n2i(-27.0, 0.0).contains(0.1), false);
    assert_eq!(n2i(0.0, 0.0).contains(-0.01), false);
    assert_eq!(n2i(0.0, 0.0).contains(1e-06), false);
    assert_eq!(n2i(-0.0, -0.0).contains(111110.0), false);
    assert_eq!(n2i(5.0, 12.4).contains(4.9), false);
    assert_eq!(n2i(5.0, 12.4).contains(-6.3), false);
    assert_eq!(I::empty().contains(0.0), false);
    assert_eq!(I::empty().contains(-4535.3), false);
    assert_eq!(I::empty().contains(f64::NEG_INFINITY), false);
    assert_eq!(I::empty().contains(f64::INFINITY), false);
    assert_eq!(I::empty().contains(f64::NAN), false);
    assert_eq!(I::entire().contains(f64::NEG_INFINITY), false);
    assert_eq!(I::entire().contains(f64::INFINITY), false);
    assert_eq!(I::entire().contains(f64::NAN), false);
}

#[test]
fn minimal_is_member_dec_test() {
    assert!(nd2di(-27.0, -27.0, D::Trv).contains(-27.0));
    assert!(nd2di(-27.0, 0.0, D::Def).contains(-27.0));
    assert!(nd2di(-27.0, 0.0, D::Dac).contains(-7.0));
    assert!(nd2di(-27.0, 0.0, D::Com).contains(0.0));
    assert!(nd2di(0.0, 0.0, D::Trv).contains(-0.0));
    assert!(nd2di(0.0, 0.0, D::Def).contains(0.0));
    assert!(nd2di(-0.0, -0.0, D::Dac).contains(0.0));
    assert!(nd2di(-0.0, 0.0, D::Com).contains(0.0));
    assert!(nd2di(0.0, -0.0, D::Trv).contains(0.0));
    assert!(nd2di(5.0, 12.4, D::Def).contains(5.0));
    assert!(nd2di(5.0, 12.4, D::Dac).contains(6.3));
    assert!(nd2di(5.0, 12.4, D::Com).contains(12.4));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).contains(0.0));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).contains(5.0));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).contains(6.3));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).contains(12.4));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).contains(hexf64!("0x1.fffffffffffffp+1023")));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).contains(hexf64!("-0x1.fffffffffffffp+1023")));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).contains(hexf64!("0x1.0000000000000p-1022")));
    assert!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).contains(hexf64!("-0x1.0000000000000p-1022")));
    assert_eq!(nd2di(-27.0, 0.0, D::Trv).contains(-71.0), false);
    assert_eq!(nd2di(-27.0, 0.0, D::Def).contains(0.1), false);
    assert_eq!(nd2di(0.0, 0.0, D::Dac).contains(-0.01), false);
    assert_eq!(nd2di(0.0, 0.0, D::Com).contains(1e-06), false);
    assert_eq!(nd2di(-0.0, -0.0, D::Trv).contains(111110.0), false);
    assert_eq!(nd2di(5.0, 12.4, D::Def).contains(4.9), false);
    assert_eq!(nd2di(5.0, 12.4, D::Dac).contains(-6.3), false);
    assert_eq!(DI::empty().contains(0.0), false);
    assert_eq!(DI::empty().contains(0.0), false);
    assert_eq!(DI::empty().contains(-4535.3), false);
    assert_eq!(DI::empty().contains(-4535.3), false);
    assert_eq!(DI::empty().contains(f64::NEG_INFINITY), false);
    assert_eq!(DI::nai().contains(f64::NEG_INFINITY), false);
    assert_eq!(DI::empty().contains(f64::INFINITY), false);
    assert_eq!(DI::nai().contains(f64::INFINITY), false);
    assert_eq!(DI::empty().contains(f64::NAN), false);
    assert_eq!(DI::nai().contains(f64::NAN), false);
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv).contains(f64::NEG_INFINITY), false);
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).contains(f64::INFINITY), false);
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).contains(f64::NAN), false);
}