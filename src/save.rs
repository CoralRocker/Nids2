//! # Save
//! `save` defines the `Saveable<T>` trait, which requires types to be able to be converted
//! to big-endian bytes. The object can then be read to type `T` from bytes

use std::cell::RefCell;
use std::convert::TryInto;
use std::error;
use std::ops::Deref;
use std::rc::Rc;

use raylib::color;
use raylib::prelude::*;

/// Holds a read object T, and the amount of bytes read for that object.
/// Used to track how many bytes of a bytearray have been read.
pub struct SaveInfo<T>(pub T, pub usize);

/// Trait converting from a type to a byte array so that
/// it can be written to a file on the disk. Prefer big-endian
/// endianess for consistency.
/// from_bytes can return a different type than `Self`. That type is `T`.
/// This is used for certain types, such as str and Ref which cannot themselves
/// be created and returned, but which can be saved to bytes.
///
/// # Example
/// ```
/// impl Saveable<str> for str { /*...*/ } // wouldn't work because `from_bytes` cannot return str as it has no definite size.
/// impl Saveable<String> for str { /*...*/ } // would work because `String` can be returned.
/// ```
pub trait Saveable<T>
where
    T: Saveable<T>,
{
    /// To big-endian byte vector
    fn to_bytes(&self) -> Vec<u8>;
    /// From big-endian byte vector to type `T`. `T` is not necessarily the same as `Self`.
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<T>, Box<dyn error::Error>>;
}

/* Primitive Data Types */
impl Saveable<Self> for f32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        Ok(SaveInfo(f32::from_be_bytes(bytes[0..4].try_into()?), 4))
    }
}

impl Saveable<Self> for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        Ok(SaveInfo(bytes[0], 1))
    }
}

impl Saveable<Self> for i8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        Ok(SaveInfo(bytes[0] as i8, 1))
    }
}

impl Saveable<Self> for i16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        Ok(SaveInfo(i16::from_be_bytes(bytes[0..2].try_into()?), 2))
    }
}

impl Saveable<Self> for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        Ok(SaveInfo(i32::from_be_bytes(bytes[0..4].try_into()?), 4))
    }
}

impl Saveable<Self> for usize {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        Ok(SaveInfo(usize::from_be_bytes(bytes[0..8].try_into()?), 8))
    }
}

impl Saveable<Self> for bool {
    fn to_bytes(&self) -> Vec<u8> {
        (*self as i32).to_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let item = i32::from_bytes(bytes)?;
        Ok(SaveInfo(item.0 > 0, item.1))
    }
}

impl Saveable<Self> for String {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.len().to_bytes();
        result.extend(self.as_bytes().to_vec().iter());
        result
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let size = usize::from_bytes(bytes)?.0;
        Ok(SaveInfo(
            String::from_utf8(bytes[8..size + 8].to_vec())?,
            8 + size,
        ))
    }
}

impl Saveable<String> for str {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.len().to_bytes();
        result.extend(self.as_bytes().to_vec().iter());
        result
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<String>, Box<dyn error::Error>> {
        let size = usize::from_bytes(bytes)?.0;
        Ok(SaveInfo(
            String::from_utf8(bytes[8..size + 8].to_vec())?,
            8 + size,
        ))
    }
}

/* Raylib Objects */
impl Saveable<color::Color> for color::Color {
    fn to_bytes(&self) -> Vec<u8> {
        self.color_to_int().to_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<color::Color>, Box<dyn error::Error>> {
        Ok(SaveInfo(
            color::Color::get_color(i32::from_bytes(bytes)?.0),
            4,
        ))
    }
}

impl Saveable<Self> for Rectangle {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.x.to_bytes();
        result.extend(self.y.to_bytes().iter());
        result.extend(self.width.to_bytes().iter());
        result.extend(self.height.to_bytes().iter());
        result
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let x = f32::from_bytes(bytes)?;
        let y = f32::from_bytes(&bytes[4..])?;
        let width = f32::from_bytes(&bytes[8..])?;
        let height = f32::from_bytes(&bytes[12..])?;
        Ok(SaveInfo(rrect(x.0, y.0, width.0, height.0), 16))
    }
}

impl Saveable<Self> for Vector2 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.x.to_bytes();
        result.extend(self.y.to_bytes().iter());
        result
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let x = f32::from_bytes(bytes)?;
        let y = f32::from_bytes(&bytes[4..])?;
        Ok(SaveInfo(rvec2(x.0, y.0), 8))
    }
}

/* Collections */
impl<T> Saveable<Self> for Vec<T>
where
    T: Saveable<T>,
{
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.len().to_bytes().iter());
        for item in self.iter() {
            result.extend(item.to_bytes().iter());
        }
        result
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let mut result = Vec::<T>::new();
        let size = usize::from_bytes(bytes)?;
        let mut bytes_read = size.1;
        for _ in 0..size.0 {
            let item = T::from_bytes(&bytes[bytes_read..])?;
            bytes_read += item.1;
            result.push(item.0);
        }
        Ok(SaveInfo(result, bytes_read))
    }
}

impl<T> Saveable<Vec<T>> for [T]
where
    T: Saveable<T> + Clone,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.to_vec().to_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Vec<T>>, Box<dyn error::Error>> {
        Vec::<T>::from_bytes(bytes)
    }
}

/* Options */
impl<T> Saveable<Self> for Option<T>
where
    T: Saveable<T>,
{
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = self.is_some().to_bytes();
        if let Some(t) = self {
            result.extend(t.to_bytes().iter());
        }
        result
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let is_some = bool::from_bytes(bytes)?.0;
        if is_some {
            let res = T::from_bytes(&bytes[4..])?;
            Ok(SaveInfo(Some(res.0), res.1 + 4))
        } else {
            Ok(SaveInfo(None, 4))
        }
    }
}

/* Smart Pointers and Reference Types */
impl<'a, T> Saveable<T> for std::cell::Ref<'a, T>
where
    T: Saveable<T>,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.deref().to_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<T>, Box<dyn error::Error>> {
        let result = T::from_bytes(bytes)?;
        Ok(result)
    }
}

impl<T> Saveable<Self> for Rc<T>
where
    T: Saveable<T>,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.as_ref().to_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let result = T::from_bytes(bytes)?;
        Ok(SaveInfo(Rc::new(result.0), result.1))
    }
}

impl<T> Saveable<Self> for RefCell<T>
where
    T: Saveable<T>,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.borrow().to_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let result = T::from_bytes(bytes)?;
        Ok(SaveInfo(RefCell::new(result.0), result.1))
    }
}

impl<T> Saveable<Self> for Box<T>
where
    T: Saveable<T>,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.as_ref().to_bytes()
    }
    fn from_bytes(bytes: &[u8]) -> Result<SaveInfo<Self>, Box<dyn error::Error>> {
        let target = T::from_bytes(bytes)?;
        Ok(SaveInfo(Box::new(target.0), target.1))
    }
}
