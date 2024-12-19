#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate libc;
extern crate nfc_sys;

use libc::{off_t, size_t, ssize_t};

use nfc_sys::{nfc_device, nfc_target, nfc_iso14443a_info};

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_freefare_tag_type {
    FELICA = 0,
    MIFARE_CLASSIC_1K = 1,
    MIFARE_CLASSIC_4K = 2,
    MIFARE_DESFIRE = 3,
    MIFARE_ULTRALIGHT = 4,
    MIFARE_ULTRALIGHT_C = 5,
}
pub enum Struct_freefare_tag {
}
pub type FreefareTag = *mut Struct_freefare_tag;
pub type MifareTag = *mut Struct_freefare_tag;
pub enum Struct_mifare_desfire_key {
}
pub type MifareDESFireKey = *mut Struct_mifare_desfire_key;
pub type MifareUltralightPageNumber = u8;
pub type MifareUltralightPage = [::std::os::raw::c_uchar; 4usize];
pub type MifareClassicBlock = [::std::os::raw::c_uchar; 16usize];
pub type MifareClassicSectorNumber = u8;
pub type MifareClassicBlockNumber = ::std::os::raw::c_uchar;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_Unnamed1 {
    MFC_KEY_A = 0,
    MFC_KEY_B = 1,
}
pub type MifareClassicKeyType = Enum_Unnamed1;
pub type MifareClassicKey = [::std::os::raw::c_uchar; 6usize];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_mad_aid {
    pub application_code: u8,
    pub function_cluster_code: u8,
}
impl ::std::default::Default for Struct_mad_aid {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type MadAid = Struct_mad_aid;
pub enum Struct_mad {
}
pub type Mad = *mut Struct_mad;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Enum_mifare_desfire_file_types {
    MDFT_STANDARD_DATA_FILE = 0,
    MDFT_BACKUP_DATA_FILE = 1,
    MDFT_VALUE_FILE_WITH_BACKUP = 2,
    MDFT_LINEAR_RECORD_FILE_WITH_BACKUP = 3,
    MDFT_CYCLIC_RECORD_FILE_WITH_BACKUP = 4,
}
pub enum Struct_mifare_desfire_aid {
}
pub type MifareDESFireAID = *mut Struct_mifare_desfire_aid;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_mifare_desfire_df {
    pub aid: u32,
    pub fid: u16,
    pub df_name: [u8; 16usize],
    pub df_name_len: size_t,
}
impl ::std::default::Default for Struct_mifare_desfire_df {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type MifareDESFireDF = Struct_mifare_desfire_df;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_mifare_desfire_version_info {
    pub hardware: Struct_Unnamed2,
    pub software: Struct_Unnamed3,
    pub uid: [u8; 7usize],
    pub batch_number: [u8; 5usize],
    pub production_week: u8,
    pub production_year: u8,
}
impl ::std::default::Default for Struct_mifare_desfire_version_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed2 {
    pub vendor_id: u8,
    pub _type: u8,
    pub subtype: u8,
    pub version_major: u8,
    pub version_minor: u8,
    pub storage_size: u8,
    pub protocol: u8,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed3 {
    pub vendor_id: u8,
    pub _type: u8,
    pub subtype: u8,
    pub version_major: u8,
    pub version_minor: u8,
    pub storage_size: u8,
    pub protocol: u8,
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_mifare_desfire_file_settings {
    pub file_type: u8,
    pub communication_settings: u8,
    pub access_rights: u16,
    pub settings: Union_Unnamed4,
}
impl ::std::default::Default for Struct_mifare_desfire_file_settings {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed4 {
    pub _bindgen_data_: [u32; 4usize],
}
impl Union_Unnamed4 {
    pub unsafe fn standard_file(&mut self) -> *mut Struct_Unnamed5 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn value_file(&mut self) -> *mut Struct_Unnamed6 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn linear_record_file(&mut self) -> *mut Struct_Unnamed7 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed5 {
    pub file_size: u32,
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed6 {
    pub lower_limit: i32,
    pub upper_limit: i32,
    pub limited_credit_value: i32,
    pub limited_credit_enabled: u8,
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed7 {
    pub record_size: u32,
    pub max_number_of_records: u32,
    pub current_number_of_records: u32,
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ntag21x_key {
    _unused: [u8; 0],
}
pub type NTAG21xKey = *mut ntag21x_key;

pub const ntag_tag_subtype_NTAG_213: ntag_tag_subtype = 0;
pub const ntag_tag_subtype_NTAG_215: ntag_tag_subtype = 1;
pub const ntag_tag_subtype_NTAG_216: ntag_tag_subtype = 2;
pub type ntag_tag_subtype = u32;

#[link(name = "freefare", kind = "dylib")]
extern "C" {
    pub static mifare_classic_nfcforum_public_key_a: MifareClassicKey;
    pub static mad_public_key_a: MifareClassicKey;
    pub static mad_free_aid: MadAid;
    pub static mad_defect_aid: MadAid;
    pub static mad_reserved_aid: MadAid;
    pub static mad_card_holder_aid: MadAid;
    pub static mad_not_applicable_aid: MadAid;
    pub static mad_nfcforum_aid: MadAid;
}
#[link(name = "freefare", kind = "dylib")]
extern "C" {
    pub fn freefare_get_tags(device: *mut nfc_device) -> *mut FreefareTag;
    pub fn freefare_tag_new(device: *mut nfc_device, target: nfc_target) -> FreefareTag;
    pub fn freefare_get_tag_type(tag: FreefareTag) -> Enum_freefare_tag_type;
    pub fn freefare_get_tag_friendly_name(tag: FreefareTag) -> *const ::std::os::raw::c_char;
    pub fn freefare_get_tag_uid(tag: MifareTag) -> *mut ::std::os::raw::c_char;
    pub fn freefare_free_tag(tag: FreefareTag);
    pub fn freefare_free_tags(tags: *mut FreefareTag);
    pub fn freefare_selected_tag_is_present(device: *mut nfc_device) -> u8;
    pub fn freefare_strerror(tag: FreefareTag) -> *const ::std::os::raw::c_char;
    pub fn freefare_strerror_r(
        tag: FreefareTag,
        buffer: *mut ::std::os::raw::c_char,
        len: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn freefare_perror(tag: FreefareTag, string: *const ::std::os::raw::c_char);
    pub fn felica_taste(device: *mut nfc_device, target: nfc_target) -> u8;
    pub fn felica_tag_new(device: *mut nfc_device, target: nfc_target) -> FreefareTag;
    pub fn felica_tag_free(tag: FreefareTag);
    pub fn felica_read(
        tag: FreefareTag,
        service: u16,
        block: u8,
        data: *mut u8,
        length: size_t,
    ) -> ssize_t;
    pub fn felica_read_ex(
        tag: FreefareTag,
        service: u16,
        block_count: u8,
        blocks: *mut u8,
        data: *mut u8,
        length: size_t,
    ) -> ssize_t;
    pub fn felica_write(
        tag: FreefareTag,
        service: u16,
        block: u8,
        data: *mut u8,
        length: size_t,
    ) -> ssize_t;
    pub fn felica_write_ex(
        tag: FreefareTag,
        service: u16,
        block_count: u8,
        blocks: *mut u8,
        data: *mut u8,
        length: size_t,
    ) -> ssize_t;
    pub fn mifare_ultralight_taste(device: *mut nfc_device, target: nfc_target) -> u8;
    pub fn mifare_ultralightc_taste(device: *mut nfc_device, target: nfc_target) -> u8;
    pub fn mifare_ultralight_tag_new(device: *mut nfc_device, target: nfc_target) -> FreefareTag;
    pub fn mifare_ultralightc_tag_new(device: *mut nfc_device, target: nfc_target) -> FreefareTag;
    pub fn mifare_ultralight_tag_free(tag: FreefareTag);
    pub fn mifare_ultralightc_tag_free(tag: FreefareTag);
    pub fn mifare_ultralight_connect(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_ultralight_disconnect(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_ultralight_read(
        tag: FreefareTag,
        page: MifareUltralightPageNumber,
        data: *mut MifareUltralightPage,
    ) -> ::std::os::raw::c_int;
    
    pub fn mifare_ultralight_write(
        tag: FreefareTag,
        page: MifareUltralightPageNumber,
        data: *const MifareUltralightPage,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_ultralightc_authenticate(
        tag: FreefareTag,
        key: MifareDESFireKey,
    ) -> ::std::os::raw::c_int;
    pub fn is_mifare_ultralight(tag: FreefareTag) -> u8;
    pub fn is_mifare_ultralightc(tag: FreefareTag) -> u8;
    pub fn is_mifare_ultralightc_on_reader(device: *mut nfc_device, nai: nfc_iso14443a_info) -> u8;
    pub fn mifare_classic1k_taste(device: *mut nfc_device, target: nfc_target) -> u8;
    pub fn mifare_classic4k_taste(device: *mut nfc_device, target: nfc_target) -> u8;
    pub fn mifare_classic1k_tag_new(device: *mut nfc_device, target: nfc_target) -> FreefareTag;
    pub fn mifare_classic4k_tag_new(device: *mut nfc_device, target: nfc_target) -> FreefareTag;
    pub fn mifare_classic_tag_free(tag: FreefareTag);
    pub fn mifare_classic_connect(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_classic_disconnect(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_classic_authenticate(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        key: *const MifareClassicKey,
        key_type: MifareClassicKeyType,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_read(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        data: *mut MifareClassicBlock,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_init_value(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        value: i32,
        adr: MifareClassicBlockNumber,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_read_value(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        value: *mut i32,
        adr: *mut MifareClassicBlockNumber,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_write(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        data: *const MifareClassicBlock,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_increment(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        amount: u32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_decrement(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        amount: u32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_restore(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_transfer(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_get_trailer_block_permission(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        permission: u16,
        key_type: MifareClassicKeyType,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_get_data_block_permission(
        tag: FreefareTag,
        block: MifareClassicBlockNumber,
        permission: ::std::os::raw::c_uchar,
        key_type: MifareClassicKeyType,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_format_sector(
        tag: FreefareTag,
        sector: MifareClassicSectorNumber,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_classic_trailer_block(
        block: *mut MifareClassicBlock,
        key_a: *const MifareClassicKey,
        ab_0: u8,
        ab_1: u8,
        ab_2: u8,
        ab_tb: u8,
        gpb: u8,
        key_b: *const MifareClassicKey,
    );
    pub fn mifare_classic_block_sector(
        block: MifareClassicBlockNumber,
    ) -> MifareClassicSectorNumber;
    pub fn mifare_classic_sector_first_block(
        sector: MifareClassicSectorNumber,
    ) -> MifareClassicBlockNumber;
    pub fn mifare_classic_sector_block_count(sector: MifareClassicSectorNumber) -> size_t;
    pub fn mifare_classic_sector_last_block(
        sector: MifareClassicSectorNumber,
    ) -> MifareClassicBlockNumber;
    pub fn mad_new(version: u8) -> Mad;
    pub fn mad_read(tag: FreefareTag) -> Mad;
    pub fn mad_write(
        tag: FreefareTag,
        mad: Mad,
        key_b_sector_00: *const MifareClassicKey,
        key_b_sector_10: *const MifareClassicKey,
    ) -> ::std::os::raw::c_int;
    pub fn mad_get_version(mad: Mad) -> ::std::os::raw::c_int;
    pub fn mad_set_version(mad: Mad, version: u8);
    pub fn mad_get_card_publisher_sector(mad: Mad) -> MifareClassicSectorNumber;
    pub fn mad_set_card_publisher_sector(
        mad: Mad,
        cps: MifareClassicSectorNumber,
    ) -> ::std::os::raw::c_int;
    pub fn mad_get_aid(
        mad: Mad,
        sector: MifareClassicSectorNumber,
        aid: *mut MadAid,
    ) -> ::std::os::raw::c_int;
    pub fn mad_set_aid(
        mad: Mad,
        sector: MifareClassicSectorNumber,
        aid: MadAid,
    ) -> ::std::os::raw::c_int;
    pub fn mad_sector_reserved(sector: MifareClassicSectorNumber) -> u8;
    pub fn mad_free(mad: Mad);
    pub fn mifare_application_alloc(
        mad: Mad,
        aid: MadAid,
        size: size_t,
    ) -> *mut MifareClassicSectorNumber;
    pub fn mifare_application_read(
        tag: FreefareTag,
        mad: Mad,
        aid: MadAid,
        buf: *mut ::std::os::raw::c_void,
        nbytes: size_t,
        key: *const MifareClassicKey,
        key_type: MifareClassicKeyType,
    ) -> ssize_t;
    pub fn mifare_application_write(
        tag: FreefareTag,
        mad: Mad,
        aid: MadAid,
        buf: *const ::std::os::raw::c_void,
        nbytes: size_t,
        key: *const MifareClassicKey,
        key_type: MifareClassicKeyType,
    ) -> ssize_t;
    pub fn mifare_application_free(mad: Mad, aid: MadAid) -> ::std::os::raw::c_int;
    pub fn mifare_application_find(mad: Mad, aid: MadAid) -> *mut MifareClassicSectorNumber;
    pub fn mifare_desfire_taste(device: *mut nfc_device, target: nfc_target) -> u8;
    pub fn mifare_desfire_aid_new(aid: u32) -> MifareDESFireAID;
    pub fn mifare_desfire_aid_new_with_mad_aid(mad_aid: MadAid, n: u8) -> MifareDESFireAID;
    pub fn mifare_desfire_aid_get_aid(aid: MifareDESFireAID) -> u32;
    pub fn mifare_desfire_last_pcd_error(tag: FreefareTag) -> u8;
    pub fn mifare_desfire_last_picc_error(tag: FreefareTag) -> u8;
    pub fn mifare_desfire_tag_new(device: *mut nfc_device, target: nfc_target) -> FreefareTag;
    pub fn mifare_desfire_tag_free(tags: FreefareTag);
    pub fn mifare_desfire_connect(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_disconnect(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_authenticate(
        tag: FreefareTag,
        key_no: u8,
        key: MifareDESFireKey,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_authenticate_iso(
        tag: FreefareTag,
        key_no: u8,
        key: MifareDESFireKey,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_authenticate_aes(
        tag: FreefareTag,
        key_no: u8,
        key: MifareDESFireKey,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_change_key_settings(
        tag: FreefareTag,
        settings: u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_key_settings(
        tag: FreefareTag,
        settings: *mut u8,
        max_keys: *mut u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_change_key(
        tag: FreefareTag,
        key_no: u8,
        new_key: MifareDESFireKey,
        old_key: MifareDESFireKey,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_key_version(
        tag: FreefareTag,
        key_no: u8,
        version: *mut u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application(
        tag: FreefareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_3k3des(
        tag: FreefareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_aes(
        tag: FreefareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_iso(
        tag: FreefareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: ::std::os::raw::c_int,
        iso_file_id: u16,
        iso_file_name: *mut u8,
        iso_file_name_len: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_3k3des_iso(
        tag: FreefareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: ::std::os::raw::c_int,
        iso_file_id: u16,
        iso_file_name: *mut u8,
        iso_file_name_len: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_application_aes_iso(
        tag: FreefareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: ::std::os::raw::c_int,
        iso_file_id: u16,
        iso_file_name: *mut u8,
        iso_file_name_len: size_t,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_delete_application(
        tag: FreefareTag,
        aid: MifareDESFireAID,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_application_ids(
        tag: FreefareTag,
        aids: *mut *mut MifareDESFireAID,
        count: *mut size_t,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_df_names(
        tag: FreefareTag,
        dfs: *mut *mut MifareDESFireDF,
        count: *mut size_t,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_free_application_ids(aids: *mut MifareDESFireAID);
    pub fn mifare_desfire_select_application(
        tag: FreefareTag,
        aid: MifareDESFireAID,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_format_picc(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_version(
        tag: FreefareTag,
        version_info: *mut Struct_mifare_desfire_version_info,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_free_mem(tag: FreefareTag, size: *mut u32) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_set_configuration(
        tag: FreefareTag,
        disable_format: u8,
        enable_random_uid: u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_set_default_key(
        tag: FreefareTag,
        key: MifareDESFireKey,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_set_ats(tag: FreefareTag, ats: *mut u8) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_card_uid(
        tag: FreefareTag,
        uid: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_file_ids(
        tag: FreefareTag,
        files: *mut *mut u8,
        count: *mut size_t,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_iso_file_ids(
        tag: FreefareTag,
        files: *mut *mut u16,
        count: *mut size_t,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_file_settings(
        tag: FreefareTag,
        file_no: u8,
        settings: *mut Struct_mifare_desfire_file_settings,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_change_file_settings(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_std_data_file(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_std_data_file_iso(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
        iso_file_id: u16,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_backup_data_file(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_backup_data_file_iso(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
        iso_file_id: u16,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_value_file(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        lower_limit: i32,
        upper_limit: i32,
        value: i32,
        limited_credit_enable: u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_linear_record_file(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_linear_record_file_iso(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
        iso_file_id: u16,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_cyclic_record_file(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_create_cyclic_record_file_iso(
        tag: FreefareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
        iso_file_id: u16,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_delete_file(tag: FreefareTag, file_no: u8) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_read_data(
        tag: FreefareTag,
        file_no: u8,
        offset: off_t,
        length: size_t,
        data: *mut ::std::os::raw::c_void,
    ) -> ssize_t;
    pub fn mifare_desfire_read_data_ex(
        tag: FreefareTag,
        file_no: u8,
        offset: off_t,
        length: size_t,
        data: *mut ::std::os::raw::c_void,
        cs: ::std::os::raw::c_int,
    ) -> ssize_t;
    pub fn mifare_desfire_write_data(
        tag: FreefareTag,
        file_no: u8,
        offset: off_t,
        length: size_t,
        data: *const ::std::os::raw::c_void,
    ) -> ssize_t;
    pub fn mifare_desfire_write_data_ex(
        tag: FreefareTag,
        file_no: u8,
        offset: off_t,
        length: size_t,
        data: *const ::std::os::raw::c_void,
        cs: ::std::os::raw::c_int,
    ) -> ssize_t;
    pub fn mifare_desfire_get_value(
        tag: FreefareTag,
        file_no: u8,
        value: *mut i32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_get_value_ex(
        tag: FreefareTag,
        file_no: u8,
        value: *mut i32,
        cs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_credit(
        tag: FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_credit_ex(
        tag: FreefareTag,
        file_no: u8,
        amount: i32,
        cs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_debit(
        tag: FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_debit_ex(
        tag: FreefareTag,
        file_no: u8,
        amount: i32,
        cs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_limited_credit(
        tag: FreefareTag,
        file_no: u8,
        amount: i32,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_limited_credit_ex(
        tag: FreefareTag,
        file_no: u8,
        amount: i32,
        cs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_write_record(
        tag: FreefareTag,
        file_no: u8,
        offset: off_t,
        length: size_t,
        data: *mut ::std::os::raw::c_void,
    ) -> ssize_t;
    pub fn mifare_desfire_write_record_ex(
        tag: FreefareTag,
        file_no: u8,
        offset: off_t,
        length: size_t,
        data: *mut ::std::os::raw::c_void,
        cs: ::std::os::raw::c_int,
    ) -> ssize_t;
    pub fn mifare_desfire_read_records(
        tag: FreefareTag,
        file_no: u8,
        offset: off_t,
        length: size_t,
        data: *mut ::std::os::raw::c_void,
    ) -> ssize_t;
    pub fn mifare_desfire_read_records_ex(
        tag: FreefareTag,
        file_no: u8,
        offset: off_t,
        length: size_t,
        data: *mut ::std::os::raw::c_void,
        cs: ::std::os::raw::c_int,
    ) -> ssize_t;
    pub fn mifare_desfire_clear_record_file(
        tag: FreefareTag,
        file_no: u8,
    ) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_commit_transaction(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_abort_transaction(tag: FreefareTag) -> ::std::os::raw::c_int;
    pub fn mifare_desfire_des_key_new(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_3des_key_new(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_des_key_new_with_version(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_3des_key_new_with_version(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_3k3des_key_new(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_3k3des_key_new_with_version(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_aes_key_new(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_aes_key_new_with_version(
        value: *mut u8,
        version: u8,
    ) -> MifareDESFireKey;
    pub fn mifare_desfire_key_get_version(key: MifareDESFireKey) -> u8;
    pub fn mifare_desfire_key_set_version(key: MifareDESFireKey, version: u8);
    pub fn mifare_desfire_key_free(key: MifareDESFireKey);
    pub fn tlv_encode(
        _type: u8,
        istream: *const u8,
        isize: u16,
        osize: *mut size_t,
    ) -> *mut u8;
    pub fn tlv_decode(
        istream: *const u8,
        _type: *mut u8,
        size: *mut u16,
    ) -> *mut u8;
    pub fn tlv_record_length(
        istream: *const u8,
        field_length_size: *mut size_t,
        field_value_size: *mut size_t,
    ) -> size_t;
    pub fn tlv_append(a: *mut u8, b: *mut u8) -> *mut u8;

    pub fn ntag21x_taste(device: *mut nfc_device, target: nfc_target) -> bool;

    pub fn ntag21x_tag_new(device: *mut nfc_device, target: nfc_target) -> FreefareTag;

    pub fn ntag21x_tag_reuse(tag: FreefareTag) -> FreefareTag;

    pub fn ntag21x_key_new(data: *const u8, pack: *const u8) -> NTAG21xKey;

    pub fn ntag21x_key_free(key: NTAG21xKey);

    pub fn ntag21x_tag_free(tag: FreefareTag);

    pub fn ntag21x_connect(tag: FreefareTag) -> ::std::os::raw::c_int;

    pub fn ntag21x_disconnect(tag: FreefareTag) -> ::std::os::raw::c_int;

    pub fn ntag21x_get_info(tag: FreefareTag) -> ::std::os::raw::c_int;

    pub fn ntag21x_get_subtype(tag: FreefareTag) -> ntag_tag_subtype;

    pub fn ntag21x_get_last_page(tag: FreefareTag) -> u8;

    pub fn ntag21x_read_signature(tag: FreefareTag, data: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_set_pwd(tag: FreefareTag, data: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_set_pack(tag: FreefareTag, data: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_set_key(tag: FreefareTag, key: NTAG21xKey) -> ::std::os::raw::c_int;

    pub fn ntag21x_set_auth(tag: FreefareTag, byte: u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_get_auth(tag: FreefareTag, byte: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_access_enable(tag: FreefareTag, byte: u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_access_disable(tag: FreefareTag, byte: u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_get_access(tag: FreefareTag, byte: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_check_access(
        tag: FreefareTag,
        byte: u8,
        result: *mut bool,
    ) -> ::std::os::raw::c_int;

    pub fn ntag21x_get_authentication_limit(
        tag: FreefareTag,
        byte: *mut u8,
    ) -> ::std::os::raw::c_int;

    pub fn ntag21x_set_authentication_limit(tag: FreefareTag, byte: u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_read(tag: FreefareTag, page: u8, data: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_read4(tag: FreefareTag, page: u8, data: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_fast_read(
        tag: FreefareTag,
        start_page: u8,
        end_page: u8,
        data: *mut u8,
    ) -> ::std::os::raw::c_int;

    pub fn ntag21x_fast_read4(tag: FreefareTag, page: u8, data: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_read_cnt(tag: FreefareTag, data: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_write(tag: FreefareTag, page: u8, data: *mut u8) -> ::std::os::raw::c_int;

    pub fn ntag21x_compatibility_write(
        tag: FreefareTag,
        page: u8,
        data: *mut u8,
    ) -> ::std::os::raw::c_int;

    pub fn ntag21x_authenticate(tag: FreefareTag, key: NTAG21xKey) -> ::std::os::raw::c_int;

    pub fn is_ntag21x(tag: FreefareTag) -> bool;

    pub fn ntag21x_is_auth_supported(device: *mut nfc_device, nai: nfc_iso14443a_info) -> bool;

}
