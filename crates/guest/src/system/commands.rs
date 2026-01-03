use super::{ParamBuilder, ParamCursor, SharedEvent, SystemParam};
use crate::world::{Bundle, World};
use bevy_ecs::entity::Entity;
use bevy_mod_ffi_core::commands;
use bevy_mod_ffi_guest_sys;
use std::ptr;

pub trait Command<Out = ()>: Send + 'static {
    fn apply(self, world: &mut World) -> Out;
}

impl<F: FnOnce(&mut World) + Send + 'static> Command for F {
    fn apply(self, world: &mut World) {
        self(world);
    }
}

pub struct Commands<'w, 's> {
    ptr: *mut commands,
    _marker: std::marker::PhantomData<(&'w (), &'s ())>,
}

impl<'w, 's> Commands<'w, 's> {
    pub(crate) unsafe fn from_ptr(ptr: *mut commands) -> Self {
        Self {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn spawn<B: Bundle + Send + 'static>(&mut self, bundle: B) {
        self.push(move |world: &mut World| {
            world.spawn(bundle);
        });
    }

    pub fn trigger<E: SharedEvent + Send + 'static>(&mut self, event: E) {
        self.push(move |world: &mut World| {
            world.trigger(event);
        });
    }

    pub fn trigger_targets<E: SharedEvent + Send + 'static>(&mut self, event: E, entity: Entity) {
        self.push(move |world: &mut World| {
            world.trigger_targets(event, entity);
        });
    }

    pub fn push<C: Command>(&mut self, command: C) {
        let command_boxed: Box<dyn FnOnce(*mut bevy_mod_ffi_core::world)> =
            Box::new(move |w: *mut bevy_mod_ffi_core::world| {
                let mut world = unsafe { World::from_ptr(w) };
                command.apply(&mut world);
            });

        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::param::bevy_commands_push(
                self.ptr,
                ptr::null_mut(),
                Box::into_raw(Box::new(command_boxed)) as _,
                bevy_mod_ffi_guest_sys::system::param::bevy_guest_run_command,
            )
        };

        assert!(success, "Failed to push command");
    }
}

unsafe impl SystemParam for Commands<'_, '_> {
    type State = ();
    type Item<'w, 's> = Commands<'w, 's>;

    fn build(_world: &mut World, builder: &mut ParamBuilder) {
        builder.add_commands();
    }

    unsafe fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        cursor: &mut ParamCursor<'_>,
    ) -> Self::Item<'w, 's> {
        let dyn_param_ptr = cursor.next().unwrap();
        let mut commands_ptr: *mut commands = ptr::null_mut();
        let success = unsafe {
            bevy_mod_ffi_guest_sys::system::param::bevy_dyn_system_param_downcast_commands(
                dyn_param_ptr,
                &mut commands_ptr,
            )
        };
        if !success || commands_ptr.is_null() {
            panic!("Failed to downcast DynSystemParam to Commands");
        }
        unsafe { Commands::from_ptr(commands_ptr) }
    }
}
