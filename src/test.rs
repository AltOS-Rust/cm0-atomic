/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

macro_rules! atomic {
    { $( $code:tt )* } => {{
        __atomic_inner!($($code)*)
    }};
}

macro_rules! __atomic_inner {
    () => {};
    ( $first:stmt; $($rest:tt)* ) => {{
        $first;
        __atomic_inner!($($rest)*)
    }};
    ( $first:expr; $($rest:tt)* ) => {{
        $first;
        __atomic_inner!($($rest)*)
    }};
    ( $first:stmt; ) => {{
        $first;
    }};
    ( $first:expr ) => {{
        $first
    }};
}
