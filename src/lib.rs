//! Helper [`SystemParam`] for when you have a resource containing a handle

#![warn(missing_docs)]

use std::ops::Deref;

use bevy::{ecs::system::SystemParam, prelude::*};

/// Implemented for resources that dereference into a handle
pub trait SingletonHandle: Resource + Deref<Target = Handle<Self::Asset>> {
    /// The asset type of the handle
    type Asset: Asset;
}

impl<A: Asset, T: Resource + Deref<Target = Handle<A>>> SingletonHandle for T {
    type Asset = A;
}

/// [`SystemParam`] that makes it easier to get an asset for which you have a singleton
#[derive(SystemParam)]
pub struct AssetSingleton<'w, T: SingletonHandle> {
    /// [`Assets`] resource for the asset type
    pub assets: Res<'w, Assets<<T as SingletonHandle>::Asset>>,
    /// Your resource
    pub handle: Res<'w, T>,
}

impl<T: SingletonHandle> AssetSingleton<'_, T>
where
    T::Asset: Asset + Sized,
{
    /// Get the asset
    pub fn get(&self) -> Option<&T::Asset> {
        self.assets.get(&**self.handle)
    }

    /// Get the asset and panic if it doesn't exist
    pub fn unwrap(&self) -> &T::Asset {
        self.get().unwrap()
    }
}

/// [`AssetSingleton`], except it can get the asset mutably
#[derive(SystemParam)]
pub struct AssetSingletonMut<'w, T: SingletonHandle> {
    /// [`Assets`] resource for the asset type
    pub assets: ResMut<'w, Assets<<T as SingletonHandle>::Asset>>,
    /// Your resource
    pub handle: Res<'w, T>,
}

impl<T: SingletonHandle> AssetSingletonMut<'_, T>
where
    T::Asset: Asset + Sized,
{
    /// Get the asset
    pub fn get(&self) -> Option<&T::Asset> {
        self.assets.get(&**self.handle)
    }

    /// Get the asset mutably
    pub fn get_mut(&mut self) -> Option<&mut T::Asset> {
        self.assets.get_mut(&**self.handle)
    }

    /// Get the asset and panic if it doesn't exist
    pub fn unwrap(&self) -> &T::Asset {
        self.get().unwrap()
    }

    /// Get the asset mutably and panic if it doesn't exist
    pub fn unwrap_mut(&mut self) -> &mut T::Asset {
        self.get_mut().unwrap()
    }
}
