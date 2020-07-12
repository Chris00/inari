/*
 *
 * Unit tests from libieeep1788 for interval constructors
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
fn minimal_nums_to_interval_test() {
    assert_eq!(n2i(-1.0, 1.0), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(-1.0, f64::INFINITY), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, f64::INFINITY), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(n2i(f64::NAN, f64::NAN), I::empty());
    assert_eq!(n2i(1.0, -1.0), I::empty());
    assert_eq!(n2i(f64::NEG_INFINITY, f64::NEG_INFINITY), I::empty());
    assert_eq!(n2i(f64::INFINITY, f64::INFINITY), I::empty());
}

#[test]
fn minimal_nums_to_decorated_interval_test() {
    assert_eq!(n2di(-1.0, 1.0), nd2di(-1.0, 1.0, D::Com));
    assert_eq!(n2di(f64::NEG_INFINITY, 1.0), nd2di(f64::NEG_INFINITY, 1.0, D::Dac));
    assert_eq!(n2di(-1.0, f64::INFINITY), nd2di(-1.0, f64::INFINITY, D::Dac));
    assert_eq!(n2di(f64::NEG_INFINITY, f64::INFINITY), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert!(n2di(f64::NAN, f64::NAN).is_nai());
    assert!(n2di(1.0, -1.0).is_nai());
    assert!(n2di(f64::NEG_INFINITY, f64::NEG_INFINITY).is_nai());
    assert!(n2di(f64::INFINITY, f64::INFINITY).is_nai());
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_text_to_interval_test() {
    assert_eq!(t2i("[ Empty  ]"), I::empty());
    assert_eq!(t2i("[ Empty  ]_trv"), I::empty());
    assert_eq!(t2i("[  ]"), I::empty());
    assert_eq!(t2i("[  ]_trv"), I::empty());
    assert_eq!(t2i("[,]"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[,]_trv"), I::empty());
    assert_eq!(t2i("[ entire  ]"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[ ENTIRE ]_dac"), I::empty());
    assert_eq!(t2i("[ ENTIRE ]"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[ -inf , INF  ]"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[ -inf, INF ]_def"), I::empty());
    assert_eq!(t2i("[-1.0,1.0]"), n2i(-1.0, 1.0));
    assert_eq!(t2i("[  -1.0  ,  1.0  ]"), n2i(-1.0, 1.0));
    assert_eq!(t2i("[  -1.0  , 1.0]"), n2i(-1.0, 1.0));
    assert_eq!(t2i("[-1,]"), n2i(-1.0, f64::INFINITY));
    assert_eq!(t2i("[-1.0, +inf]"), n2i(-1.0, f64::INFINITY));
    assert_eq!(t2i("[-1.0, +infinity]"), n2i(-1.0, f64::INFINITY));
    assert_eq!(t2i("[-Inf, 1.000 ]"), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(t2i("[-Infinity, 1.000 ]"), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(t2i("[1.0E+400 ]"), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(t2i("[ -4/2, 10/5 ]"), n2i(-2.0, 2.0));
    assert_eq!(t2i("[ -1/10, 1/10 ]"), n2i(-0.1, 0.1));
    assert_eq!(t2i("0.0?"), n2i(-0.05, 0.05));
    assert_eq!(t2i("0.0?u"), n2i(0.0, 0.05));
    assert_eq!(t2i("0.0?d"), n2i(-0.05, 0.0));
    assert_eq!(t2i("2.5?"), n2i(hexf64!("0x1.3999999999999p+1"), hexf64!("0x1.4666666666667p+1")));
    assert_eq!(t2i("2.5?u"), n2i(2.5, hexf64!("0x1.4666666666667p+1")));
    assert_eq!(t2i("2.5?d"), n2i(hexf64!("0x1.3999999999999p+1"), 2.5));
    assert_eq!(t2i("0.000?5"), n2i(-0.005, 0.005));
    assert_eq!(t2i("0.000?5u"), n2i(0.0, 0.005));
    assert_eq!(t2i("0.000?5d"), n2i(-0.005, 0.0));
    assert_eq!(t2i("2.500?5"), n2i(hexf64!("0x1.3f5c28f5c28f5p+1"), hexf64!("0x1.40a3d70a3d70bp+1")));
    assert_eq!(t2i("2.500?5u"), n2i(2.5, hexf64!("0x1.40a3d70a3d70bp+1")));
    assert_eq!(t2i("2.500?5d"), n2i(hexf64!("0x1.3f5c28f5c28f5p+1"), 2.5));
    assert_eq!(t2i("0.0??"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("0.0??u"), n2i(0.0, f64::INFINITY));
    assert_eq!(t2i("0.0??d"), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(t2i("2.5??"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("2.5??u"), n2i(2.5, f64::INFINITY));
    assert_eq!(t2i("2.5??d"), n2i(f64::NEG_INFINITY, 2.5));
    assert_eq!(t2i("2.500?5e+27"), n2i(hexf64!("0x1.01fa19a08fe7fp+91"), hexf64!("0x1.0302cc4352683p+91")));
    assert_eq!(t2i("2.500?5ue4"), n2i(hexf64!("0x1.86a0000000000p+14"), hexf64!("0x1.8768000000000p+14")));
    assert_eq!(t2i("2.500?5de-5"), n2i(hexf64!("0x1.a2976f1cee4d5p-16"), hexf64!("0x1.a36e2eb1c432dp-16")));
    assert_eq!(t2i("10?3"), n2i(7.0, 13.0));
    assert_eq!(t2i("10?3e380"), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(t2i("1.0000000000000001?1"), n2i(1.0, hexf64!("0x1.0000000000001p+0")));
    // 10?18 + 308 zeros
    assert_eq!(t2i("10?1800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(t2i("[ Nai  ]"), I::empty());
    assert_eq!(t2i("[ Nai  ]_ill"), I::empty());
    assert_eq!(t2i("[ Nai  ]_trv"), I::empty());
    assert_eq!(t2i("[ Empty  ]_ill"), I::empty());
    assert_eq!(t2i("[  ]_com"), I::empty());
    assert_eq!(t2i("[,]_com"), I::empty());
    assert_eq!(t2i("[   Entire ]_com"), I::empty());
    assert_eq!(t2i("[ -inf ,  INF ]_com"), I::empty());
    assert_eq!(t2i("[  -1.0  , 1.0]_ill"), I::empty());
    assert_eq!(t2i("[  -1.0  , 1.0]_fooo"), I::empty());
    assert_eq!(t2i("[  -1.0  , 1.0]_da"), I::empty());
    assert_eq!(t2i("[-1.0,]_com"), I::empty());
    assert_eq!(t2i("[-Inf, 1.000 ]_ill"), I::empty());
    assert_eq!(t2i("[-I  nf, 1.000 ]"), I::empty());
    assert_eq!(t2i("[-Inf, 1.0  00 ]"), I::empty());
    assert_eq!(t2i("[-Inf ]"), I::empty());
    assert_eq!(t2i("[Inf , INF]"), I::empty());
    assert_eq!(t2i("[ foo ]"), I::empty());
    assert_eq!(t2i("0.0??_com"), I::empty());
    assert_eq!(t2i("0.0??u_ill"), I::empty());
    assert_eq!(t2i("0.0??d_com"), I::empty());
    assert_eq!(t2i("[1.0000000000000002,1.0000000000000001]"), I::empty());
    assert_eq!(t2i("[10000000000000001/10000000000000000,10000000000000002/10000000000000001]"), I::empty());
    assert_eq!(t2i("[0x1.00000000000002p0,0x1.00000000000001p0]"), I::empty());
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_text_to_decorated_interval_test() {
    assert_eq!(t2di("[ Empty  ]"), DI::empty());
    assert_eq!(t2di("[ Empty  ]_trv"), DI::empty());
    assert_eq!(t2di("[  ]"), DI::empty());
    assert_eq!(t2di("[  ]_trv"), DI::empty());
    assert_eq!(t2di("[,]"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(t2di("[,]_trv"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(t2di("[ entire  ]"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(t2di("[ ENTIRE ]_dac"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(t2di("[ -inf , INF  ]"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(t2di("[ -inf, INF ]_def"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def));
    assert_eq!(t2di("[-1.0,1.0]"), nd2di(-1.0, 1.0, D::Com));
    assert_eq!(t2di("[  -1.0  ,  1.0  ]_com"), nd2di(-1.0, 1.0, D::Com));
    assert_eq!(t2di("[  -1.0  , 1.0]_trv"), nd2di(-1.0, 1.0, D::Trv));
    assert_eq!(t2di("[-1,]"), nd2di(-1.0, f64::INFINITY, D::Dac));
    assert_eq!(t2di("[-1.0, +inf]_def"), nd2di(-1.0, f64::INFINITY, D::Def));
    assert_eq!(t2di("[-1.0, +infinity]_def"), nd2di(-1.0, f64::INFINITY, D::Def));
    assert_eq!(t2di("[-Inf, 1.000 ]"), nd2di(f64::NEG_INFINITY, 1.0, D::Dac));
    assert_eq!(t2di("[-Infinity, 1.000 ]_trv"), nd2di(f64::NEG_INFINITY, 1.0, D::Trv));
    assert_eq!(t2di("[1.0E+400 ]_com"), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac));
    assert_eq!(t2di("[ -4/2, 10/5 ]_com"), nd2di(-2.0, 2.0, D::Com));
    assert_eq!(t2di("[ -1/10, 1/10 ]_com"), nd2di(-0.1, 0.1, D::Com));
    assert_eq!(t2di("0.0?"), nd2di(-0.05, 0.05, D::Com));
    assert_eq!(t2di("0.0?u_trv"), nd2di(0.0, 0.05, D::Trv));
    assert_eq!(t2di("0.0?d_dac"), nd2di(-0.05, 0.0, D::Dac));
    assert_eq!(t2di("2.5?"), nd2di(hexf64!("0x1.3999999999999p+1"), hexf64!("0x1.4666666666667p+1"), D::Com));
    assert_eq!(t2di("2.5?u"), nd2di(2.5, hexf64!("0x1.4666666666667p+1"), D::Com));
    assert_eq!(t2di("2.5?d_trv"), nd2di(hexf64!("0x1.3999999999999p+1"), 2.5, D::Trv));
    assert_eq!(t2di("0.000?5"), nd2di(-0.005, 0.005, D::Com));
    assert_eq!(t2di("0.000?5u_def"), nd2di(0.0, 0.005, D::Def));
    assert_eq!(t2di("0.000?5d"), nd2di(-0.005, 0.0, D::Com));
    assert_eq!(t2di("2.500?5"), nd2di(hexf64!("0x1.3f5c28f5c28f5p+1"), hexf64!("0x1.40a3d70a3d70bp+1"), D::Com));
    assert_eq!(t2di("2.500?5u"), nd2di(2.5, hexf64!("0x1.40a3d70a3d70bp+1"), D::Com));
    assert_eq!(t2di("2.500?5d"), nd2di(hexf64!("0x1.3f5c28f5c28f5p+1"), 2.5, D::Com));
    assert_eq!(t2di("0.0??_dac"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(t2di("0.0??u_trv"), nd2di(0.0, f64::INFINITY, D::Trv));
    assert_eq!(t2di("0.0??d"), nd2di(f64::NEG_INFINITY, 0.0, D::Dac));
    assert_eq!(t2di("2.5??"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(t2di("2.5??u_def"), nd2di(2.5, f64::INFINITY, D::Def));
    assert_eq!(t2di("2.5??d_dac"), nd2di(f64::NEG_INFINITY, 2.5, D::Dac));
    assert_eq!(t2di("2.500?5e+27"), nd2di(hexf64!("0x1.01fa19a08fe7fp+91"), hexf64!("0x1.0302cc4352683p+91"), D::Com));
    assert_eq!(t2di("2.500?5ue4_def"), nd2di(hexf64!("0x1.86a0000000000p+14"), hexf64!("0x1.8768000000000p+14"), D::Def));
    assert_eq!(t2di("2.500?5de-5"), nd2di(hexf64!("0x1.a2976f1cee4d5p-16"), hexf64!("0x1.a36e2eb1c432dp-16"), D::Com));
    assert!(t2di("[ Nai  ]").is_nai());
    // 10?18 + 308 zeros + _com
    assert_eq!(t2di("10?1800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000_com"), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(t2di("10?3_com"), nd2di(7.0, 13.0, D::Com));
    assert_eq!(t2di("10?3e380_com"), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac));
    assert!(t2di("[ Nai  ]_ill").is_nai());
    assert!(t2di("[ Nai  ]_trv").is_nai());
    assert!(t2di("[ Empty  ]_ill").is_nai());
    assert!(t2di("[  ]_com").is_nai());
    assert!(t2di("[,]_com").is_nai());
    assert!(t2di("[   Entire ]_com").is_nai());
    assert!(t2di("[ -inf ,  INF ]_com").is_nai());
    assert!(t2di("[  -1.0  , 1.0]_ill").is_nai());
    assert!(t2di("[  -1.0  , 1.0]_fooo").is_nai());
    assert!(t2di("[  -1.0  , 1.0]_da").is_nai());
    assert!(t2di("[-1.0,]_com").is_nai());
    assert!(t2di("[-Inf, 1.000 ]_ill").is_nai());
    assert!(t2di("[-I  nf, 1.000 ]").is_nai());
    assert!(t2di("[-Inf, 1.0  00 ]").is_nai());
    assert!(t2di("[-Inf ]").is_nai());
    assert!(t2di("[Inf , INF]").is_nai());
    assert!(t2di("[ foo ]").is_nai());
    assert!(t2di("0.0??_com").is_nai());
    assert!(t2di("0.0??u_ill").is_nai());
    assert!(t2di("0.0??d_com").is_nai());
    assert!(t2di("0.0??_com").is_nai());
    assert!(t2di("[1.0,2.0").is_nai());
    assert!(t2di("[1.0000000000000002,1.0000000000000001]").is_nai());
    assert!(t2di("[10000000000000001/10000000000000000,10000000000000002/10000000000000001]").is_nai());
    assert!(t2di("[0x1.00000000000002p0,0x1.00000000000001p0]").is_nai());
}

#[test]
fn minimal_interval_part_test() {
    assert_eq!(interval_part(nd2di(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.9999999999999p-4"), D::Trv)), n2i(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.9999999999999p-4")));
    assert_eq!(interval_part(nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4"), D::Com)), n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4")));
    assert_eq!(interval_part(nd2di(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.99999a0000000p-4"), D::Dac)), n2i(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.99999a0000000p-4")));
    assert_eq!(interval_part(nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.99999a0000000p-4"), D::Def)), n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.99999a0000000p-4")));
    assert_eq!(interval_part(nd2di(hexf64!("-0x0.0000000000001p-1022"), hexf64!("-0x0.0000000000001p-1022"), D::Trv)), n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("-0x0.0000000000001p-1022")));
    assert_eq!(interval_part(nd2di(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022"), D::Trv)), n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(interval_part(nd2di(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022"), D::Trv)), n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(interval_part(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023"), D::Trv)), n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")));
    assert_eq!(interval_part(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Trv)), n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(interval_part(nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Trv)), n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(interval_part(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv)), n2i(f64::NEG_INFINITY, f64::INFINITY));
    assert_eq!(interval_part(DI::empty()), I::empty());
    assert_eq!(interval_part(nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4"), D::Com)), n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4")));
    assert_eq!(interval_part(DI::nai()), I::empty());
}

#[test]
fn minimal_new_dec_test() {
    assert_eq!(DI::new(n2i(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.9999999999999p-4"))), nd2di(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.9999999999999p-4"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4"))), nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.99999a0000000p-4"))), nd2di(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.99999a0000000p-4"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.99999a0000000p-4"))), nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.99999a0000000p-4"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("-0x0.0000000000001p-1022"))), nd2di(hexf64!("-0x0.0000000000001p-1022"), hexf64!("-0x0.0000000000001p-1022"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022"))), nd2di(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022"))), nd2di(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023"))), nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"))), nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Com));
    assert_eq!(DI::new(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"))), nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Com));
    assert_eq!(DI::new(n2i(f64::NEG_INFINITY, f64::INFINITY)), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(DI::new(I::empty()), DI::empty());
    assert_eq!(DI::new(n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4"))), nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4"), D::Com));
}

#[test]
fn minimal_set_dec_test() {
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.9999999999999p-4")), D::Trv), nd2di(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.9999999999999p-4"), D::Trv));
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4")), D::Com), nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4"), D::Com));
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.99999a0000000p-4")), D::Dac), nd2di(hexf64!("-0x1.99999a842549ap+4"), hexf64!("0x1.99999a0000000p-4"), D::Dac));
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.99999a0000000p-4")), D::Def), nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.99999a0000000p-4"), D::Def));
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("-0x0.0000000000001p-1022")), D::Trv), nd2di(hexf64!("-0x0.0000000000001p-1022"), hexf64!("-0x0.0000000000001p-1022"), D::Trv));
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022")), D::Def), nd2di(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022"), D::Def));
    assert_eq!(DI::set_dec(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022")), D::Dac), nd2di(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x0.0000000000001p-1022"), D::Dac));
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")), D::Com), nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023"), D::Com));
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")), D::Def), nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Def));
    assert_eq!(DI::set_dec(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")), D::Trv), nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Trv));
    assert_eq!(DI::set_dec(n2i(f64::NEG_INFINITY, f64::INFINITY), D::Dac), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(DI::set_dec(I::empty(), D::Trv), DI::empty());
    assert_eq!(DI::set_dec(n2i(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4")), D::Com), nd2di(hexf64!("-0x1.99999c0000000p+4"), hexf64!("0x1.9999999999999p-4"), D::Com));
    assert_eq!(DI::set_dec(I::empty(), D::Def), DI::empty());
    assert_eq!(DI::set_dec(I::empty(), D::Dac), DI::empty());
    assert_eq!(DI::set_dec(I::empty(), D::Com), DI::empty());
    assert_eq!(DI::set_dec(n2i(1.0, f64::INFINITY), D::Com), nd2di(1.0, f64::INFINITY, D::Dac));
    assert_eq!(DI::set_dec(n2i(f64::NEG_INFINITY, 3.0), D::Com), nd2di(f64::NEG_INFINITY, 3.0, D::Dac));
    assert_eq!(DI::set_dec(n2i(f64::NEG_INFINITY, f64::INFINITY), D::Com), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert!(DI::set_dec(I::empty(), D::Ill).is_nai());
    assert!(DI::set_dec(n2i(f64::NEG_INFINITY, 3.0), D::Ill).is_nai());
    assert!(DI::set_dec(n2i(-1.0, 3.0), D::Ill).is_nai());
}

#[test]
fn minimal_decoration_part_test() {
    assert_eq!(DI::nai().decoration_part(), D::Ill);
    assert_eq!(DI::empty().decoration_part(), D::Trv);
    assert_eq!(nd2di(-1.0, 3.0, D::Trv).decoration_part(), D::Trv);
    assert_eq!(nd2di(-1.0, 3.0, D::Def).decoration_part(), D::Def);
    assert_eq!(nd2di(-1.0, 3.0, D::Dac).decoration_part(), D::Dac);
    assert_eq!(nd2di(-1.0, 3.0, D::Com).decoration_part(), D::Com);
}