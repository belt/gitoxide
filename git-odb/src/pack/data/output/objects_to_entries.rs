use crate::{pack, pack::data::output, FindExt};
use git_features::{hash, parallel, progress::Progress};
use git_hash::oid;

/// Write all `objects` into `out` without attempting to apply any delta compression.
/// This allows objects to be written rather immediately.
/// Objects are held in memory and compressed using DEFLATE, with those in-flight chunks of compressed
/// objects being sent to the current thread for writing. No buffering of these objects is performed,
/// allowing for natural back-pressure in case of slow writers.
///
/// * `objects`
///   * A unique list of objects to add to the pack. These must not be duplicated as no such validation is performed here.
/// * `progress`
///   * a way to obtain progress information
/// * `options`
///   * more configuration
///
/// _Returns_ the checksum of the pack
///
/// ## Discussion
///
/// ### Advantages
///
/// * Begins writing immediately and supports back-pressure.
/// * Abstract over object databases and how input is provided.
///
/// ### Disadvantages
///
/// * **does not yet support thin packs** as we don't have a way to determine which objects are supposed to be thin.
/// * ~~currently there is no way to easily write the pack index, even though the state here is uniquely positioned to do
///   so with minimal overhead (especially compared to `gixp index-from-pack`)~~ Probably works now by chaining Iterators
///  or keeping enough state to write a pack and then generate an index with recorded data.
///
pub fn objects_to_entries_iter<Locate, Iter, Oid, Cache>(
    db: Locate,
    make_cache: impl Fn() -> Cache + Send + Clone + Sync + 'static,
    objects: Iter,
    _progress: impl Progress,
    Options {
        version,
        thread_limit,
        input_object_expansion,
        chunk_size,
    }: Options,
) -> impl Iterator<Item = Result<Vec<output::Entry>, Error<Locate::Error>>>
       + parallel::reduce::Finalize<
    Reduce = parallel::reduce::IdentityWithResult<Vec<output::Entry>, Error<Locate::Error>>,
>
where
    Locate: crate::Find + Clone + Send + Sync + 'static,
    <Locate as crate::Find>::Error: Send,
    Iter: Iterator<Item = Oid> + Send + 'static,
    Oid: AsRef<oid> + Send + 'static,
    Cache: pack::cache::DecodeEntry,
{
    assert!(
        matches!(version, pack::data::Version::V2),
        "currently we can only write version 2"
    );
    let lower_bound = objects.size_hint().0;
    let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(
        chunk_size,
        if lower_bound == 0 { None } else { Some(lower_bound) },
        thread_limit,
        None,
    );
    let chunks = util::Chunks {
        iter: objects,
        size: chunk_size,
    };

    parallel::reduce::Stepwise::new(
        chunks,
        thread_limit,
        move |_n| {
            (
                Vec::new(),   // object locate buffer
                make_cache(), // cache to speed up pack operations
            )
        },
        move |oids: Vec<Oid>, (buf, cache)| {
            use ObjectExpansion::*;
            let mut out = Vec::new();
            type TraversalState = git_traverse::tree::breadthfirst::State<
                <tree::traverse::AllUnseen as git_traverse::tree::Visit>::PathId,
            >;
            let mut tree_traversal_state: Option<TraversalState> = None;
            for id in oids.into_iter() {
                let id = id.as_ref();
                let obj = db
                    .find(id, buf, cache)?
                    .ok_or_else(|| Error::NotFound { oid: id.to_owned() })?;
                match input_object_expansion {
                    TreeAdditionsComparedToAncestor => {
                        todo!("tree additions compared to ancestor")
                    }
                    TreeContents => {
                        use git_object::Kind::*;
                        let state = tree_traversal_state.get_or_insert_with(TraversalState::default);
                        let mut delegate = tree::traverse::AllUnseen::default();
                        let mut obj = obj;
                        loop {
                            out.push(obj_to_entry(&db, version, id, &obj)?);
                            match obj.kind {
                                Tree => {
                                    git_traverse::tree::breadthfirst(
                                        id,
                                        state,
                                        |oid, buf| {
                                            if oid == id {
                                                buf.resize(obj.data.len(), 0);
                                                buf.copy_from_slice(obj.data);
                                                Some(git_object::immutable::TreeIter::from_bytes(buf))
                                            } else {
                                                db.find_existing_tree_iter(oid, buf, cache).ok()
                                            }
                                        },
                                        &mut delegate,
                                    )
                                    .map_err(Error::TreeTraverse)?;
                                    for id in delegate.objects.into_iter() {
                                        let obj =
                                            db.find(id, buf, cache)?.ok_or_else(|| Error::NotFound { oid: id })?;
                                        out.push(obj_to_entry(&db, version, &id, &obj)?);
                                    }
                                    break;
                                }
                                Commit => {
                                    let tree_id = obj
                                        .into_commit_iter()
                                        .expect("kind is valid")
                                        .tree_id()
                                        .expect("every commit has a tree");
                                    obj = db.find_existing(tree_id, buf, cache).map_err(|_| Error::NotFound {
                                        oid: tree_id.to_owned(),
                                    })?;
                                    continue;
                                }
                                Blob | Tag => break,
                            }
                        }
                    }
                    AsIs => out.push(obj_to_entry(&db, version, id, &obj)?),
                }
            }
            Ok(out)
        },
        parallel::reduce::IdentityWithResult::default(),
    )
}

mod tree {
    pub mod traverse {
        use git_hash::{bstr::BStr, ObjectId};
        use git_object::immutable::tree::Entry;
        use git_traverse::tree::visit::{Action, Visit};
        use std::collections::HashSet;

        #[derive(Default)]
        pub struct AllUnseen {
            pub objects: HashSet<ObjectId>,
        }

        impl Visit for AllUnseen {
            type PathId = ();

            fn set_current_path(&mut self, _id: Self::PathId) {}

            fn push_tracked_path_component(&mut self, _component: &BStr) -> Self::PathId {}

            fn push_path_component(&mut self, _component: &BStr) {}

            fn pop_path_component(&mut self) {}

            fn visit_tree(&mut self, entry: &Entry<'_>) -> Action {
                self.objects.insert(entry.oid.to_owned());
                Action::Continue
            }

            fn visit_nontree(&mut self, entry: &Entry<'_>) -> Action {
                self.objects.insert(entry.oid.to_owned());
                Action::Continue
            }
        }
    }
}

fn obj_to_entry<Locate>(
    db: &Locate,
    version: pack::data::Version,
    id: &oid,
    obj: &crate::data::Object<'_>,
) -> Result<output::Entry, Error<Locate::Error>>
where
    Locate: crate::Find,
{
    Ok(match db.pack_entry(&obj) {
        Some(entry) if entry.version == version => {
            let pack_entry = pack::data::Entry::from_bytes(entry.data, 0);
            if let Some(expected) = entry.crc32 {
                let actual = hash::crc32(entry.data);
                if actual != expected {
                    return Err(Error::PackToPackCopyCrc32Mismatch { actual, expected });
                }
            }
            if pack_entry.header.is_base() {
                output::Entry {
                    id: id.to_owned(),
                    object_kind: pack_entry.header.to_kind().expect("non-delta"),
                    kind: output::entry::Kind::Base,
                    decompressed_size: obj.data.len(),
                    compressed_data: entry.data[pack_entry.data_offset as usize..].to_owned(),
                }
            } else {
                output::Entry::from_data(id, &obj).map_err(Error::NewEntry)?
            }
        }
        _ => output::Entry::from_data(id, &obj).map_err(Error::NewEntry)?,
    })
}

mod util {
    pub struct Chunks<I> {
        pub size: usize,
        pub iter: I,
    }

    impl<I, Item> Iterator for Chunks<I>
    where
        I: Iterator<Item = Item>,
    {
        type Item = Vec<Item>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut res = Vec::with_capacity(self.size);
            let mut items_left = self.size;
            while let Some(item) = self.iter.next() {
                res.push(item);
                items_left -= 1;
                if items_left == 0 {
                    break;
                }
            }
            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        }
    }
}

mod types {
    use crate::pack::data::output::entry;
    use git_hash::ObjectId;

    /// The way input objects are handled
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum ObjectExpansion {
        /// Don't do anything with the input objects except for transforming them into pack entries
        AsIs,
        /// If the input object is a Commit then turn it into a pack entry. Additionally obtain its tree, turn it into a pack entry
        /// along with all of its contents, that is nested trees, and any other objects reachable from it.
        /// Otherwise, the same as [`AsIs`][ObjectExpansion::AsIs].
        ///
        /// This mode is useful if all reachable objects should be added, as in cloning a repository.
        TreeContents,
        /// If the input is a commit, obtain its ancestors and turn them into pack entries. Obtain the ancestor trees along with the commits
        /// tree and turn them into pack entries. Finally obtain the added/changed objects when comparing the ancestor trees with the
        /// current tree and turn them into entries as well.
        /// Otherwise, the same as [`AsIs`][ObjectExpansion::AsIs].
        ///
        /// This mode is useful to build a pack containing only new objects compared to a previous state.
        TreeAdditionsComparedToAncestor,
    }

    impl Default for ObjectExpansion {
        fn default() -> Self {
            ObjectExpansion::AsIs
        }
    }

    /// Configuration options for the pack generation functions provied in [this module][crate::pack::data::output].
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Options {
        /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
        pub thread_limit: Option<usize>,
        /// The amount of objects per chunk or unit of work to be sent to threads for processing
        /// TODO: could this become the window size?
        pub chunk_size: usize,
        /// The pack data version to produce
        pub version: crate::pack::data::Version,
        /// The way input objects are handled
        pub input_object_expansion: ObjectExpansion,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                thread_limit: None,
                chunk_size: 10,
                version: Default::default(),
                input_object_expansion: Default::default(),
            }
        }
    }

    /// The error returned by the pack generation function [`to_entry_iter()`][crate::pack::data::output::objects_to_entries_iter()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error<LocateErr>
    where
        LocateErr: std::error::Error + 'static,
    {
        #[error(transparent)]
        Locate(#[from] LocateErr),
        #[error(transparent)]
        TreeTraverse(git_traverse::tree::breadthfirst::Error),
        #[error("Object id {oid} wasn't found in object database")]
        NotFound { oid: ObjectId },
        #[error("Entry expected to have hash {expected}, but it had {actual}")]
        PackToPackCopyCrc32Mismatch { actual: u32, expected: u32 },
        #[error(transparent)]
        NewEntry(entry::Error),
    }
}
pub use types::{Error, ObjectExpansion, Options};
