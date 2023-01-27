pub struct CompileDescriptor {
    pub package: String,
    pub package_options: String,
    pub destination: String,
}

impl CompileDescriptor {
    pub fn new<T: Into<String>>(package: T, package_options: T, destination: T) -> Self {
        Self {
            package: package.into(),
            package_options: package_options.into(),
            destination: destination.into(),
        }
    }
}
