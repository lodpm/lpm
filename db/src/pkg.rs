use common::pkg::LodPkg;
use min_sqlite3_sys::prelude::*;

pub trait LodPkgCoreDbOps {
    fn insert(&self, db: &Database);
}

impl<'a> LodPkgCoreDbOps for LodPkg<'a> {
    fn insert(&self, db: &Database) {
        let meta = &self.meta_dir.as_ref().unwrap().meta;

        let statement = String::from(
            "
            INSERT INTO packages
                (name, description, maintainer, repository_id,
                homepage, depended_package_id, package_kind_id,
                installed_size, license, v_major, v_minor, v_patch,
                v_tag, v_readable)
            VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        );

        let mut sql = db
            .prepare(
                statement,
                None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>,
            )
            .unwrap();

        sql.bind_val(1, meta.name.clone());
        sql.bind_val(2, meta.description.clone());
        sql.bind_val(3, meta.maintainer.clone());
        sql.bind_val(4, 1_u32); // TODO

        if let Some(homepage) = &meta.homepage {
            sql.bind_val(5, homepage.clone());
        } else {
            sql.bind_val(5, SQLITE_NULL);
        }

        sql.bind_val(6, SQLITE_NULL); // TODO
        sql.bind_val(7, 1_i32); // TODO
        sql.bind_val(8, meta.installed_size as i64);

        if let Some(license) = &meta.license {
            sql.bind_val(9, license.clone());
        } else {
            sql.bind_val(9, SQLITE_NULL);
        }

        sql.bind_val(10, self.version.major);
        sql.bind_val(11, self.version.minor);
        sql.bind_val(12, self.version.patch);

        if let Some(vtag) = &self.version.tag {
            sql.bind_val(13, vtag.clone());
        } else {
            sql.bind_val(13, SQLITE_NULL);
        }

        sql.bind_val(14, self.version.readable_format.clone());

        let _status = sql.execute_prepared();
        sql.kill();
    }
}

pub fn insert_pkg_kinds(
    kinds: Vec<String>,
    db: &Database,
) -> Result<SqlitePrimaryResult, MinSqliteWrapperError> {
    db.execute(
        String::from("BEGIN TRANSACTION;"),
        Some(super::simple_error_callback),
    )?;

    for kind in kinds {
        let statement = String::from(
            "
            INSERT INTO package_kinds
                (kind)
            VALUES
                (?);",
        );

        let mut sql = db.prepare(statement, Some(super::simple_error_callback))?;

        sql.bind_val(1, kind);

        sql.execute_prepared();
    }

    db.execute(String::from("COMMIT;"), Some(super::simple_error_callback))
}

pub fn delete_pkg_kinds(
    kinds: Vec<String>,
    db: &Database,
) -> Result<SqlitePrimaryResult, MinSqliteWrapperError> {
    db.execute(
        String::from("BEGIN TRANSACTION;"),
        Some(super::simple_error_callback),
    )?;

    for kind in kinds {
        let statement = String::from(
            "
            DELETE FROM package_kinds
            WHERE
                kind = ?;",
        );

        let mut sql = db.prepare(statement, Some(super::simple_error_callback))?;
        sql.bind_val(1, kind);

        sql.execute_prepared();
    }

    db.execute(String::from("COMMIT;"), Some(super::simple_error_callback))
}
