use utils::{ehandle::RuntimeError, file_io::copy_recursively};

use crate::{pkg::LodPkg, ExtractionTasks, ValidationTasks};

impl<'a> super::InstallationTasks for LodPkg<'a> {
    fn start_installation(&mut self) -> Result<(), RuntimeError> {
        self.start_extraction()?;
        self.start_validations()?;
        self.install_program()?;
        self.cleanup()?;

        Ok(())
    }

    fn install_program(&self) -> Result<(), RuntimeError> {
        let src = super::EXTRACTION_OUTPUT_PATH.to_string()
            + "/"
            + self.path.file_stem().unwrap().to_str().unwrap()
            + "/program/";

        copy_recursively(&src, "/")?;

        Ok(())
    }
}

