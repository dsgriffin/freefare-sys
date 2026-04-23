#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use core::ffi::{c_char, c_int, c_void};
use libc::off_t;
use nfc_sys::{nfc_device, nfc_iso14443a_info};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FreefareTagType {
    ULTRALIGHT = 0,
    ULTRALIGHT_C = 1,
    CLASSIC_1K = 2,
    CLASSIC_4K = 3,
    DESFIRE = 4,
}

#[repr(C)]
pub struct mifare_tag {
    _private: [u8; 0],
}

pub type MifareTag = *mut mifare_tag;
pub type FreefareTag = MifareTag;

#[repr(C)]
pub struct mifare_desfire_key {
    _private: [u8; 0],
}

pub type MifareDESFireKey = *mut mifare_desfire_key;

pub type MifareUltralightPageNumber = u8;
pub type MifareUltralightPage = [u8; 4];
pub type MifareClassicBlock = [u8; 16];
pub type MifareClassicSectorNumber = u8;
pub type MifareClassicBlockNumber = u8;
pub type MifareClassicKey = [u8; 6];

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MifareClassicKeyType {
    MFC_KEY_A = 0,
    MFC_KEY_B = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MadAid {
    pub application_code: u8,
    pub function_cluster_code: u8,
}

#[repr(C)]
pub struct mad {
    _private: [u8; 0],
}

pub type Mad = *mut mad;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MifareDesfireFileType {
    MDFT_STANDARD_DATA_FILE = 0x00,
    MDFT_BACKUP_DATA_FILE = 0x01,
    MDFT_VALUE_FILE_WITH_BACKUP = 0x02,
    MDFT_LINEAR_RECORD_FILE_WITH_BACKUP = 0x03,
    MDFT_CYCLIC_RECORD_FILE_WITH_BACKUP = 0x04,
}

#[repr(C)]
pub struct mifare_desfire_aid {
    _private: [u8; 0],
}

pub type MifareDESFireAID = *mut mifare_desfire_aid;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MifareDESFireDF {
    pub aid: u32,
    pub fid: u16,
    pub df_name: [u8; 16],
    pub df_name_len: usize,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MifareDESFireVersionComponent {
    pub vendor_id: u8,
    pub type_: u8,
    pub subtype: u8,
    pub version_major: u8,
    pub version_minor: u8,
    pub storage_size: u8,
    pub protocol: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MifareDESFireVersionInfo {
    pub hardware: MifareDESFireVersionComponent,
    pub software: MifareDESFireVersionComponent,
    pub uid: [u8; 7],
    pub batch_number: [u8; 5],
    pub production_week: u8,
    pub production_year: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MifareDESFireFileSettingsData {
    pub standard_file: MifareDESFireStandardFileSettings,
    pub value_file: MifareDESFireValueFileSettings,
    pub linear_record_file: MifareDESFireRecordFileSettings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MifareDESFireFileSettings {
    pub file_type: u8,
    pub communication_settings: u8,
    pub access_rights: u16,
    pub settings: MifareDESFireFileSettingsData,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MifareDESFireStandardFileSettings {
    pub file_size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MifareDESFireValueFileSettings {
    pub lower_limit: i32,
    pub upper_limit: i32,
    pub limited_credit_value: i32,
    pub limited_credit_enabled: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MifareDESFireRecordFileSettings {
    pub record_size: u32,
    pub max_number_of_records: u32,
    pub current_number_of_records: u32,
}

pub const C_000: u32 = 0;
pub const C_001: u32 = 1;
pub const C_010: u32 = 2;
pub const C_011: u32 = 3;
pub const C_100: u32 = 4;
pub const C_101: u32 = 5;
pub const C_110: u32 = 6;
pub const C_111: u32 = 7;
pub const C_DEFAULT: u32 = 255;

pub const MCAB_R: u32 = 0x8;
pub const MCAB_W: u32 = 0x4;
pub const MCAB_D: u32 = 0x2;
pub const MCAB_I: u32 = 0x1;

pub const MCAB_READ_KEYA: u32 = 0x400;
pub const MCAB_WRITE_KEYA: u32 = 0x100;
pub const MCAB_READ_ACCESS_BITS: u32 = 0x040;
pub const MCAB_WRITE_ACCESS_BITS: u32 = 0x010;
pub const MCAB_READ_KEYB: u32 = 0x004;
pub const MCAB_WRITE_KEYB: u32 = 0x001;

pub const MDCM_PLAIN: u32 = 0x00;
pub const MDCM_MACED: u32 = 0x01;
pub const MDCM_ENCIPHERED: u32 = 0x03;

pub const APPLICATION_CRYPTO_DES: u32 = 0x00;
pub const APPLICATION_CRYPTO_3K3DES: u32 = 0x40;
pub const APPLICATION_CRYPTO_AES: u32 = 0x80;

pub const MDAR_KEY0: u16 = 0x0;
pub const MDAR_KEY1: u16 = 0x1;
pub const MDAR_KEY2: u16 = 0x2;
pub const MDAR_KEY3: u16 = 0x3;
pub const MDAR_KEY4: u16 = 0x4;
pub const MDAR_KEY5: u16 = 0x5;
pub const MDAR_KEY6: u16 = 0x6;
pub const MDAR_KEY7: u16 = 0x7;
pub const MDAR_KEY8: u16 = 0x8;
pub const MDAR_KEY9: u16 = 0x9;
pub const MDAR_KEY10: u16 = 0xa;
pub const MDAR_KEY11: u16 = 0xb;
pub const MDAR_KEY12: u16 = 0xc;
pub const MDAR_KEY13: u16 = 0xd;
pub const MDAR_FREE: u16 = 0xE;
pub const MDAR_DENY: u16 = 0xF;

pub const OPERATION_OK: u8 = 0x00;
pub const NO_CHANGES: u8 = 0x0C;
pub const OUT_OF_EEPROM_ERROR: u8 = 0x0E;
pub const ILLEGAL_COMMAND_CODE: u8 = 0x1C;
pub const INTEGRITY_ERROR: u8 = 0x1E;
pub const NO_SUCH_KEY: u8 = 0x40;
pub const LENGTH_ERROR: u8 = 0x7E;
pub const PERMISSION_ERROR: u8 = 0x9D;
pub const PARAMETER_ERROR: u8 = 0x9E;
pub const APPLICATION_NOT_FOUND: u8 = 0xA0;
pub const APPL_INTEGRITY_ERROR: u8 = 0xA1;
pub const AUTHENTICATION_ERROR: u8 = 0xAE;
pub const ADDITIONAL_FRAME: u8 = 0xAF;
pub const BOUNDARY_ERROR: u8 = 0xBE;
pub const PICC_INTEGRITY_ERROR: u8 = 0xC1;
pub const COMMAND_ABORTED: u8 = 0xCA;
pub const PICC_DISABLED_ERROR: u8 = 0xCD;
pub const COUNT_ERROR: u8 = 0xCE;
pub const DUPLICATE_ERROR: u8 = 0xDE;
pub const EEPROM_ERROR: u8 = 0xEE;
pub const FILE_NOT_FOUND: u8 = 0xF0;
pub const FILE_INTEGRITY_ERROR: u8 = 0xF1;
pub const CRYPTO_ERROR: u8 = 0x01;

pub const fn mdar(read: u16, write: u16, read_write: u16, change_access_rights: u16) -> u16 {
    (read << 12) | (write << 8) | (read_write << 4) | change_access_rights
}

pub const fn mdar_read(access_rights: u16) -> u16 {
    (access_rights >> 12) & 0x0f
}

pub const fn mdar_write(access_rights: u16) -> u16 {
    (access_rights >> 8) & 0x0f
}

pub const fn mdar_read_write(access_rights: u16) -> u16 {
    (access_rights >> 4) & 0x0f
}

pub const fn mdar_change_ar(access_rights: u16) -> u16 {
    access_rights & 0x0f
}

unsafe extern "C" {
    pub static mifare_classic_nfcforum_public_key_a: MifareClassicKey;
    pub static mad_public_key_a: MifareClassicKey;
    pub static mad_free_aid: MadAid;
    pub static mad_defect_aid: MadAid;
    pub static mad_reserved_aid: MadAid;
    pub static mad_card_holder_aid: MadAid;
    pub static mad_not_applicable_aid: MadAid;
    pub static mad_nfcforum_aid: MadAid;

    pub fn freefare_get_tags(device: *mut nfc_device) -> *mut MifareTag;
    pub fn freefare_tag_new(device: *mut nfc_device, nai: nfc_iso14443a_info) -> MifareTag;
    pub fn freefare_get_tag_type(tag: MifareTag) -> FreefareTagType;
    pub fn freefare_get_tag_friendly_name(tag: MifareTag) -> *const c_char;
    pub fn freefare_get_tag_uid(tag: MifareTag) -> *mut c_char;
    pub fn freefare_free_tag(tag: MifareTag);
    pub fn freefare_free_tags(tags: *mut MifareTag);

    pub fn freefare_strerror(tag: MifareTag) -> *const c_char;
    pub fn freefare_strerror_r(tag: MifareTag, buffer: *mut c_char, len: usize) -> c_int;
    pub fn freefare_perror(tag: MifareTag, string: *const c_char);

    pub fn mifare_ultralight_connect(tag: MifareTag) -> c_int;
    pub fn mifare_ultralight_disconnect(tag: MifareTag) -> c_int;
    pub fn mifare_ultralight_read(
        tag: MifareTag,
        page: MifareUltralightPageNumber,
        data: *mut MifareUltralightPage,
    ) -> c_int;
    pub fn mifare_ultralight_write(
        tag: MifareTag,
        page: MifareUltralightPageNumber,
        data: *const MifareUltralightPage,
    ) -> c_int;
    pub fn mifare_ultralightc_authenticate(tag: MifareTag, key: MifareDESFireKey) -> c_int;
    pub fn is_mifare_ultralightc_on_reader(
        device: *mut nfc_device,
        nai: nfc_iso14443a_info,
    ) -> bool;

    pub fn mifare_classic_connect(tag: MifareTag) -> c_int;
    pub fn mifare_classic_disconnect(tag: MifareTag) -> c_int;
    pub fn mifare_classic_authenticate(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        key: *const MifareClassicKey,
        key_type: MifareClassicKeyType,
    ) -> c_int;
    pub fn mifare_classic_read(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        data: *mut MifareClassicBlock,
    ) -> c_int;
    pub fn mifare_classic_init_value(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        value: i32,
        adr: MifareClassicBlockNumber,
    ) -> c_int;
    pub fn mifare_classic_read_value(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        value: *mut i32,
        adr: *mut MifareClassicBlockNumber,
    ) -> c_int;
    pub fn mifare_classic_write(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        data: *const MifareClassicBlock,
    ) -> c_int;
    pub fn mifare_classic_increment(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        amount: u32,
    ) -> c_int;
    pub fn mifare_classic_decrement(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        amount: u32,
    ) -> c_int;
    pub fn mifare_classic_restore(tag: MifareTag, block: MifareClassicBlockNumber) -> c_int;
    pub fn mifare_classic_transfer(tag: MifareTag, block: MifareClassicBlockNumber) -> c_int;
    pub fn mifare_classic_get_trailer_block_permission(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        permission: u16,
        key_type: MifareClassicKeyType,
    ) -> c_int;
    pub fn mifare_classic_get_data_block_permission(
        tag: MifareTag,
        block: MifareClassicBlockNumber,
        permission: u8,
        key_type: MifareClassicKeyType,
    ) -> c_int;
    pub fn mifare_classic_format_sector(tag: MifareTag, sector: MifareClassicSectorNumber)
        -> c_int;
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
    pub fn mifare_classic_sector_block_count(sector: MifareClassicSectorNumber) -> usize;
    pub fn mifare_classic_sector_last_block(
        sector: MifareClassicSectorNumber,
    ) -> MifareClassicBlockNumber;

    pub fn mad_new(version: u8) -> Mad;
    pub fn mad_read(tag: MifareTag) -> Mad;
    pub fn mad_write(
        tag: MifareTag,
        mad: Mad,
        key_b_sector_00: *const MifareClassicKey,
        key_b_sector_10: *const MifareClassicKey,
    ) -> c_int;
    pub fn mad_get_version(mad: Mad) -> c_int;
    pub fn mad_set_version(mad: Mad, version: u8);
    pub fn mad_get_card_publisher_sector(mad: Mad) -> MifareClassicSectorNumber;
    pub fn mad_set_card_publisher_sector(mad: Mad, cps: MifareClassicSectorNumber) -> c_int;
    pub fn mad_get_aid(mad: Mad, sector: MifareClassicSectorNumber, aid: *mut MadAid) -> c_int;
    pub fn mad_set_aid(mad: Mad, sector: MifareClassicSectorNumber, aid: MadAid) -> c_int;
    pub fn mad_sector_reserved(sector: MifareClassicSectorNumber) -> bool;
    pub fn mad_free(mad: Mad);

    pub fn mifare_application_alloc(
        mad: Mad,
        aid: MadAid,
        size: usize,
    ) -> *mut MifareClassicSectorNumber;
    pub fn mifare_application_read(
        tag: MifareTag,
        mad: Mad,
        aid: MadAid,
        buf: *mut c_void,
        nbytes: usize,
        key: *const MifareClassicKey,
        key_type: MifareClassicKeyType,
    ) -> isize;
    pub fn mifare_application_write(
        tag: MifareTag,
        mad: Mad,
        aid: MadAid,
        buf: *const c_void,
        nbytes: usize,
        key: *const MifareClassicKey,
        key_type: MifareClassicKeyType,
    ) -> isize;
    pub fn mifare_application_free(mad: Mad, aid: MadAid);
    pub fn mifare_application_find(mad: Mad, aid: MadAid) -> *mut MifareClassicSectorNumber;

    pub fn mifare_desfire_aid_new(aid: u32) -> MifareDESFireAID;
    pub fn mifare_desfire_aid_new_with_mad_aid(mad_aid: MadAid, n: u8) -> MifareDESFireAID;
    pub fn mifare_desfire_aid_get_aid(aid: MifareDESFireAID) -> u32;
    pub fn mifare_desfire_last_pcd_error(tag: MifareTag) -> u8;
    pub fn mifare_desfire_last_picc_error(tag: MifareTag) -> u8;
    pub fn mifare_desfire_connect(tag: MifareTag) -> c_int;
    pub fn mifare_desfire_disconnect(tag: MifareTag) -> c_int;
    pub fn mifare_desfire_authenticate(tag: MifareTag, key_no: u8, key: MifareDESFireKey) -> c_int;
    pub fn mifare_desfire_authenticate_iso(
        tag: MifareTag,
        key_no: u8,
        key: MifareDESFireKey,
    ) -> c_int;
    pub fn mifare_desfire_authenticate_aes(
        tag: MifareTag,
        key_no: u8,
        key: MifareDESFireKey,
    ) -> c_int;
    pub fn mifare_desfire_change_key_settings(tag: MifareTag, settings: u8) -> c_int;
    pub fn mifare_desfire_get_key_settings(
        tag: MifareTag,
        settings: *mut u8,
        max_keys: *mut u8,
    ) -> c_int;
    pub fn mifare_desfire_change_key(
        tag: MifareTag,
        key_no: u8,
        new_key: MifareDESFireKey,
        old_key: MifareDESFireKey,
    ) -> c_int;
    pub fn mifare_desfire_get_key_version(tag: MifareTag, key_no: u8, version: *mut u8) -> c_int;
    pub fn mifare_desfire_create_application(
        tag: MifareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> c_int;
    pub fn mifare_desfire_create_application_3k3des(
        tag: MifareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> c_int;
    pub fn mifare_desfire_create_application_aes(
        tag: MifareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
    ) -> c_int;
    pub fn mifare_desfire_create_application_iso(
        tag: MifareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: c_int,
        iso_file_id: u16,
        iso_file_name: *mut u8,
        iso_file_name_len: usize,
    ) -> c_int;
    pub fn mifare_desfire_create_application_3k3des_iso(
        tag: MifareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: c_int,
        iso_file_id: u16,
        iso_file_name: *mut u8,
        iso_file_name_len: usize,
    ) -> c_int;
    pub fn mifare_desfire_create_application_aes_iso(
        tag: MifareTag,
        aid: MifareDESFireAID,
        settings: u8,
        key_no: u8,
        want_iso_file_identifiers: c_int,
        iso_file_id: u16,
        iso_file_name: *mut u8,
        iso_file_name_len: usize,
    ) -> c_int;
    pub fn mifare_desfire_delete_application(tag: MifareTag, aid: MifareDESFireAID) -> c_int;
    pub fn mifare_desfire_get_application_ids(
        tag: MifareTag,
        aids: *mut *mut MifareDESFireAID,
        count: *mut usize,
    ) -> c_int;
    pub fn mifare_desfire_get_df_names(
        tag: MifareTag,
        dfs: *mut *mut MifareDESFireDF,
        count: *mut usize,
    ) -> c_int;
    pub fn mifare_desfire_free_application_ids(aids: *mut MifareDESFireAID);
    pub fn mifare_desfire_select_application(tag: MifareTag, aid: MifareDESFireAID) -> c_int;
    pub fn mifare_desfire_format_picc(tag: MifareTag) -> c_int;
    pub fn mifare_desfire_get_version(
        tag: MifareTag,
        version_info: *mut MifareDESFireVersionInfo,
    ) -> c_int;
    pub fn mifare_desfire_free_mem(tag: MifareTag, size: *mut u32) -> c_int;
    pub fn mifare_desfire_set_configuration(
        tag: MifareTag,
        disable_format: bool,
        enable_random_uid: bool,
    ) -> c_int;
    pub fn mifare_desfire_set_default_key(tag: MifareTag, key: MifareDESFireKey) -> c_int;
    pub fn mifare_desfire_set_ats(tag: MifareTag, ats: *mut u8) -> c_int;
    pub fn mifare_desfire_get_card_uid(tag: MifareTag, uid: *mut *mut c_char) -> c_int;
    pub fn mifare_desfire_get_file_ids(
        tag: MifareTag,
        files: *mut *mut u8,
        count: *mut usize,
    ) -> c_int;
    pub fn mifare_desfire_get_iso_file_ids(
        tag: MifareTag,
        files: *mut *mut u16,
        count: *mut usize,
    ) -> c_int;
    pub fn mifare_desfire_get_file_settings(
        tag: MifareTag,
        file_no: u8,
        settings: *mut MifareDESFireFileSettings,
    ) -> c_int;
    pub fn mifare_desfire_change_file_settings(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
    ) -> c_int;
    pub fn mifare_desfire_create_std_data_file(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
    ) -> c_int;
    pub fn mifare_desfire_create_std_data_file_iso(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
        iso_file_id: u16,
    ) -> c_int;
    pub fn mifare_desfire_create_backup_data_file(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
    ) -> c_int;
    pub fn mifare_desfire_create_backup_data_file_iso(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        file_size: u32,
        iso_file_id: u16,
    ) -> c_int;
    pub fn mifare_desfire_create_value_file(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        lower_limit: i32,
        upper_limit: i32,
        value: i32,
        limited_credit_enable: u8,
    ) -> c_int;
    pub fn mifare_desfire_create_linear_record_file(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
    ) -> c_int;
    pub fn mifare_desfire_create_linear_record_file_iso(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
        iso_file_id: u16,
    ) -> c_int;
    pub fn mifare_desfire_create_cyclic_record_file(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
    ) -> c_int;
    pub fn mifare_desfire_create_cyclic_record_file_iso(
        tag: MifareTag,
        file_no: u8,
        communication_settings: u8,
        access_rights: u16,
        record_size: u32,
        max_number_of_records: u32,
        iso_file_id: u16,
    ) -> c_int;
    pub fn mifare_desfire_delete_file(tag: MifareTag, file_no: u8) -> c_int;
    pub fn mifare_desfire_read_data(
        tag: MifareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: *mut c_void,
    ) -> isize;
    pub fn mifare_desfire_read_data_ex(
        tag: MifareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: *mut c_void,
        cs: c_int,
    ) -> isize;
    pub fn mifare_desfire_write_data(
        tag: MifareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: *const c_void,
    ) -> isize;
    pub fn mifare_desfire_write_data_ex(
        tag: MifareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: *const c_void,
        cs: c_int,
    ) -> isize;
    pub fn mifare_desfire_get_value(tag: MifareTag, file_no: u8, value: *mut i32) -> c_int;
    pub fn mifare_desfire_get_value_ex(
        tag: MifareTag,
        file_no: u8,
        value: *mut i32,
        cs: c_int,
    ) -> c_int;
    pub fn mifare_desfire_credit(tag: MifareTag, file_no: u8, amount: i32) -> c_int;
    pub fn mifare_desfire_credit_ex(tag: MifareTag, file_no: u8, amount: i32, cs: c_int) -> c_int;
    pub fn mifare_desfire_debit(tag: MifareTag, file_no: u8, amount: i32) -> c_int;
    pub fn mifare_desfire_debit_ex(tag: MifareTag, file_no: u8, amount: i32, cs: c_int) -> c_int;
    pub fn mifare_desfire_limited_credit(tag: MifareTag, file_no: u8, amount: i32) -> c_int;
    pub fn mifare_desfire_limited_credit_ex(
        tag: MifareTag,
        file_no: u8,
        amount: i32,
        cs: c_int,
    ) -> c_int;
    pub fn mifare_desfire_write_record(
        tag: MifareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: *mut c_void,
    ) -> isize;
    pub fn mifare_desfire_write_record_ex(
        tag: MifareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: *mut c_void,
        cs: c_int,
    ) -> isize;
    pub fn mifare_desfire_read_records(
        tag: MifareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: *mut c_void,
    ) -> isize;
    pub fn mifare_desfire_read_records_ex(
        tag: MifareTag,
        file_no: u8,
        offset: off_t,
        length: usize,
        data: *mut c_void,
        cs: c_int,
    ) -> isize;
    pub fn mifare_desfire_clear_record_file(tag: MifareTag, file_no: u8) -> c_int;
    pub fn mifare_desfire_commit_transaction(tag: MifareTag) -> c_int;
    pub fn mifare_desfire_abort_transaction(tag: MifareTag) -> c_int;
    pub fn mifare_desfire_des_key_new(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_3des_key_new(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_des_key_new_with_version(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_3des_key_new_with_version(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_3k3des_key_new(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_3k3des_key_new_with_version(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_aes_key_new(value: *mut u8) -> MifareDESFireKey;
    pub fn mifare_desfire_aes_key_new_with_version(value: *mut u8, version: u8)
        -> MifareDESFireKey;
    pub fn mifare_desfire_key_get_version(key: MifareDESFireKey) -> u8;
    pub fn mifare_desfire_key_set_version(key: MifareDESFireKey, version: u8);
    pub fn mifare_desfire_key_free(key: MifareDESFireKey);

    pub fn tlv_encode(_type: u8, istream: *const u8, isize: u16, osize: *mut usize) -> *mut u8;
    pub fn tlv_decode(istream: *const u8, _type: *mut u8, size: *mut u16) -> *mut u8;
    pub fn tlv_record_length(
        istream: *const u8,
        field_length_size: *mut usize,
        field_value_size: *mut usize,
    ) -> usize;
    pub fn tlv_append(a: *mut u8, b: *mut u8) -> *mut u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_and_layout_smoke_tests() {
        assert_eq!(core::mem::size_of::<MadAid>(), 2);
        assert_eq!(core::mem::size_of::<MifareDESFireVersionComponent>(), 7);
        assert_eq!(core::mem::size_of::<MifareDESFireVersionInfo>(), 28);
        assert_eq!(core::mem::size_of::<MifareDESFireFileSettings>(), 20);
    }

    #[test]
    fn access_right_helpers_match_header_macros() {
        let access_rights = mdar(MDAR_KEY1, MDAR_KEY2, MDAR_FREE, MDAR_DENY);
        assert_eq!(mdar_read(access_rights), MDAR_KEY1);
        assert_eq!(mdar_write(access_rights), MDAR_KEY2);
        assert_eq!(mdar_read_write(access_rights), MDAR_FREE);
        assert_eq!(mdar_change_ar(access_rights), MDAR_DENY);
    }

    #[test]
    fn links_against_system_libraries() {
        let version = unsafe { nfc_sys::nfc_version() };
        assert!(!version.is_null());
    }
}
