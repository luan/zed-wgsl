use zed_extension_api::{self as zed, Result};

struct WgslExtension;

impl zed::Extension for WgslExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // https://github.com/nolanderc/glasgow
        // TODO: add auto download support
        // TODO: add support for other language servers
        let wgsl_lsp_cmd = worktree.which("glasgow");
        let path = wgsl_lsp_cmd.ok_or_else(|| "glasgow must be in your path".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(WgslExtension);
