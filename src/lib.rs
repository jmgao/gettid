// Copyright (C) 2019 by Josh Gao <josh@jmgao.dev>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

#![doc = include_str!("../README.md")]

/// Get the current thread's thread id.
///
/// ```
/// use gettid::gettid;
/// let main_tid = gettid();
/// let pid = std::process::id();
///
/// if cfg!(target_os = "linux") {
///     // On Linux, the first thread ID is the same as the PID
///     assert_eq!(pid as u64, main_tid);
/// }
///
/// // gettid() returns a consistent value for a given thread.
/// assert_eq!(main_tid, gettid());
///
/// let thread_tid = std::thread::spawn(gettid).join().unwrap();
/// assert_ne!(main_tid, thread_tid);
/// ```
pub fn gettid() -> u64 {
  gettid_impl()
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn gettid_impl() -> u64 {
  unsafe { libc::gettid() as u64 }
}

#[cfg(target_os = "macos")]
pub fn gettid_impl() -> u64 {
  let mut result = 0;
  let res = unsafe { libc::pthread_threadid_np(0, &mut result) };
  assert_eq!(res, 0, "error retrieving thread ID");
  result
}

#[cfg(target_os = "freebsd")]
pub fn gettid_impl() -> u64 {
  unsafe { libc::pthread_getthreadid_np() as u64 }
}

#[cfg(target_os = "openbsd")]
pub fn gettid_impl() -> u64 {
  unsafe { libc::getthrid() as u64 }
}

#[cfg(target_os = "netbsd")]
pub fn gettid_impl() -> u64 {
  unsafe { libc::_lwp_self() as u64 }
}

#[cfg(target_os = "windows")]
pub fn gettid_impl() -> u64 {
  unsafe extern "system" {
    fn GetCurrentThreadId() -> u32;
  }

  unsafe { GetCurrentThreadId().into() }
}
