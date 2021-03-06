// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! This module defines the 32-Bit Windows Base APIs
//99
pub const FILE_BEGIN: ::DWORD = 0;
pub const FILE_CURRENT: ::DWORD = 1;
pub const FILE_END: ::DWORD = 2;
//123
pub const FILE_FLAG_WRITE_THROUGH: ::DWORD = 0x80000000;
pub const FILE_FLAG_OVERLAPPED: ::DWORD = 0x40000000;
pub const FILE_FLAG_NO_BUFFERING: ::DWORD = 0x20000000;
pub const FILE_FLAG_RANDOM_ACCESS: ::DWORD = 0x10000000;
pub const FILE_FLAG_SEQUENTIAL_SCAN: ::DWORD = 0x08000000;
pub const FILE_FLAG_DELETE_ON_CLOSE: ::DWORD = 0x04000000;
pub const FILE_FLAG_BACKUP_SEMANTICS: ::DWORD = 0x02000000;
pub const FILE_FLAG_POSIX_SEMANTICS: ::DWORD = 0x01000000;
pub const FILE_FLAG_SESSION_AWARE: ::DWORD = 0x00800000;
pub const FILE_FLAG_OPEN_REPARSE_POINT: ::DWORD = 0x00200000;
pub const FILE_FLAG_OPEN_NO_RECALL: ::DWORD = 0x00100000;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: ::DWORD = 0x00080000;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: ::DWORD = 0x00040000;
pub const PROGRESS_CONTINUE: ::DWORD = 0;
pub const PROGRESS_CANCEL: ::DWORD = 1;
pub const PROGRESS_STOP: ::DWORD = 2;
pub const PROGRESS_QUIET: ::DWORD = 3;
pub const CALLBACK_CHUNK_FINISHED: ::DWORD = 0x00000000;
pub const CALLBACK_STREAM_SWITCH: ::DWORD = 0x00000001;
pub const COPY_FILE_FAIL_IF_EXISTS: ::DWORD = 0x00000001;
pub const COPY_FILE_RESTARTABLE: ::DWORD = 0x00000002;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: ::DWORD = 0x00000004;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: ::DWORD = 0x00000008;
pub const COPY_FILE_COPY_SYMLINK: ::DWORD = 0x00000800;
pub const COPY_FILE_NO_BUFFERING: ::DWORD = 0x00001000;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: ::DWORD = 0x00002000;
pub const COPY_FILE_RESUME_FROM_PAUSE: ::DWORD = 0x00004000;
pub const COPY_FILE_NO_OFFLOAD: ::DWORD = 0x00040000;
pub const REPLACEFILE_WRITE_THROUGH: ::DWORD = 0x00000001;
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: ::DWORD = 0x00000002;
pub const REPLACEFILE_IGNORE_ACL_ERRORS: ::DWORD = 0x00000004;
pub const PIPE_ACCESS_INBOUND: ::DWORD = 0x00000001;
pub const PIPE_ACCESS_OUTBOUND: ::DWORD = 0x00000002;
pub const PIPE_ACCESS_DUPLEX: ::DWORD = 0x00000003;
pub const PIPE_CLIENT_END: ::DWORD = 0x00000000;
pub const PIPE_SERVER_END: ::DWORD = 0x00000001;
pub const PIPE_WAIT: ::DWORD = 0x00000000;
pub const PIPE_NOWAIT: ::DWORD = 0x00000001;
pub const PIPE_READMODE_BYTE: ::DWORD = 0x00000000;
pub const PIPE_READMODE_MESSAGE: ::DWORD = 0x00000002;
pub const PIPE_TYPE_BYTE: ::DWORD = 0x00000000;
pub const PIPE_TYPE_MESSAGE: ::DWORD = 0x00000004;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: ::DWORD = 0x00000000;
pub const PIPE_REJECT_REMOTE_CLIENTS: ::DWORD = 0x00000008;
pub const PIPE_UNLIMITED_INSTANCES: ::DWORD = 255;
//268
pub const SECURITY_CONTEXT_TRACKING: ::DWORD = 0x00040000;
pub const SECURITY_EFFECTIVE_ONLY: ::DWORD = 0x00080000;
pub const SECURITY_SQOS_PRESENT: ::DWORD = 0x00100000;
pub const SECURITY_VALID_SQOS_FLAGS: ::DWORD = 0x001F0000;
//687
pub const STD_INPUT_HANDLE: ::DWORD = 0xFFFFFFF6;
pub const STD_OUTPUT_HANDLE: ::DWORD = 0xFFFFFFF5;
pub const STD_ERROR_HANDLE: ::DWORD = 0xFFFFFFF4;
pub const NOPARITY: ::DWORD = 0;
pub const ODDPARITY: ::DWORD = 1;
pub const EVENPARITY: ::DWORD = 2;
pub const MARKPARITY: ::DWORD = 3;
pub const SPACEPARITY: ::DWORD = 4;
pub const ONESTOPBIT: ::DWORD = 0;
pub const ONE5STOPBITS: ::DWORD = 1;
pub const TWOSTOPBITS: ::DWORD = 2;
pub const IGNORE: ::DWORD = 0;
pub const INFINITE: ::DWORD = 0xFFFFFFFF;
//2939
// startup flags STARTUPINFOW::dwFlags
pub const STARTF_USESHOWWINDOW: ::DWORD = 0x00000001;
pub const STARTF_USESIZE: ::DWORD = 0x00000002;
pub const STARTF_USEPOSITION: ::DWORD = 0x00000004;
pub const STARTF_USECOUNTCHARS: ::DWORD = 0x00000008;
pub const STARTF_USEFILLATTRIBUTE: ::DWORD = 0x00000010;
pub const STARTF_RUNFULLSCREEN: ::DWORD = 0x00000020;
pub const STARTF_FORCEONFEEDBACK: ::DWORD = 0x00000040;
pub const STARTF_FORCEOFFFEEDBACK: ::DWORD = 0x00000080;
pub const STARTF_USESTDHANDLES: ::DWORD = 0x00000100;
pub const STARTF_USEHOTKEY: ::DWORD = 0x00000200;
pub const STARTF_TITLEISLINKNAME: ::DWORD = 0x00000800;
pub const STARTF_TITLEISAPPID: ::DWORD = 0x00001000;
pub const STARTF_PREVENTPINNING: ::DWORD = 0x00002000;
//5454
pub const MOVEFILE_REPLACE_EXISTING: ::DWORD = 0x00000001;
pub const MOVEFILE_COPY_ALLOWED: ::DWORD = 0x00000002;
pub const MOVEFILE_DELAY_UNTIL_REBOOT: ::DWORD = 0x00000004;
pub const MOVEFILE_WRITE_THROUGH: ::DWORD = 0x00000008;
pub const MOVEFILE_CREATE_HARDLINK: ::DWORD = 0x00000010;
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: ::DWORD = 0x00000020;

pub const FORMAT_MESSAGE_IGNORE_INSERTS: ::DWORD = 0x00000200;
pub const FORMAT_MESSAGE_FROM_STRING: ::DWORD = 0x00000400;
pub const FORMAT_MESSAGE_FROM_HMODULE: ::DWORD = 0x00000800;
pub const FORMAT_MESSAGE_FROM_SYSTEM: ::DWORD = 0x00001000;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: ::DWORD = 0x00002000;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: ::DWORD = 0x000000FF;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: ::DWORD = 0x00000100;

pub const DEBUG_PROCESS: ::DWORD = 0x00000001;
pub const DEBUG_ONLY_THIS_PROCESS: ::DWORD = 0x00000002;
pub const CREATE_SUSPENDED: ::DWORD = 0x00000004;
pub const DETACHED_PROCESS: ::DWORD = 0x00000008;

pub const CREATE_NEW_CONSOLE: ::DWORD = 0x00000010;
pub const NORMAL_PRIORITY_CLASS: ::DWORD = 0x00000020;
pub const IDLE_PRIORITY_CLASS: ::DWORD = 0x00000040;
pub const HIGH_PRIORITY_CLASS: ::DWORD = 0x00000080;

pub const REALTIME_PRIORITY_CLASS: ::DWORD = 0x00000100;
pub const CREATE_NEW_PROCESS_GROUP: ::DWORD = 0x00000200;
pub const CREATE_UNICODE_ENVIRONMENT: ::DWORD = 0x00000400;
pub const CREATE_SEPARATE_WOW_VDM: ::DWORD = 0x00000800;

pub const CREATE_SHARED_WOW_VDM: ::DWORD = 0x00001000;
pub const CREATE_FORCEDOS: ::DWORD = 0x00002000;
pub const BELOW_NORMAL_PRIORITY_CLASS: ::DWORD = 0x00004000;
pub const ABOVE_NORMAL_PRIORITY_CLASS: ::DWORD = 0x00008000;

pub const INHERIT_PARENT_AFFINITY: ::DWORD = 0x00010000;
pub const INHERIT_CALLER_PRIORITY: ::DWORD = 0x00020000;
pub const CREATE_PROTECTED_PROCESS: ::DWORD = 0x00040000;
pub const EXTENDED_STARTUPINFO_PRESENT: ::DWORD = 0x00080000;

pub const PROCESS_MODE_BACKGROUND_BEGIN: ::DWORD = 0x00100000;
pub const PROCESS_MODE_BACKGROUND_END: ::DWORD = 0x00200000;

pub const CREATE_BREAKAWAY_FROM_JOB: ::DWORD = 0x01000000;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: ::DWORD = 0x02000000;
pub const CREATE_DEFAULT_ERROR_MODE: ::DWORD = 0x04000000;
pub const CREATE_NO_WINDOW: ::DWORD = 0x08000000;

pub const PROFILE_USER: ::DWORD = 0x10000000;
pub const PROFILE_KERNEL: ::DWORD = 0x20000000;
pub const PROFILE_SERVER: ::DWORD = 0x40000000;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: ::DWORD = 0x80000000;

pub const THREAD_BASE_PRIORITY_LOWRT: ::DWORD = 15;
pub const THREAD_BASE_PRIORITY_MAX: ::DWORD = 2;
pub const THREAD_BASE_PRIORITY_MIN: ::DWORD = -2i32 as ::DWORD;
pub const THREAD_BASE_PRIORITY_IDLE: ::DWORD = -15i32 as ::DWORD;

pub const THREAD_PRIORITY_LOWEST: ::DWORD = THREAD_BASE_PRIORITY_MIN;
pub const THREAD_PRIORITY_BELOW_NORMAL: ::DWORD = THREAD_PRIORITY_LOWEST + 1;
pub const THREAD_PRIORITY_NORMAL: ::DWORD = 0;
pub const THREAD_PRIORITY_HIGHEST: ::DWORD = THREAD_BASE_PRIORITY_MAX;
pub const THREAD_PRIORITY_ABOVE_NORMAL: ::DWORD = THREAD_PRIORITY_HIGHEST - 1;
pub const THREAD_PRIORITY_ERROR_RETURN: ::DWORD = 0x7fffffff;

pub const THREAD_PRIORITY_TIME_CRITICAL: ::DWORD = THREAD_BASE_PRIORITY_LOWRT;
pub const THREAD_PRIORITY_IDLE: ::DWORD = THREAD_BASE_PRIORITY_IDLE;

pub const THREAD_MODE_BACKGROUND_BEGIN: ::DWORD = 0x00010000;
pub const THREAD_MODE_BACKGROUND_END: ::DWORD = 0x00020000;
