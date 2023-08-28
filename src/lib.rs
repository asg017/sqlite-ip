use sqlite_loadable::{
    api::{self, ValueType},
    define_scalar_function,
    errors::{Error, Result},
    FunctionFlags,
};
use sqlite_loadable::{define_collation, prelude::*};
use std::{
    cmp::Ordering,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

pub fn ip_version(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(context, format!("v{}", env!("CARGO_PKG_VERSION")))?;
    Ok(())
}

pub fn ip_debug(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    api::result_text(
        context,
        format!(
            "Version: v{}
    Source: {}
    ",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH")
        ),
    )?;
    Ok(())
}

fn ip_collate(a: &[u8], b: &[u8]) -> i32 {
    let a = std::str::from_utf8(a);
    let b = std::str::from_utf8(b);
    let (a, b) = match (a, b) {
        (Ok(a), Ok(b)) => (a, b),
        _ => return -1,
    };
    let a = IpAddr::from_str(a);
    let b = IpAddr::from_str(b);
    let (a, b) = match (a, b) {
        (Ok(a), Ok(b)) => (a, b),
        _ => return -1,
    };
    match a.cmp(&b) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

fn value_ipaddr(value: &*mut sqlite3_value) -> Result<IpAddr> {
    match api::value_type(value) {
        ValueType::Text => {
            let source = api::value_text(value)
                .map_err(|_| Error::new_message("Exepcted UTF8 in IpAddress"))?;
            let ip = IpAddr::from_str(source)
                .map_err(|e| Error::new_message(format!("Error parsing IP address: {}", e)))?;
            Ok(ip)
        }
        _ => Err(Error::new_message("TODO")),
    }
}
fn value_ipv4addr(value: &*mut sqlite3_value) -> Result<Ipv4Addr> {
    match api::value_type(value) {
        ValueType::Text => {
            let source = api::value_text(value)
                .map_err(|_| Error::new_message("Exepcted UTF8 in IpAddress"))?;
            let ip = Ipv4Addr::from_str(source)
                .map_err(|e| Error::new_message(format!("Error parsing IP address: {}", e)))?;
            Ok(ip)
        }
        _ => Err(Error::new_message("TODO")),
    }
}
fn value_ipv6addr(value: &*mut sqlite3_value) -> Result<Ipv6Addr> {
    match api::value_type(value) {
        ValueType::Text => {
            let source = api::value_text(value)
                .map_err(|_| Error::new_message("Exepcted UTF8 in IpAddress"))?;
            let ip = Ipv6Addr::from_str(source)
                .map_err(|e| Error::new_message(format!("Error parsing IP address: {}", e)))?;
            Ok(ip)
        }
        _ => Err(Error::new_message("TODO")),
    }
}

pub fn ip_valid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ipaddr = value_ipaddr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    );
    api::result_bool(context, ipaddr.is_ok());
    Ok(())
}
pub fn ipv4_valid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ipaddr = value_ipv4addr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    );
    api::result_bool(context, ipaddr.is_ok());
    Ok(())
}
pub fn ipv6_valid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ipaddr = value_ipv6addr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    );
    api::result_bool(context, ipaddr.is_ok());
    Ok(())
}
pub fn ip_is_unspecified(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let ipaddr = value_ipaddr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    )?;
    api::result_bool(context, ipaddr.is_unspecified());
    Ok(())
}
pub fn ip_is_loopback(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ipaddr = value_ipaddr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    )?;
    api::result_bool(context, ipaddr.is_loopback());
    Ok(())
}
pub fn ip_is_multicast(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ipaddr = value_ipaddr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    )?;
    api::result_bool(context, ipaddr.is_multicast());
    Ok(())
}
pub fn ip_is_ipv4(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ipaddr = value_ipaddr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    )?;
    api::result_bool(context, ipaddr.is_ipv4());
    Ok(())
}
pub fn ip_is_ipv6(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ipaddr = value_ipaddr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    )?;
    api::result_bool(context, ipaddr.is_ipv6());
    Ok(())
}
/*
pub fn ip_(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let ipaddr = value_ipaddr(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected 1st argument as IP address"))?,
    )?;
    api::result_null(context);
    Ok(())
} */

#[sqlite_entrypoint]
pub fn sqlite3_ip_init(db: *mut sqlite3) -> Result<()> {
    let flags = FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC;

    define_scalar_function(db, "ip_version", 0, ip_version, flags)?;
    define_scalar_function(db, "ip_debug", 0, ip_debug, flags)?;

    define_scalar_function(db, "ip_valid", 1, ip_valid, flags)?;
    define_scalar_function(db, "ip_is_unspecified", 1, ip_is_unspecified, flags)?;
    define_scalar_function(db, "ip_is_loopback", 1, ip_is_loopback, flags)?;
    define_scalar_function(db, "ip_is_multicast", 1, ip_is_multicast, flags)?;
    define_scalar_function(db, "ip_is_ipv4", 1, ip_is_ipv4, flags)?;
    define_scalar_function(db, "ip_is_ipv6", 1, ip_is_ipv6, flags)?;

    define_scalar_function(db, "ipv4_valid", 1, ipv4_valid, flags)?;
    define_scalar_function(db, "ipv6_valid", 1, ipv6_valid, flags)?;

    define_collation(db, "ip", ip_collate)?;
    Ok(())
}
