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

//! A crate to help with fetching thread ids across multiple platforms.

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
/// let thread_tid = std::thread::spawn(gettid).join().unwrap();
/// assert_ne!(main_tid, thread_tid);
/// ```
pub fn gettid() -> u64 {
  imp::gettid()
}

#[cfg(any(target_os = "linux", target_os = "android"))]
mod imp {
  pub fn gettid() -> u64 {
    unsafe { libc::syscall(libc::SYS_gettid) as u64 }
  }
}

#[cfg(target_os = "macos")]
mod imp {
  #[link(name = "pthread")]
  extern "C" {
    fn pthread_threadid_np(thread: libc::pthread_t, thread_id: *mut u64) -> libc::c_int;
  }

  pub fn gettid() -> u64 {
    let mut result = 0;
    let res = unsafe { pthread_threadid_np(0, &mut result) };
    assert_eq!(res, 0, "error retrieving thread ID");
    result
  }
}

#[cfg(target_os = "freebsd")]
mod imp {
  #[link(name = "pthread")]
  extern "C" {
    fn pthread_getthreadid_np(thread: libc::pthread_t, thread_id: *mut u64) -> libc::c_int;
  }

  pub fn gettid() -> u64 {
    let mut result = 0;
    unsafe {let _ = pthread_getthreadid_np(0, &mut result); }
    result
  }
}

#[cfg(target_os = "windows")]
mod imp {
  pub fn gettid() -> u64 {
    unsafe { winapi::um::processthreadsapi::GetCurrentThreadId().into() }
  }
}
