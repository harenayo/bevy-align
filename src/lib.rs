//! A simple crate to align entities in [`bevy`].
//!
//! # Features
//!
//! feature | description
//! - | -
//! `dev` | Enable other features required by the examples.

use bevy::{
    app::{
        App,
        Plugin,
        Update,
    },
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::QueryEntityError,
        system::Query,
    },
    hierarchy::Parent,
    log::error,
    math::{
        bool::BVec3,
        f32::Vec3,
    },
    transform::components::{
        GlobalTransform,
        Transform,
    },
};

/// A component for aligning.
/// An entity with this component should have [`Transform`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub struct Aligning {
    /// A target.
    pub target: Entity,
    /// Whether or not each elements of [`Transform::translation`] is aligned.
    pub enabled: BVec3,
}

impl Default for Aligning {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            enabled: BVec3::default(),
        }
    }
}

/// A bundle for [`Aligning`].
#[derive(Clone, Copy, PartialEq, Debug, Bundle)]
pub struct AligningBundle {
    pub aligning: Aligning,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// Update [`Transform`] of entities with [`Aligning`].
pub fn align_entities(
    mut entities: Query<(&Aligning, &mut Transform, Option<&Parent>)>,
    global_transforms: Query<&GlobalTransform>,
) {
    enum Error {
        QueryTarget(QueryEntityError),
        QueryParent(QueryEntityError),
    }

    entities
        .iter_mut()
        .map(|(aligning, mut transform, parent)| -> Result<_, Error> {
            transform.translation = Vec3::select(
                aligning.enabled,
                global_transforms
                    .get(aligning.target)
                    .map_err(Error::QueryTarget)?
                    .reparented_to(match parent {
                        Option::Some(parent) => global_transforms
                            .get(parent.get())
                            .map_err(Error::QueryParent)?,
                        Option::None => &GlobalTransform::IDENTITY,
                    })
                    .translation,
                transform.translation,
            );

            Result::Ok(())
        })
        .filter_map(Result::err)
        .for_each(|error| match error {
            Error::QueryTarget(error) => error!("a target not found: {error}"),
            Error::QueryParent(error) => error!("a parent not found: {error}"),
        })
}

/// A plugin for [`Aligning`].
/// This plugin just add [`align_entities`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct AlignPlugin;

impl Plugin for AlignPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, align_entities);
    }
}
