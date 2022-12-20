/*
 * Copyright 2022 XXIV
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
 */
use std::os::raw::c_void;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use webbrowser::open;
use webbrowser::open_browser;
use webbrowser::open_browser_with_options;
use webbrowser::Browser;
use webbrowser::BrowserOptions;

#[repr(C)]
struct webbrowser_browser_options_t {
    browser_options: *mut c_void
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum webbrowser_browser_t {
    WEBBROWSER_BROWSER_DEFAULT,
    WEBBROWSER_BROWSER_FIREFOX,
    WEBBROWSER_BROWSER_INTERNETEXPLORER,
    WEBBROWSER_BROWSER_CHROME,
    WEBBROWSER_BROWSER_OPERA,
    WEBBROWSER_BROWSER_SAFARI,
    WEBBROWSER_BROWSER_WEBPOSITIVE,
}

fn int_to_bool(int: c_int) -> bool {
    if int == 0 {
	false
    } else {
	true
    }
}

#[no_mangle]
unsafe extern "C" fn webbrowser_browser_options_new() -> webbrowser_browser_options_t {
    let browser_options = BrowserOptions::new();
    webbrowser_browser_options_t {
	browser_options: Box::into_raw(Box::new(browser_options)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn webbrowser_browser_options_with_suppress_output(suppress_output: c_int) -> webbrowser_browser_options_t {
    let mut browser_options = BrowserOptions::new();
    browser_options.with_suppress_output(int_to_bool(suppress_output));
    webbrowser_browser_options_t {
	browser_options: Box::into_raw(Box::new(browser_options)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn webbrowser_browser_options_with_target_hint(target_hint: *const c_char) -> webbrowser_browser_options_t {
    let target_hint_rs = match CStr::from_ptr(target_hint).to_str() {
	Ok(str) => str,
	Err(_) => return webbrowser_browser_options_new()
    };
    let mut browser_options = BrowserOptions::new();
    browser_options.with_target_hint(target_hint_rs);
    webbrowser_browser_options_t {
	browser_options: Box::into_raw(Box::new(browser_options)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn webbrowser_browser_options_with_dry_run(dry_run: c_int) -> webbrowser_browser_options_t {
    let mut browser_options = BrowserOptions::new();
    browser_options.with_dry_run(int_to_bool(dry_run));
    webbrowser_browser_options_t {
	browser_options: Box::into_raw(Box::new(browser_options)) as *mut c_void
    }
}

#[no_mangle]
unsafe extern "C" fn webbrowser_open(url: *const c_char) -> c_int {
    let url_rs = match CStr::from_ptr(url).to_str() {
	Ok(str) => str,
	Err(_) => return -1
    };
    match open(url_rs) {
	Ok(_) => 0,
	Err(_) => -1
    }
}

#[no_mangle]
unsafe extern "C" fn webbrowser_open_browser(browser: webbrowser_browser_t, url: *const c_char) -> c_int {
    let browser_rs = match browser {
	webbrowser_browser_t::WEBBROWSER_BROWSER_DEFAULT => Browser::Default,
	webbrowser_browser_t::WEBBROWSER_BROWSER_FIREFOX => Browser::Firefox,
	webbrowser_browser_t::WEBBROWSER_BROWSER_INTERNETEXPLORER => Browser::InternetExplorer,
	webbrowser_browser_t::WEBBROWSER_BROWSER_CHROME => Browser::Chrome,
	webbrowser_browser_t::WEBBROWSER_BROWSER_OPERA => Browser::Opera,
	webbrowser_browser_t::WEBBROWSER_BROWSER_SAFARI => Browser::Safari,
	webbrowser_browser_t::WEBBROWSER_BROWSER_WEBPOSITIVE => Browser::WebPositive,
    };
    let url_rs = match CStr::from_ptr(url).to_str() {
	Ok(str) => str,
	Err(_) => return -1
    };
    match open_browser(browser_rs, url_rs) {
	Ok(_) => 0,
	Err(_) => -1
    }
}

#[no_mangle]
unsafe extern "C" fn webbrowser_open_browser_with_options(browser: webbrowser_browser_t, url: *const c_char, options: *const webbrowser_browser_options_t) -> c_int {
    let browser_rs = match browser {
	webbrowser_browser_t::WEBBROWSER_BROWSER_DEFAULT => Browser::Default,
	webbrowser_browser_t::WEBBROWSER_BROWSER_FIREFOX => Browser::Firefox,
	webbrowser_browser_t::WEBBROWSER_BROWSER_INTERNETEXPLORER => Browser::InternetExplorer,
	webbrowser_browser_t::WEBBROWSER_BROWSER_CHROME => Browser::Chrome,
	webbrowser_browser_t::WEBBROWSER_BROWSER_OPERA => Browser::Opera,
	webbrowser_browser_t::WEBBROWSER_BROWSER_SAFARI => Browser::Safari,
	webbrowser_browser_t::WEBBROWSER_BROWSER_WEBPOSITIVE => Browser::WebPositive,

    };
    let url_rs = match CStr::from_ptr(url).to_str() {
	Ok(str) => str,
	Err(_) => return -1
    };
    let options_rs = &*((*options).browser_options as *mut BrowserOptions);
    match open_browser_with_options(browser_rs, url_rs, options_rs) {
	Ok(_) => 0,
	Err(_) => -1
    }
}

#[no_mangle]
unsafe extern "C" fn webbrowser_browser_options_clean(browser_options: *mut webbrowser_browser_options_t) {
    if !browser_options.is_null() {
	let _ = Box::from_raw((*browser_options).browser_options as *mut BrowserOptions);
    }
}
