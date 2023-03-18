use rfd::FileDialog;
use std::fs;

impl super::Editor {
	pub(super) fn new_file(&mut self) {
		self.content.clear();
		self.opened_file = None;
		self.caret_pos = 0;

		self.update_lines();
	}

	pub(super) fn open_file(&mut self) {
		if let Some(path) = self.construct_file_dialog().pick_file() {
			// Why sequence converting PathBuf to string is so long?
			self.open_file_from_path(path.to_str().unwrap().to_string());
		}
	}

	pub(super) fn open_file_from_path(&mut self, path: String) {
		if let Ok(data) = fs::read_to_string(path.clone()) {
			self.opened_file = Some(path);
			self.content = data.replace("    ", "\t").chars().collect();
			self.caret_pos = 0;

			self.update_lines();
		}
	}
	
	pub(super) fn save_file(&mut self) {
		if let Some(path) = &self.opened_file {
			fs::write(path, self.content_as_text()).unwrap();
		}
		else {
			self.save_file_as();
		}
	}

	pub(super) fn save_file_as(&mut self) {
		if let Some(path) = self.construct_file_dialog().save_file() {
			// Why sequence converting PathBuf to string is so long?
			self.opened_file = Some(path.to_str().unwrap().to_string());
			self.save_file();
		}
	}

	fn construct_file_dialog(&self) -> FileDialog {
		let mut dialog = FileDialog::new();

		// Open dialog in current file's directory, if any file is opened
		if let Some(path) = &self.opened_file {
			let mut path_vec: Vec<_> = path.split("/").collect();
			path_vec.pop();
			
			dialog = dialog.set_directory(path_vec.join("/"));
		}

		dialog
	}
}
