use crate::mod_manager::{ModError, ModIndex};
use ql_core::{err, file_utils, info, pt, InstanceSelection, IoError};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

pub async fn delete_mods_w(
    ids: Vec<String>,
    instance_name: InstanceSelection,
) -> Result<Vec<String>, String> {
    if ids.is_empty() {
        return Ok(ids);
    }

    delete_mods(&ids, &instance_name)
        .await
        .map_err(|err| err.to_string())
        .map(|()| ids)
}

pub async fn delete_mods(
    ids: &[String],
    instance_name: &InstanceSelection,
) -> Result<(), ModError> {
    info!("Deleting mods:");
    let mut index = ModIndex::get(instance_name)?;

    let mods_dir = file_utils::get_dot_minecraft_dir(instance_name)?.join("mods");

    // let mut downloaded_mods = HashSet::new();

    for id in ids {
        pt!("Deleting mod {id}");
        delete_mod(&mut index, id, &mods_dir)?;
        // delete_item(id, None, &mut index, &mods_dir, &mut downloaded_mods)?;
    }

    let mut has_been_removed;
    let mut iteration = 0;
    loop {
        iteration += 1;
        has_been_removed = false;
        pt!("Iteration {iteration}");
        let mut removed_dependents_map = HashMap::new();

        for (mod_id, mod_info) in &index.mods {
            if !mod_info.manually_installed {
                let mut removed_dependents = HashSet::new();
                for dependent in &mod_info.dependents {
                    if !index.mods.contains_key(dependent) {
                        removed_dependents.insert(dependent.clone());
                    }
                }
                removed_dependents_map.insert(mod_id.clone(), removed_dependents);
            }
        }

        for (id, removed_dependents) in removed_dependents_map {
            if let Some(mod_info) = index.mods.get_mut(&id) {
                for dependent in removed_dependents {
                    has_been_removed = true;
                    mod_info.dependents.remove(&dependent);
                }
            } else {
                err!("Dependent {id} does not exist");
            }
        }

        let mut orphaned_mods = HashSet::new();

        for (mod_id, mod_info) in &index.mods {
            if !mod_info.manually_installed && mod_info.dependents.is_empty() {
                pt!("Deleting child {}", mod_info.name);
                orphaned_mods.insert(mod_id.clone());
            }
        }

        for orphan in orphaned_mods {
            has_been_removed = true;
            delete_mod(&mut index, &orphan, &mods_dir)?;
        }

        if !has_been_removed {
            break;
        }
    }

    index.save()?;
    info!("Finished deleting mods");
    Ok(())
}

fn delete_mod(index: &mut ModIndex, id: &String, mods_dir: &Path) -> Result<(), ModError> {
    if let Some(mod_info) = index.mods.remove(id) {
        for file in &mod_info.files {
            if mod_info.enabled {
                delete_file(mods_dir, &file.filename)?;
            } else {
                delete_file(mods_dir, &format!("{}.disabled", file.filename))?;
            }
        }
    } else {
        err!("Deleted mod does not exist");
    }
    Ok(())
}

fn delete_file(mods_dir: &Path, file: &str) -> Result<(), ModError> {
    let path = mods_dir.join(file);
    if let Err(err) = std::fs::remove_file(&path) {
        if let std::io::ErrorKind::NotFound = err.kind() {
            err!("File does not exist, skipping: {path:?}");
        } else {
            let err = IoError::Io {
                error: err,
                path: path.clone(),
            };
            Err(err)?;
        }
    }
    Ok(())
}