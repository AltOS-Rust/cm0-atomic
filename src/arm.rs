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

macro_rules! __start_critical {
    () => {{
        unsafe {
            let primask: u32;
            asm!(
                concat!(
                    "mrs $0, PRIMASK\n",
                    "cpsid i\n"
                )
                : "=r"(primask)
                : /* no inputs */
                : /* no clobbers */
                : "volatile"
            );
            primask
        }
    }}
}

macro_rules! __end_critical {
    ($var:ident) => {{
        unsafe {
            asm!("msr PRIMASK, $0"
            : /* no outputs */
            : "r"($var)
            : /* no clobbers */
            : "volatile");
        }
    }}
}

macro_rules! atomic {
    { $( $code:tt )* } => {{
        #[cfg(target_arch="arm")]
        let primask: u32 = __start_critical!();
        let result = __atomic_inner!($($code)*);
        #[cfg(target_arch="arm")]
        __end_critical!(primask);
        result
    }}
}

macro_rules! __atomic_inner {
    // Recursive base case
    () => {};

    // If the first item in the tree is a statement
    ( $first:stmt; $($rest:tt)* ) => {{
        $first;
        __atomic_inner!($($rest)*)
    }};

    // If the first item in the tree is an expression
    ( $first:expr; $($rest:tt)* ) => {{
        $first;
        __atomic_inner!($($rest)*)
    }};

    // Handle only one statement
    ( $first:stmt; ) => {{
        $first;
    }};

    // Handle only one expression
    ( $first:expr ) => {{
        $first
    }};
}
