use crate::QueryBuilder;
use bevy_reflect::TypePath;
use std::marker::PhantomData;

pub trait QueryFilter: Sized {
    fn filter(builder: &mut QueryBuilder);
}

impl QueryFilter for () {
    fn filter(builder: &mut QueryBuilder) {
        let _ = builder;
    }
}

pub struct With<T>(PhantomData<T>);

impl<T: TypePath + 'static> QueryFilter for With<T> {
    fn filter(builder: &mut QueryBuilder) {
        builder.with::<T>();
    }
}

pub struct Without<T>(PhantomData<T>);

impl<T: TypePath + 'static> QueryFilter for Without<T> {
    fn filter(builder: &mut QueryBuilder) {
        builder.without::<T>();
    }
}

macro_rules! impl_query_filter_tuple {
    ($($items:ident),+) => {
        impl<$($items: QueryFilter),+> QueryFilter for ($($items),+) {
            fn filter(builder: &mut QueryBuilder) {
                $($items::filter(builder);)+
            }
        }
    };
}

impl_query_filter_tuple!(A, B);
impl_query_filter_tuple!(A, B, C);
impl_query_filter_tuple!(A, B, C, D);
impl_query_filter_tuple!(A, B, C, D, E);
impl_query_filter_tuple!(A, B, C, D, E, F);
impl_query_filter_tuple!(A, B, C, D, E, F, G);
impl_query_filter_tuple!(A, B, C, D, E, F, G, H);
