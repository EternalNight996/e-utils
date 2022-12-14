fn main() {
    #[cfg(feature="sys_info")]
    {
        extern crate cc;
        let mut builder = cc::Build::new();
        #[cfg(target_os = "windows")]
        builder.file("c/windows.c");
    
        #[cfg(any(target_os = "linux", target_os = "android", target_os = "androideabi"))]
        builder.file("c/linux.c");
        
        builder.compile("links")
    }
}
