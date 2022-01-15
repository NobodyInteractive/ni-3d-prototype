

mod pref_impl_v1{

    use std::collections::HashMap;
    use std::fmt;
    use std::any::Any;
        pub struct AppPrefs(HashMap<String, Box<dyn Any>>);
        
        impl AppPrefs 
        {
            pub fn new() -> Self
            {
                Self ( HashMap::new())
                
            }
        
            pub fn set<T>(&mut self, key:String, value: T)
            where T: Any
            {
                self.0.insert(key, Box::new(value));
            }
        
            pub fn get<T>(&self, key:String) -> Option<&T>
            where T: Any
            {
                self.0.get(&key).and_then(|value| value.downcast_ref::<T>())
            }
        }
        
        // impl fmt::Display for AppPrefs 
        // {
        //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        //     {
        //         f.write_fmt(format_args!("{}", self))
        //     }
        // }
        
        impl fmt::Debug for AppPrefs
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
            {
                f
                    .debug_map()
                    .entries(
                        self.0.iter().map(
                            |(k, v)| (k, v)))
                    .finish()
            }

            
        }

}

mod pref_impl_v2
{
    use std::collections::HashMap;
    use std::fmt;

    #[derive(Debug)]
    pub enum AppPrefs
    {
        Id(u32),
        Path(&'static str),
    }
}

#[cfg(test)]
mod test 
{
    #[test]
    fn pref_impl_v1_test()
    {
        use crate::prelude::pref_impl_v1;
        let mut prefs = pref_impl_v1::AppPrefs::new();
        prefs.set(String::from("string"), "test");
        prefs.set(String::from("int"), 1);
        println!("{:?}", prefs);
        assert_eq!(
            format!("{:?}", prefs),
            "{\"string\": \"test\",\"int\": 1}"
        );
    }

    #[test]
    fn pref_impl_v2_test()
    {
        use crate::prelude::pref_impl_v2;   
        // let mut prefs = pref_impl_v2::AppPrefs{
        //     Id: 1,
        //     Path: "/usr/root",
        // };
    }
}