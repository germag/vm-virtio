/* automatically generated by rust-bindgen 0.63.0 */

pub const VIRTIO_SCSI_CDB_DEFAULT_SIZE: u32 = 32;
pub const VIRTIO_SCSI_SENSE_DEFAULT_SIZE: u32 = 96;
pub const VIRTIO_SCSI_CDB_SIZE: u32 = 32;
pub const VIRTIO_SCSI_SENSE_SIZE: u32 = 96;
pub const VIRTIO_SCSI_F_INOUT: u32 = 0;
pub const VIRTIO_SCSI_F_HOTPLUG: u32 = 1;
pub const VIRTIO_SCSI_F_CHANGE: u32 = 2;
pub const VIRTIO_SCSI_F_T10_PI: u32 = 3;
pub const VIRTIO_SCSI_S_OK: u32 = 0;
pub const VIRTIO_SCSI_S_OVERRUN: u32 = 1;
pub const VIRTIO_SCSI_S_ABORTED: u32 = 2;
pub const VIRTIO_SCSI_S_BAD_TARGET: u32 = 3;
pub const VIRTIO_SCSI_S_RESET: u32 = 4;
pub const VIRTIO_SCSI_S_BUSY: u32 = 5;
pub const VIRTIO_SCSI_S_TRANSPORT_FAILURE: u32 = 6;
pub const VIRTIO_SCSI_S_TARGET_FAILURE: u32 = 7;
pub const VIRTIO_SCSI_S_NEXUS_FAILURE: u32 = 8;
pub const VIRTIO_SCSI_S_FAILURE: u32 = 9;
pub const VIRTIO_SCSI_S_FUNCTION_SUCCEEDED: u32 = 10;
pub const VIRTIO_SCSI_S_FUNCTION_REJECTED: u32 = 11;
pub const VIRTIO_SCSI_S_INCORRECT_LUN: u32 = 12;
pub const VIRTIO_SCSI_T_TMF: u32 = 0;
pub const VIRTIO_SCSI_T_AN_QUERY: u32 = 1;
pub const VIRTIO_SCSI_T_AN_SUBSCRIBE: u32 = 2;
pub const VIRTIO_SCSI_T_TMF_ABORT_TASK: u32 = 0;
pub const VIRTIO_SCSI_T_TMF_ABORT_TASK_SET: u32 = 1;
pub const VIRTIO_SCSI_T_TMF_CLEAR_ACA: u32 = 2;
pub const VIRTIO_SCSI_T_TMF_CLEAR_TASK_SET: u32 = 3;
pub const VIRTIO_SCSI_T_TMF_I_T_NEXUS_RESET: u32 = 4;
pub const VIRTIO_SCSI_T_TMF_LOGICAL_UNIT_RESET: u32 = 5;
pub const VIRTIO_SCSI_T_TMF_QUERY_TASK: u32 = 6;
pub const VIRTIO_SCSI_T_TMF_QUERY_TASK_SET: u32 = 7;
pub const VIRTIO_SCSI_T_EVENTS_MISSED: u32 = 2147483648;
pub const VIRTIO_SCSI_T_NO_EVENT: u32 = 0;
pub const VIRTIO_SCSI_T_TRANSPORT_RESET: u32 = 1;
pub const VIRTIO_SCSI_T_ASYNC_NOTIFY: u32 = 2;
pub const VIRTIO_SCSI_T_PARAM_CHANGE: u32 = 3;
pub const VIRTIO_SCSI_EVT_RESET_HARD: u32 = 0;
pub const VIRTIO_SCSI_EVT_RESET_RESCAN: u32 = 1;
pub const VIRTIO_SCSI_EVT_RESET_REMOVED: u32 = 2;
pub const VIRTIO_SCSI_S_SIMPLE: u32 = 0;
pub const VIRTIO_SCSI_S_ORDERED: u32 = 1;
pub const VIRTIO_SCSI_S_HEAD: u32 = 2;
pub const VIRTIO_SCSI_S_ACA: u32 = 3;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __u32 = ::std::os::raw::c_uint;
pub type __u64 = ::std::os::raw::c_ulonglong;
pub type __virtio16 = __u16;
pub type __virtio32 = __u32;
pub type __virtio64 = __u64;
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_cmd_req {
    pub lun: [__u8; 8usize],
    pub tag: __virtio64,
    pub task_attr: __u8,
    pub prio: __u8,
    pub crn: __u8,
    pub cdb: [__u8; 32usize],
}
#[test]
fn bindgen_test_layout_virtio_scsi_cmd_req() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_cmd_req> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_cmd_req>(),
        51usize,
        concat!("Size of: ", stringify!(virtio_scsi_cmd_req))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_cmd_req>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_cmd_req))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lun) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req),
            "::",
            stringify!(lun)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tag) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req),
            "::",
            stringify!(tag)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).task_attr) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req),
            "::",
            stringify!(task_attr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prio) as usize - ptr as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req),
            "::",
            stringify!(prio)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).crn) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req),
            "::",
            stringify!(crn)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cdb) as usize - ptr as usize },
        19usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req),
            "::",
            stringify!(cdb)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_cmd_req_pi {
    pub lun: [__u8; 8usize],
    pub tag: __virtio64,
    pub task_attr: __u8,
    pub prio: __u8,
    pub crn: __u8,
    pub pi_bytesout: __virtio32,
    pub pi_bytesin: __virtio32,
    pub cdb: [__u8; 32usize],
}
#[test]
fn bindgen_test_layout_virtio_scsi_cmd_req_pi() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_cmd_req_pi> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_cmd_req_pi>(),
        59usize,
        concat!("Size of: ", stringify!(virtio_scsi_cmd_req_pi))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_cmd_req_pi>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_cmd_req_pi))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lun) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req_pi),
            "::",
            stringify!(lun)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tag) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req_pi),
            "::",
            stringify!(tag)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).task_attr) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req_pi),
            "::",
            stringify!(task_attr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prio) as usize - ptr as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req_pi),
            "::",
            stringify!(prio)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).crn) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req_pi),
            "::",
            stringify!(crn)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pi_bytesout) as usize - ptr as usize },
        19usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req_pi),
            "::",
            stringify!(pi_bytesout)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pi_bytesin) as usize - ptr as usize },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req_pi),
            "::",
            stringify!(pi_bytesin)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cdb) as usize - ptr as usize },
        27usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_req_pi),
            "::",
            stringify!(cdb)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct virtio_scsi_cmd_resp {
    pub sense_len: __virtio32,
    pub resid: __virtio32,
    pub status_qualifier: __virtio16,
    pub status: __u8,
    pub response: __u8,
    pub sense: [__u8; 96usize],
}
#[test]
fn bindgen_test_layout_virtio_scsi_cmd_resp() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_cmd_resp> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_cmd_resp>(),
        108usize,
        concat!("Size of: ", stringify!(virtio_scsi_cmd_resp))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_cmd_resp>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_cmd_resp))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sense_len) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_resp),
            "::",
            stringify!(sense_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).resid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_resp),
            "::",
            stringify!(resid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status_qualifier) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_resp),
            "::",
            stringify!(status_qualifier)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_resp),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response) as usize - ptr as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_resp),
            "::",
            stringify!(response)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sense) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_cmd_resp),
            "::",
            stringify!(sense)
        )
    );
}
impl Default for virtio_scsi_cmd_resp {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_ctrl_tmf_req {
    pub type_: __virtio32,
    pub subtype: __virtio32,
    pub lun: [__u8; 8usize],
    pub tag: __virtio64,
}
#[test]
fn bindgen_test_layout_virtio_scsi_ctrl_tmf_req() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_ctrl_tmf_req> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_ctrl_tmf_req>(),
        24usize,
        concat!("Size of: ", stringify!(virtio_scsi_ctrl_tmf_req))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_ctrl_tmf_req>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_ctrl_tmf_req))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_tmf_req),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).subtype) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_tmf_req),
            "::",
            stringify!(subtype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lun) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_tmf_req),
            "::",
            stringify!(lun)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tag) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_tmf_req),
            "::",
            stringify!(tag)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_ctrl_tmf_resp {
    pub response: __u8,
}
#[test]
fn bindgen_test_layout_virtio_scsi_ctrl_tmf_resp() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_ctrl_tmf_resp> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_ctrl_tmf_resp>(),
        1usize,
        concat!("Size of: ", stringify!(virtio_scsi_ctrl_tmf_resp))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_ctrl_tmf_resp>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_ctrl_tmf_resp))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_tmf_resp),
            "::",
            stringify!(response)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_ctrl_an_req {
    pub type_: __virtio32,
    pub lun: [__u8; 8usize],
    pub event_requested: __virtio32,
}
#[test]
fn bindgen_test_layout_virtio_scsi_ctrl_an_req() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_ctrl_an_req> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_ctrl_an_req>(),
        16usize,
        concat!("Size of: ", stringify!(virtio_scsi_ctrl_an_req))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_ctrl_an_req>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_ctrl_an_req))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_an_req),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lun) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_an_req),
            "::",
            stringify!(lun)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_requested) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_an_req),
            "::",
            stringify!(event_requested)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_ctrl_an_resp {
    pub event_actual: __virtio32,
    pub response: __u8,
}
#[test]
fn bindgen_test_layout_virtio_scsi_ctrl_an_resp() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_ctrl_an_resp> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_ctrl_an_resp>(),
        5usize,
        concat!("Size of: ", stringify!(virtio_scsi_ctrl_an_resp))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_ctrl_an_resp>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_ctrl_an_resp))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_actual) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_an_resp),
            "::",
            stringify!(event_actual)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).response) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_ctrl_an_resp),
            "::",
            stringify!(response)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_event {
    pub event: __virtio32,
    pub lun: [__u8; 8usize],
    pub reason: __virtio32,
}
#[test]
fn bindgen_test_layout_virtio_scsi_event() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_event> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_event>(),
        16usize,
        concat!("Size of: ", stringify!(virtio_scsi_event))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_event>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_event))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_event),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lun) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_event),
            "::",
            stringify!(lun)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reason) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_event),
            "::",
            stringify!(reason)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct virtio_scsi_config {
    pub num_queues: __virtio32,
    pub seg_max: __virtio32,
    pub max_sectors: __virtio32,
    pub cmd_per_lun: __virtio32,
    pub event_info_size: __virtio32,
    pub sense_size: __virtio32,
    pub cdb_size: __virtio32,
    pub max_channel: __virtio16,
    pub max_target: __virtio16,
    pub max_lun: __virtio32,
}
#[test]
fn bindgen_test_layout_virtio_scsi_config() {
    const UNINIT: ::std::mem::MaybeUninit<virtio_scsi_config> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<virtio_scsi_config>(),
        36usize,
        concat!("Size of: ", stringify!(virtio_scsi_config))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_scsi_config>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_scsi_config))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).num_queues) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(num_queues)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).seg_max) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(seg_max)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_sectors) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(max_sectors)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cmd_per_lun) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(cmd_per_lun)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event_info_size) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(event_info_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sense_size) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(sense_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cdb_size) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(cdb_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_channel) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(max_channel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_target) as usize - ptr as usize },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(max_target)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_lun) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(virtio_scsi_config),
            "::",
            stringify!(max_lun)
        )
    );
}